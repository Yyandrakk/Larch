pub mod commands;
pub mod domain;
pub mod entities;
pub mod error;
pub mod repositories;
pub mod services;

use crate::repositories::Repository;
use tauri::Manager;

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
            commands::issue_commands::get_issue_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
