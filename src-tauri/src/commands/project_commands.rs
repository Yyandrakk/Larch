use crate::domain::issue::Issue;
use crate::domain::project::Project;
use crate::error::Result;
use crate::repositories::Repository;
use crate::services::{credentials, token_refresh};
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
    async fn fetch(client: &TaigaClient) -> Result<Vec<Project>> {
        let token = credentials::get_api_token()?;
        let me = client.get_me(&token).await?;
        let projects_dto = client.get_projects(&token, Some(me.id)).await?;
        Ok(projects_dto.into_iter().map(|p| p.into()).collect())
    }

    match fetch(&client).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client).await
        }
        result => result,
    }
}

#[tauri::command]
pub async fn list_issues(
    client: tauri::State<'_, TaigaClient>,
    project_id: i64,
) -> Result<Vec<Issue>> {
    async fn fetch(client: &TaigaClient, project_id: i64) -> Result<Vec<Issue>> {
        let token = credentials::get_api_token()?;
        let issues_dto = client.list_issues(&token, project_id, None).await?;
        Ok(issues_dto.into_iter().map(|i| i.into()).collect())
    }

    match fetch(&client, project_id).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, project_id).await
        }
        result => result,
    }
}

#[tauri::command]
pub async fn get_aggregated_issues(
    client: tauri::State<'_, TaigaClient>,
    repo: tauri::State<'_, crate::repositories::SqliteRepository>,
    filters: FilterObject,
) -> Result<Vec<Issue>> {
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

    // 4. Fetch concurrently with token refresh support
    async fn fetch_all_issues(
        client: &TaigaClient,
        project_ids: &[i64],
        query_params: &[(String, String)],
    ) -> (Vec<taiga_client::models::IssueDto>, bool) {
        let token = match credentials::get_api_token() {
            Ok(t) => t,
            Err(_) => return (vec![], false),
        };

        let mut tasks = Vec::new();
        for pid in project_ids {
            let client = client.clone();
            let token = token.clone();
            let params: Vec<(String, String)> = query_params.to_vec();
            let pid = *pid;
            tasks.push(tauri::async_runtime::spawn(async move {
                client.list_issues(&token, pid, Some(params)).await
            }));
        }

        let mut all_issues = Vec::new();
        let mut had_unauthorized = false;
        for task in tasks {
            match task.await {
                Ok(Ok(issues)) => all_issues.extend(issues),
                Ok(Err(taiga_client::errors::TaigaClientError::Unauthorized(_))) => {
                    had_unauthorized = true;
                }
                Ok(Err(e)) => log::error!("Failed to fetch issues: {}", e),
                Err(e) => log::error!("Task join error: {}", e),
            }
        }

        (all_issues, had_unauthorized)
    }

    let (mut all_issues, had_unauthorized) =
        fetch_all_issues(&client, &target_project_ids, &query_params).await;

    if had_unauthorized {
        log::info!("Unauthorized detected in batch fetch, attempting token refresh");
        token_refresh::refresh_token(&client).await?;
        let (retried_issues, _) =
            fetch_all_issues(&client, &target_project_ids, &query_params).await;
        all_issues = retried_issues;
    }

    // Sort globally by modified_date (newest first) AFTER aggregation
    all_issues.sort_by(|a, b| {
        match (&a.modified_date, &b.modified_date) {
            (Some(date_a), Some(date_b)) => date_b.cmp(date_a), // Descending (newest first)
            (Some(_), None) => std::cmp::Ordering::Less,        // Issues with dates come first
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });

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

#[cfg(test)]
mod tests {
    use taiga_client::models::{IssueDto, IssueStatusExtraInfo};

    fn create_issue_dto(id: i64, modified_date: Option<&str>) -> IssueDto {
        IssueDto {
            id,
            subject: format!("Issue {}", id),
            project: 1,
            status: 1,
            status_extra_info: Some(IssueStatusExtraInfo {
                name: "Open".to_string(),
                color: "#000".to_string(),
                is_closed: false,
            }),
            owner: None,
            assigned_to: None,
            assigned_to_extra_info: None,
            modified_date: modified_date.map(String::from),
            priority: None,
            severity: None,
            type_: None,
        }
    }

    #[test]
    fn test_global_sorting_by_modified_date_descending() {
        let mut issues = vec![
            create_issue_dto(1, Some("2024-01-01T10:00:00Z")),
            create_issue_dto(2, Some("2024-01-03T10:00:00Z")),
            create_issue_dto(3, Some("2024-01-02T10:00:00Z")),
        ];

        issues.sort_by(|a, b| match (&a.modified_date, &b.modified_date) {
            (Some(date_a), Some(date_b)) => date_b.cmp(date_a),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });

        assert_eq!(issues[0].id, 2); // 2024-01-03 (newest)
        assert_eq!(issues[1].id, 3); // 2024-01-02
        assert_eq!(issues[2].id, 1); // 2024-01-01 (oldest)
    }

    #[test]
    fn test_issues_with_null_dates_sorted_to_end() {
        let mut issues = vec![
            create_issue_dto(1, None),
            create_issue_dto(2, Some("2024-01-02T10:00:00Z")),
            create_issue_dto(3, None),
            create_issue_dto(4, Some("2024-01-01T10:00:00Z")),
        ];

        issues.sort_by(|a, b| match (&a.modified_date, &b.modified_date) {
            (Some(date_a), Some(date_b)) => date_b.cmp(date_a),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });

        assert_eq!(issues[0].id, 2); // 2024-01-02 (newest with date)
        assert_eq!(issues[1].id, 4); // 2024-01-01
                                     // Issues 1 and 3 (no date) should be at the end
        assert!(issues[2].modified_date.is_none());
        assert!(issues[3].modified_date.is_none());
    }

    #[test]
    fn test_interleaved_project_issues_sorted_globally() {
        let mut issues = vec![
            create_issue_dto(1, Some("2024-01-05T10:00:00Z")), // Project A - oldest
            create_issue_dto(2, Some("2024-01-01T10:00:00Z")), // Project A
            create_issue_dto(3, Some("2024-01-03T10:00:00Z")), // Project B - middle
            create_issue_dto(4, Some("2024-01-07T10:00:00Z")), // Project B - newest
        ];

        issues.sort_by(|a, b| match (&a.modified_date, &b.modified_date) {
            (Some(date_a), Some(date_b)) => date_b.cmp(date_a),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });

        assert_eq!(issues[0].id, 4); // 2024-01-07 (newest)
        assert_eq!(issues[1].id, 1); // 2024-01-05
        assert_eq!(issues[2].id, 3); // 2024-01-03
        assert_eq!(issues[3].id, 2); // 2024-01-01 (oldest)
    }
}
