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

/// Fetches an issue's visible history entries (comments and changes).
///
/// Hidden history entries (where `is_hidden` is `true`) are excluded from the result.
///
/// # Parameters
///
/// - `issue_id`: ID of the issue to fetch history for.
///
/// # Returns
///
/// A `Vec<HistoryEntry>` containing the issue's visible history entries.
///
/// # Examples
///
/// ```
/// # async fn example(client: tauri::State<'_, TaigaClient>) -> anyhow::Result<()> {
/// let entries = get_issue_history(client, 42).await?;
/// assert!(entries.iter().all(|e| !e.is_hidden.unwrap_or(false)));
/// # Ok(())
/// # }
/// ```
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

/// Updates an issue's status using optimistic locking via the provided `version`.
///
/// If the provided `version` does not match the server's current version, the Taiga API may respond with a
/// `VersionConflict` (HTTP 412) error.
///
/// # Examples
///
/// ```no_run
/// # async fn example(client: tauri::State<'_, TaigaClient>) -> anyhow::Result<()> {
/// let issue_id = 123;
/// let new_status_id = 5;
/// let current_version = 42;
/// let updated = change_issue_status(client, issue_id, new_status_id, current_version).await?;
/// // `updated` is the refreshed IssueDetail after the status change.
/// # Ok(())
/// # }
/// ```
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
    };

    // Call the API - will return VersionConflict on 412
    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;

    // Convert to domain model
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}