use crate::domain::issue_detail::{HistoryEntry, IssueDetail};
use crate::error::Result;
use crate::repositories::{Repository, SqliteRepository};
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
        description: None,
        assigned_to: None,
        priority: None,
        severity: None,
        type_: None,
        tags: None,
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
        description: None,
        assigned_to: None,
        priority: None,
        severity: None,
        type_: None,
        tags: None,
    };

    // Call the API - will return VersionConflict on 412
    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;

    // Convert to domain model
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}

/// Commit a description change from local draft to Taiga API
/// Reads the description draft from SQLite and commits to API
/// On success, deletes the local draft
#[tauri::command]
pub async fn commit_issue_description(
    client: tauri::State<'_, TaigaClient>,
    repository: tauri::State<'_, SqliteRepository>,
    issue_id: i64,
    version: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;

    // Read the draft from the local database
    let related_id = format!("issue_{}", issue_id);
    let draft_type = "description";

    let description = repository
        .get_draft(&related_id, draft_type)
        .await?
        .ok_or_else(|| crate::error::Error::Database("No draft found to commit".to_string()))?;

    log::info!(
        "Committing description draft for issue {} (version {})",
        issue_id,
        version
    );

    // Build the patch request with the description
    let request = taiga_client::models::PatchIssueRequest {
        version,
        status: None,
        comment: None,
        description: Some(description),
        assigned_to: None,
        priority: None,
        severity: None,
        type_: None,
        tags: None,
    };

    // Call the API - will return VersionConflict on 412
    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;

    // On success, delete the local draft
    repository.delete_draft(&related_id, draft_type).await?;

    log::info!("Successfully committed description for issue {}", issue_id);

    // Convert to domain model
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}

/// Change the assignee of an issue
/// Uses optimistic locking via the version field
/// Pass `None` for assignee_id to unassign
#[tauri::command]
pub async fn change_issue_assignee(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    assignee_id: Option<i64>,
    version: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;

    // Build the patch request with the new assignee
    let request = taiga_client::models::PatchIssueRequest {
        version,
        status: None,
        comment: None,
        description: None,
        assigned_to: Some(assignee_id),
        priority: None,
        severity: None,
        type_: None,
        tags: None,
    };

    // Call the API - will return VersionConflict on 412
    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;

    // Convert to domain model
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}

#[tauri::command]
pub async fn change_issue_priority(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    priority_id: i64,
    version: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;

    let request = taiga_client::models::PatchIssueRequest {
        version,
        status: None,
        comment: None,
        description: None,
        assigned_to: None,
        priority: Some(priority_id),
        severity: None,
        type_: None,
        tags: None,
    };

    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}

#[tauri::command]
pub async fn change_issue_severity(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    severity_id: i64,
    version: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;

    let request = taiga_client::models::PatchIssueRequest {
        version,
        status: None,
        comment: None,
        description: None,
        assigned_to: None,
        priority: None,
        severity: Some(severity_id),
        type_: None,
        tags: None,
    };

    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}

#[tauri::command]
pub async fn change_issue_type(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    type_id: i64,
    version: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;

    let request = taiga_client::models::PatchIssueRequest {
        version,
        status: None,
        comment: None,
        description: None,
        assigned_to: None,
        priority: None,
        severity: None,
        type_: Some(type_id),
        tags: None,
    };

    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}

#[tauri::command]
pub async fn update_issue_tags(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    tags: Vec<(String, Option<String>)>,
    version: i64,
) -> Result<IssueDetail> {
    let token = credentials::get_api_token()?;

    let tags_json: Vec<serde_json::Value> = tags
        .into_iter()
        .map(|(name, color)| serde_json::json!([name, color]))
        .collect();

    let request = taiga_client::models::PatchIssueRequest {
        version,
        status: None,
        comment: None,
        description: None,
        assigned_to: None,
        priority: None,
        severity: None,
        type_: None,
        tags: Some(serde_json::Value::Array(tags_json)),
    };

    let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;
    let issue_detail = IssueDetail::from_dto(updated_issue_dto);

    Ok(issue_detail)
}

#[tauri::command]
pub async fn upload_issue_attachment(
    client: tauri::State<'_, TaigaClient>,
    project_id: i64,
    issue_id: i64,
    file_name: String,
    file_data: Vec<u8>,
) -> Result<crate::domain::issue_detail::Attachment> {
    let token = credentials::get_api_token()?;

    let attachment_dto = client
        .upload_issue_attachment(&token, project_id, issue_id, file_name, file_data)
        .await?;

    Ok((&attachment_dto).into())
}

#[tauri::command]
pub async fn delete_issue_attachment(
    client: tauri::State<'_, TaigaClient>,
    attachment_id: i64,
) -> Result<()> {
    let token = credentials::get_api_token()?;

    client
        .delete_issue_attachment(&token, attachment_id)
        .await?;

    Ok(())
}
