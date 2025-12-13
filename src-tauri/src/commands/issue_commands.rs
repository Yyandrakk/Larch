use crate::domain::issue_detail::{HistoryEntry, IssueDetail};
use crate::error::Result;
use crate::services::credentials;
use taiga_client::TaigaClient;

/// Get detailed issue information by ID
/// This command fetches the full issue details from Taiga API
#[tauri::command]
pub async fn get_issue_detail(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;

    // Fetch issue detail from Taiga API
    let issue_dto = client.get_issue(&token, issue_id).await?;

    // Convert to domain model
    let issue_detail = IssueDetail::from_dto(issue_dto);

    // TODO: In the future, we could resolve type/severity/priority names here
    // by fetching project metadata and looking up the IDs
    // For now, the frontend can display "Type #X" if no name is available

    Ok(issue_detail)
}

/// Get issue history (comments and changes)
#[tauri::command]
pub async fn get_issue_history(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
) -> Result<Vec<HistoryEntry>> {
    let token = credentials::get_api_token()?;

    // Fetch history from Taiga API
    let history_dto = client.get_issue_history(&token, issue_id).await?;

    // Convert to domain models, filtering out hidden entries
    let entries: Vec<HistoryEntry> = history_dto
        .iter()
        .filter(|h| !h.is_hidden.unwrap_or(false))
        .map(|h| h.into())
        .collect();

    Ok(entries)
}
