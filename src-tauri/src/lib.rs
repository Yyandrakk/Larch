pub mod commands;
pub mod domain;
pub mod entities;
pub mod error;
pub mod repositories;
pub mod services;

use crate::repositories::Repository;
use tauri::{Emitter, Manager};

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
                app.handle().plugin(tauri_plugin_mcp_bridge::init())?;
            }

            let conn = tauri::async_runtime::block_on(services::db::init_db(app.handle()))?;
            log::info!("Database initialized");

            let repository = repositories::SqliteRepository::new(conn);

            // Restore session
            let api_url_opt =
                tauri::async_runtime::block_on(repository.get_config("taiga_api_url"))?;
            if let Some(api_url) = api_url_opt {
                log::info!("Found saved API URL: {}", api_url);
                if let Ok(url) = api_url.parse() {
                    let client = taiga_client::TaigaClient::new(url);
                    app.manage(client);
                    log::info!("Restored Taiga session for {}", api_url);
                } else {
                    log::error!("Failed to parse saved API URL: {}", api_url);
                }
            } else {
                log::info!("No saved API URL found in config");
            }

            app.manage(repository);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::auth_commands::login,
            commands::auth_commands::has_api_token,
            commands::auth_commands::logout,
            commands::user_commands::get_me,
            commands::project_commands::get_projects,
            commands::project_commands::list_issues,
            commands::project_commands::get_selected_projects,
            commands::project_commands::save_selected_projects,
            commands::project_commands::get_aggregated_issues,
            commands::project_commands::get_project_metadata,
            commands::issue_commands::get_issue_detail,
            commands::issue_commands::get_issue_history,
            commands::issue_commands::change_issue_status,
            commands::issue_commands::add_issue_comment,
            commands::draft_commands::save_local_draft,
            commands::draft_commands::get_local_draft,
            commands::draft_commands::delete_local_draft,
            commands::issue_commands::commit_issue_description,
            commands::issue_commands::change_issue_assignee,
            commands::issue_commands::change_issue_priority,
            commands::issue_commands::change_issue_severity,
            commands::issue_commands::change_issue_type,
            commands::issue_commands::update_issue_tags,
            commands::issue_commands::upload_issue_attachment,
            commands::issue_commands::delete_issue_attachment,
            commands::app_commands::force_close_app
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Prevent the window from closing immediately
                api.prevent_close();
                // Emit event to frontend to check for unsaved changes
                if let Err(e) = window.emit("app-close-requested", ()) {
                    log::error!("Failed to emit app-close-requested: {}", e);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
