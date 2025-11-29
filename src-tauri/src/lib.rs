pub mod commands;
pub mod domain;
pub mod error;
pub mod services;

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
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::auth_commands::login,
            commands::auth_commands::has_api_token,
            commands::auth_commands::logout,
            commands::user_commands::get_me,
            commands::project_commands::get_projects,
            commands::project_commands::list_issues
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
