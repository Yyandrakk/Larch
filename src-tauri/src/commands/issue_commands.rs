use crate::domain::issue_detail::{HistoryEntry, IssueDetail};
use crate::error::Result;
use crate::repositories::{Repository, SqliteRepository};
use crate::services::{credentials, token_refresh};
use taiga_client::TaigaClient;

/// Get detailed issue information by ID
/// This command fetches the full issue details from Taiga API
#[tauri::command]
pub async fn get_issue_detail(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
) -> Result<IssueDetail> {
    async fn fetch(client: &TaigaClient, issue_id: i64) -> Result<IssueDetail> {
        let token = credentials::get_api_token()?;
        let issue_dto = client.get_issue(&token, issue_id).await?;
        Ok(IssueDetail::from_dto(issue_dto))
    }

    match fetch(&client, issue_id).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id).await
        }
        result => result,
    }
}

/// Get issue history (comments and changes)
#[tauri::command]
pub async fn get_issue_history(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
) -> Result<Vec<HistoryEntry>> {
    async fn fetch(client: &TaigaClient, issue_id: i64) -> Result<Vec<HistoryEntry>> {
        let token = credentials::get_api_token()?;
        let history_dto = client.get_issue_history(&token, issue_id).await?;
        let entries: Vec<HistoryEntry> = history_dto
            .iter()
            .filter(|h| !h.is_hidden.unwrap_or(false))
            .map(|h| h.into())
            .collect();
        Ok(entries)
    }

    match fetch(&client, issue_id).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id).await
        }
        result => result,
    }
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
    async fn fetch(
        client: &TaigaClient,
        issue_id: i64,
        status_id: i64,
        version: i64,
    ) -> Result<IssueDetail> {
        let token = credentials::get_api_token()?;
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
        let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;
        Ok(IssueDetail::from_dto(updated_issue_dto))
    }

    match fetch(&client, issue_id, status_id, version).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id, status_id, version).await
        }
        result => result,
    }
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
    async fn fetch(
        client: &TaigaClient,
        issue_id: i64,
        comment: &str,
        version: i64,
    ) -> Result<IssueDetail> {
        let token = credentials::get_api_token()?;
        let request = taiga_client::models::PatchIssueRequest {
            version,
            status: None,
            comment: Some(comment.to_string()),
            description: None,
            assigned_to: None,
            priority: None,
            severity: None,
            type_: None,
            tags: None,
        };
        let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;
        Ok(IssueDetail::from_dto(updated_issue_dto))
    }

    match fetch(&client, issue_id, &comment, version).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id, &comment, version).await
        }
        result => result,
    }
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

    async fn fetch(
        client: &TaigaClient,
        issue_id: i64,
        description: &str,
        version: i64,
    ) -> Result<IssueDetail> {
        let token = credentials::get_api_token()?;
        let request = taiga_client::models::PatchIssueRequest {
            version,
            status: None,
            comment: None,
            description: Some(description.to_string()),
            assigned_to: None,
            priority: None,
            severity: None,
            type_: None,
            tags: None,
        };
        let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;
        Ok(IssueDetail::from_dto(updated_issue_dto))
    }

    let result = match fetch(&client, issue_id, &description, version).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id, &description, version).await
        }
        result => result,
    }?;

    // On success, delete the local draft
    repository.delete_draft(&related_id, draft_type).await?;

    log::info!("Successfully committed description for issue {}", issue_id);

    Ok(result)
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
    async fn fetch(
        client: &TaigaClient,
        issue_id: i64,
        assignee_id: Option<i64>,
        version: i64,
    ) -> Result<IssueDetail> {
        let token = credentials::get_api_token()?;
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
        let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;
        Ok(IssueDetail::from_dto(updated_issue_dto))
    }

    match fetch(&client, issue_id, assignee_id, version).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id, assignee_id, version).await
        }
        result => result,
    }
}

#[tauri::command]
pub async fn change_issue_priority(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    priority_id: i64,
    version: i64,
) -> Result<IssueDetail> {
    async fn fetch(
        client: &TaigaClient,
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
        Ok(IssueDetail::from_dto(updated_issue_dto))
    }

    match fetch(&client, issue_id, priority_id, version).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id, priority_id, version).await
        }
        result => result,
    }
}

#[tauri::command]
pub async fn change_issue_severity(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    severity_id: i64,
    version: i64,
) -> Result<IssueDetail> {
    async fn fetch(
        client: &TaigaClient,
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
        Ok(IssueDetail::from_dto(updated_issue_dto))
    }

    match fetch(&client, issue_id, severity_id, version).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id, severity_id, version).await
        }
        result => result,
    }
}

#[tauri::command]
pub async fn change_issue_type(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    type_id: i64,
    version: i64,
) -> Result<IssueDetail> {
    async fn fetch(
        client: &TaigaClient,
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
        Ok(IssueDetail::from_dto(updated_issue_dto))
    }

    match fetch(&client, issue_id, type_id, version).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id, type_id, version).await
        }
        result => result,
    }
}

#[tauri::command]
pub async fn update_issue_tags(
    client: tauri::State<'_, TaigaClient>,
    issue_id: i64,
    tags: Vec<(String, Option<String>)>,
    version: i64,
) -> Result<IssueDetail> {
    let tags_json: Vec<serde_json::Value> = tags
        .iter()
        .map(|(name, color)| serde_json::json!([name, color]))
        .collect();

    async fn fetch(
        client: &TaigaClient,
        issue_id: i64,
        tags_json: Vec<serde_json::Value>,
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
            type_: None,
            tags: Some(serde_json::Value::Array(tags_json)),
        };
        let updated_issue_dto = client.patch_issue(&token, issue_id, request).await?;
        Ok(IssueDetail::from_dto(updated_issue_dto))
    }

    match fetch(&client, issue_id, tags_json.clone(), version).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, issue_id, tags_json, version).await
        }
        result => result,
    }
}

#[tauri::command]
pub async fn upload_issue_attachment(
    client: tauri::State<'_, TaigaClient>,
    project_id: i64,
    issue_id: i64,
    file_name: String,
    mime_type: Option<String>,
    file_data: Vec<u8>,
) -> Result<crate::domain::issue_detail::Attachment> {
    log::debug!(
        "Command upload_issue_attachment: project={}, issue={}, type={:?}, size={} bytes",
        project_id,
        issue_id,
        mime_type,
        file_data.len()
    );

    async fn fetch(
        client: &TaigaClient,
        project_id: i64,
        issue_id: i64,
        file_name: &str,
        mime_type: Option<String>,
        file_data: &[u8],
    ) -> Result<crate::domain::issue_detail::Attachment> {
        let token = credentials::get_api_token()?;
        let attachment_dto = client
            .upload_issue_attachment(
                &token,
                project_id,
                issue_id,
                file_name.to_string(),
                mime_type,
                file_data.to_vec(),
            )
            .await?;
        Ok((&attachment_dto).into())
    }

    match fetch(
        &client,
        project_id,
        issue_id,
        &file_name,
        mime_type.clone(),
        &file_data,
    )
    .await
    {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(
                &client, project_id, issue_id, &file_name, mime_type, &file_data,
            )
            .await
        }
        result => result,
    }
}

#[tauri::command]
pub async fn delete_issue_attachment(
    client: tauri::State<'_, TaigaClient>,
    attachment_id: i64,
) -> Result<()> {
    async fn fetch(client: &TaigaClient, attachment_id: i64) -> Result<()> {
        let token = credentials::get_api_token()?;
        client
            .delete_issue_attachment(&token, attachment_id)
            .await?;
        Ok(())
    }

    match fetch(&client, attachment_id).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, attachment_id).await
        }
        result => result,
    }
}

#[tauri::command]
pub async fn get_issue_attachments(
    client: tauri::State<'_, TaigaClient>,
    project_id: i64,
    issue_id: i64,
) -> Result<Vec<crate::domain::issue_detail::Attachment>> {
    async fn fetch(
        client: &TaigaClient,
        project_id: i64,
        issue_id: i64,
    ) -> Result<Vec<crate::domain::issue_detail::Attachment>> {
        let token = credentials::get_api_token()?;
        let attachments_dto = client
            .list_issue_attachments(&token, project_id, issue_id)
            .await?;
        Ok(attachments_dto.iter().map(|a| a.into()).collect())
    }

    match fetch(&client, project_id, issue_id).await {
        Err(crate::error::Error::Unauthorized) => {
            log::info!("Unauthorized, attempting token refresh");
            token_refresh::refresh_token(&client).await?;
            fetch(&client, project_id, issue_id).await
        }
        result => result,
    }
}
