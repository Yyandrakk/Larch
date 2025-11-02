use crate::domain::issue::Issue;
use crate::domain::project::Project;
use crate::error::Result;
use crate::services::credentials;
use taiga_client::TaigaClient;

#[tauri::command]
pub async fn get_projects(client: tauri::State<'_, TaigaClient>) -> Result<Vec<Project>> {
    let token = credentials::get_api_token()?;
    let projects_dto = client.get_projects(&token).await?;
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
