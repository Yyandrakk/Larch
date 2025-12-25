use crate::error::Result;
use tauri::AppHandle;

/// Force close the application
/// Called by frontend when it's safe to close (no pending changes or user approved)
#[tauri::command]
#[allow(unreachable_code)]
pub fn force_close_app(app: AppHandle) -> Result<()> {
    app.exit(0);
    Ok(())
}
