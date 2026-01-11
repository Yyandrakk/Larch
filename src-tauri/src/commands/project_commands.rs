use crate::domain::issue::Issue;
use crate::domain::project::Project;
use crate::error::Result;
use crate::repositories::Repository;
use crate::services::credentials;
use taiga_client::TaigaClient;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FilterObject {
    pub status_ids: Option<Vec<i64>>,
    pub status_exclude: Option<bool>,
    pub assignee_ids: Option<Vec<i64>>,
    pub assignee_exclude: Option<bool>,
    pub project_ids: Option<Vec<i64>>,
    pub project_exclude: Option<bool>,
}

#[tauri::command]
pub async fn get_projects(client: tauri::State<'_, TaigaClient>) -> Result<Vec<Project>> {
    let token = credentials::get_api_token()?;
    // Fetch current user to filter projects
    let me = client.get_me(&token).await?;
    let projects_dto = client.get_projects(&token, Some(me.id)).await?;
    let projects = projects_dto.into_iter().map(|p| p.into()).collect();
    Ok(projects)
}

#[tauri::command]
pub async fn list_issues(
    client: tauri::State<'_, TaigaClient>,
    project_id: i64,
) -> Result<Vec<Issue>> {
    let token = credentials::get_api_token()?;
    let issues_dto = client.list_issues(&token, project_id, None).await?;
    let issues = issues_dto.into_iter().map(|i| i.into()).collect();
    Ok(issues)
}

#[tauri::command]
pub async fn get_aggregated_issues(
    client: tauri::State<'_, TaigaClient>,
    repo: tauri::State<'_, crate::repositories::SqliteRepository>,
    filters: FilterObject,
) -> Result<Vec<Issue>> {
    let token = credentials::get_api_token()?;

    // 1. Get selected projects from DB
    let selected_ids_opt = repo.get_config("selected_projects").await?;
    let mut target_project_ids: Vec<i64> = if let Some(val) = selected_ids_opt {
        serde_json::from_str(&val).unwrap_or_default()
    } else {
        vec![]
    };

    // 2. Apply UI project filter if present
    if let Some(ref ui_project_ids) = filters.project_ids {
        if filters.project_exclude.unwrap_or(false) {
            // Exclude these projects - keep ones NOT in ui_project_ids
            target_project_ids.retain(|id| !ui_project_ids.contains(id));
        } else {
            // Include only these projects - keep ones IN ui_project_ids
            target_project_ids.retain(|id| ui_project_ids.contains(id));
        }
    }

    if target_project_ids.is_empty() {
        return Ok(vec![]);
    }

    // 3. Construct Taiga filters
    let mut query_params = Vec::new();
    if let Some(status_ids) = filters.status_ids {
        if !status_ids.is_empty() {
            let key = if filters.status_exclude.unwrap_or(false) {
                "exclude_status"
            } else {
                "status"
            };
            let val = status_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query_params.push((key.to_string(), val));
        }
    }

    if let Some(assignee_ids) = filters.assignee_ids {
        if !assignee_ids.is_empty() {
            let key = if filters.assignee_exclude.unwrap_or(false) {
                "exclude_assigned_to"
            } else {
                "assigned_to"
            };
            let val = assignee_ids
                .iter()
                .map(|id| {
                    if *id == -1 {
                        "null".to_string()
                    } else {
                        id.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(",");
            query_params.push((key.to_string(), val));
        }
    }

    // 4. Fetch concurrently
    let mut tasks = Vec::new();
    for pid in target_project_ids {
        let client = client.inner().clone();
        let token = token.clone();
        let params = query_params.clone();
        tasks.push(tauri::async_runtime::spawn(async move {
            client.list_issues(&token, pid, Some(params)).await
        }));
    }

    let mut all_issues = Vec::new();
    for task in tasks {
        match task.await {
            Ok(Ok(issues)) => all_issues.extend(issues),
            Ok(Err(e)) => log::error!("Failed to fetch issues: {}", e), // Partial failure strategy
            Err(e) => log::error!("Task join error: {}", e),
        }
    }

    Ok(all_issues.into_iter().map(|i| i.into()).collect())
}

#[tauri::command]
pub async fn get_project_metadata(
    client: tauri::State<'_, TaigaClient>,
    project_ids: Vec<i64>,
) -> Result<std::collections::HashMap<i64, crate::domain::project::ProjectMetadata>> {
    use crate::domain::project::{IssueStatus, IssueType, Member, Priority, Severity, TagColor};

    let token = credentials::get_api_token()?;
    let mut tasks = Vec::new();

    for pid in project_ids {
        let client = client.inner().clone();
        let token = token.clone();
        tasks.push(tauri::async_runtime::spawn(async move {
            let project_res = client.get_project(&token, pid).await;

            let priorities_res = match client.get_priorities(&token, pid).await {
                Ok(p) => Ok(p),
                Err(e) => {
                    log::error!("get_priorities failed for {}: {}", pid, e);
                    Err(e)
                }
            };

            let severities_res = match client.get_severities(&token, pid).await {
                Ok(s) => Ok(s),
                Err(e) => {
                    log::error!("get_severities failed for {}: {}", pid, e);
                    Err(e)
                }
            };

            let types_res = match client.get_issue_types(&token, pid).await {
                Ok(t) => Ok(t),
                Err(e) => {
                    log::error!("get_issue_types failed for {}: {}", pid, e);
                    Err(e)
                }
            };

            let tags_res = match client.get_project_tags_colors(&token, pid).await {
                Ok(t) => Ok(t),
                Err(e) => {
                    log::error!("get_project_tags_colors failed for {}: {}", pid, e);
                    Err(e)
                }
            };

            let members_res = match client.get_memberships(&token, pid).await {
                Ok(m) => Ok(m),
                Err(e) => {
                    log::error!("get_memberships failed for {}: {}", pid, e);
                    Err(e)
                }
            };

            let dto = match project_res {
                Ok(p) => p,
                Err(e) => {
                    log::error!("Failed to fetch project {} metadata: {}", pid, e);
                    return None;
                }
            };

            let statuses: Vec<IssueStatus> = dto
                .issue_statuses
                .unwrap_or_default()
                .into_iter()
                .map(|s| IssueStatus {
                    id: s.id,
                    name: s.name,
                    color: s.color,
                    is_closed: s.is_closed,
                })
                .collect();

            let members: Vec<Member> = members_res
                .unwrap_or_else(|e| {
                    log::warn!("Failed to fetch memberships for {}: {}", pid, e);
                    vec![]
                })
                .into_iter()
                .map(|m| Member {
                    id: m.id,
                    user_id: m.user,
                    full_name: m.full_name.unwrap_or_default(),
                    role_name: m.role_name,
                    photo: m.photo,
                })
                .collect();

            let priorities: Vec<Priority> = priorities_res
                .unwrap_or_else(|e| {
                    log::warn!("Failed to fetch priorities for {}: {}", pid, e);
                    vec![]
                })
                .into_iter()
                .map(|p| Priority {
                    id: p.id,
                    name: p.name,
                    color: p.color,
                    order: p.order,
                })
                .collect();

            let severities: Vec<Severity> = severities_res
                .unwrap_or_else(|e| {
                    log::warn!("Failed to fetch severities for {}: {}", pid, e);
                    vec![]
                })
                .into_iter()
                .map(|s| Severity {
                    id: s.id,
                    name: s.name,
                    color: s.color,
                    order: s.order,
                })
                .collect();

            let issue_types: Vec<IssueType> = types_res
                .unwrap_or_else(|e| {
                    log::warn!("Failed to fetch issue types for {}: {}", pid, e);
                    vec![]
                })
                .into_iter()
                .map(|t| IssueType {
                    id: t.id,
                    name: t.name,
                    color: t.color,
                    order: t.order,
                })
                .collect();

            let tags_colors: Vec<TagColor> = tags_res
                .unwrap_or_else(|e| {
                    log::warn!("Failed to fetch tags colors for {}: {}", pid, e);
                    serde_json::Value::Null
                })
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(name, color)| TagColor {
                            name: name.clone(),
                            color: color.as_str().map(|s| s.to_string()),
                        })
                        .collect()
                })
                .unwrap_or_default();

            Some(crate::domain::project::ProjectMetadata {
                id: pid,
                statuses,
                members,
                priorities,
                severities,
                issue_types,
                tags_colors,
            })
        }));
    }

    let mut result = std::collections::HashMap::new();
    for task in tasks {
        if let Ok(Some(meta)) = task.await {
            result.insert(meta.id, meta);
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_selected_projects(
    repo: tauri::State<'_, crate::repositories::SqliteRepository>,
) -> Result<Vec<i64>> {
    let value_opt = repo.get_config("selected_projects").await?;
    if let Some(value) = value_opt {
        let ids: Vec<i64> = serde_json::from_str(&value).unwrap_or_default();
        Ok(ids)
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
pub async fn save_selected_projects(
    repo: tauri::State<'_, crate::repositories::SqliteRepository>,
    project_ids: Vec<i64>,
) -> Result<()> {
    let value = serde_json::to_string(&project_ids).unwrap_or_default();
    repo.save_config("selected_projects", &value).await?;
    Ok(())
}
