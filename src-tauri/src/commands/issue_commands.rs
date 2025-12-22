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

/// Change the status of an issue
/// Uses optimistic locking via the version field
#[tauri::command]
pub async fn change_issue_status(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    status_id: i64,
    version: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;

    // Build the patch request
    let request = taiga_client::models::PatchIssueRequest {
        version,
        status: Some(status_id),
        comment: None,
    };

    // Call the API - will return VersionConflict on 412
    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;

    // Convert to domain model
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}

/// Add a comment to an issue
/// Uses optimistic locking via the version field
#[tauri::command]
pub async fn add_issue_comment(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    comment: String,
    version: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;

    // Build the patch request with only the comment
    let request = taiga_client::models::PatchIssueRequest {
        version,
        status: None,
        comment: Some(comment),
    };

    // Call the API - will return VersionConflict on 412
    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;

    // Convert to domain model
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}
