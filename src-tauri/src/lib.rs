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
        .register_asynchronous_uri_scheme_protocol("taiga-auth", move |ctx, request, responder| {
            let app_handle = ctx.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let uri = request.uri().to_string();
                let target_url = uri.replace("taiga-auth://", "https://");

                let client = match app_handle.try_state::<taiga_client::TaigaClient>() {
                    Some(c) => c,
                    None => {
                        log::error!("TaigaClient not found in state when fetching {}", target_url);
                        if let Ok(response) = tauri::http::Response::builder()
                            .status(500)
                            .header("Access-Control-Allow-Origin", "*")
                            .body(vec![])
                        {
                            responder.respond(response);
                        }
                        return;
                    }
                };

                let token = crate::services::credentials::get_api_token().ok();

                match client.get_raw_resource(&target_url, token.as_ref()).await {
                    Ok((bytes, mime)) => {
                        match tauri::http::Response::builder()
                            .header("Access-Control-Allow-Origin", "*")
                            .header("Content-Type", mime)
                            .body(bytes)
                        {
                            Ok(response) => responder.respond(response),
                            Err(e) => {
                                log::error!("Failed to build response for {}: {}", target_url, e);
                                if let Ok(fallback) = tauri::http::Response::builder()
                                    .status(500)
                                    .body(vec![])
                                {
                                    responder.respond(fallback);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to proxy image {}: {}", target_url, e);
                        if let Ok(response) = tauri::http::Response::builder()
                            .status(404)
                            .body(vec![])
                        {
                            responder.respond(response);
                        }
                    }
                }
            });
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            commands::auth_commands::login,
            commands::auth_commands::has_api_token,
            commands::auth_commands::logout,
            commands::auth_commands::refresh_token,
            commands::auth_commands::get_taiga_base_url,
            commands::auth_commands::get_taiga_api_url,
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
            commands::issue_commands::get_issue_attachments,
            commands::app_commands::force_close_app,
            commands::view_commands::list_views,
            commands::view_commands::get_view,
            commands::view_commands::create_view,
            commands::view_commands::update_view,
            commands::view_commands::delete_view,
            commands::view_commands::switch_view,
            commands::view_commands::set_default_view,
            commands::view_commands::sanitize_views
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
