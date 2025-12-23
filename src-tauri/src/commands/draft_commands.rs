use crate::error::Result;
use crate::repositories::{Repository, SqliteRepository};

/// Saves a draft to the local SQLite database.
/// This is an offline operation - no API calls are made.
#[tauri::command]
pub async fn save_local_draft(
    related_id: String,
    draft_type: String,
    content: String,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<()> {
    if related_id.trim().is_empty() || draft_type.trim().is_empty() {
        return Err(crate::error::Error::InvalidInput(
            "related_id and draft_type cannot be empty".to_string(),
        ));
    }

    repository
        .save_draft(&related_id, &draft_type, &content)
        .await
        .map_err(|e| {
            log::error!("Failed to save draft: {}", e);
            e
        })?;

    Ok(())
}

#[tauri::command]
pub async fn get_local_draft(
    related_id: String,
    draft_type: String,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<Option<String>> {
    if related_id.trim().is_empty() || draft_type.trim().is_empty() {
        return Err(crate::error::Error::InvalidInput(
            "related_id and draft_type cannot be empty".to_string(),
        ));
    }

    log::info!("Getting draft");

    let draft = repository
        .get_draft(&related_id, &draft_type)
        .await
        .map_err(|e| {
            log::error!("Failed to get draft: {}", e);
            e
        })?;

    Ok(draft)
}

/// Deletes a draft from the local SQLite database.
#[tauri::command]
pub async fn delete_local_draft(
    related_id: String,
    draft_type: String,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<()> {
    if related_id.trim().is_empty() || draft_type.trim().is_empty() {
        return Err(crate::error::Error::InvalidInput(
            "related_id and draft_type cannot be empty".to_string(),
        ));
    }

    repository
        .delete_draft(&related_id, &draft_type)
        .await
        .map_err(|e| {
            log::error!("Failed to delete draft: {}", e);
            e
        })?;

    Ok(())
}
