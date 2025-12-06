use crate::domain::issue::Issue;
use crate::domain::project::Project;
use crate::error::Result;
use crate::repositories::Repository;
use crate::services::credentials;
use taiga_client::TaigaClient;

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
    let issues_dto = client.list_issues(&token, project_id).await?;
    let issues = issues_dto.into_iter().map(|i| i.into()).collect();
    Ok(issues)
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
