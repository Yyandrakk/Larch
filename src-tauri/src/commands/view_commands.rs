use crate::entities::saved_views;
use crate::error::Result;
use crate::repositories::{Repository, SqliteRepository};

/// Lists all saved views from the database.
#[tauri::command]
pub async fn list_views(
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<Vec<saved_views::Model>> {
    repository.list_views().await.map_err(|e| {
        log::error!("Failed to list views: {}", e);
        e
    })
}

/// Gets a specific saved view by ID.
#[tauri::command]
pub async fn get_view(
    id: i32,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<Option<saved_views::Model>> {
    repository.get_view(id).await.map_err(|e| {
        log::error!("Failed to get view {}: {}", id, e);
        e
    })
}

/// Creates a new saved view.
/// User-created views have is_system=false and is_default=false by default.
#[tauri::command]
pub async fn create_view(
    name: String,
    filter_data: String,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<saved_views::Model> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err(crate::error::Error::InvalidInput(
            "View name cannot be empty".to_string(),
        ));
    }

    repository
        .create_view(&name, &filter_data, false, false)
        .await
        .map_err(|e| {
            log::error!("Failed to create view '{}': {}", name, e);
            e
        })
}

/// Updates an existing saved view's name and filter data.
#[tauri::command]
pub async fn update_view(
    id: i32,
    name: String,
    filter_data: String,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<()> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err(crate::error::Error::InvalidInput(
            "View name cannot be empty".to_string(),
        ));
    }

    repository
        .update_view(id, &name, &filter_data)
        .await
        .map_err(|e| {
            log::error!("Failed to update view {}: {}", id, e);
            e
        })
}

/// Deletes a saved view by ID.
/// System views cannot be deleted (enforced by repository layer).
#[tauri::command]
pub async fn delete_view(
    id: i32,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<()> {
    repository.delete_view(id).await.map_err(|e| {
        log::error!("Failed to delete view {}: {}", id, e);
        e
    })
}

/// Switches to a saved view by ID.
/// This touches the view (updates last_used timestamp) and returns the full view model.
#[tauri::command]
pub async fn switch_view(
    id: i32,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<saved_views::Model> {
    repository.touch_view(id).await.map_err(|e| {
        log::error!("Failed to touch view {}: {}", id, e);
        e
    })
}

/// Sets a view as the default view.
/// Only one view can be default at a time.
#[tauri::command]
pub async fn set_default_view(
    id: i32,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<()> {
    repository.set_default_view(id).await.map_err(|e| {
        log::error!("Failed to set default view {}: {}", id, e);
        e
    })
}

/// Sanitizes all views by removing references to IDs that no longer exist.
#[tauri::command]
pub async fn sanitize_views(
    valid_project_ids: Option<Vec<i64>>,
    valid_status_ids: Option<Vec<i64>>,
    repository: tauri::State<'_, SqliteRepository>,
) -> Result<()> {
    crate::services::view_sanitizer::sanitize_all_views(
        &repository as &SqliteRepository,
        valid_project_ids.as_deref(),
        valid_status_ids.as_deref(),
    )
    .await
    .map_err(|e| {
        log::error!("sanitize_views failed: {:?}", e);
        e
    })
}
