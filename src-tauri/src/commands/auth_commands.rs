use crate::domain::user::User;
use crate::error::Result;
use crate::repositories::Repository;
use crate::services::credentials;
use taiga_client::TaigaClient;
use tauri::Manager;

#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    repo: tauri::State<'_, crate::repositories::SqliteRepository>,
    api_url: &str,
    username: &str,
    password: &str,
) -> Result<User> {
    let client = TaigaClient::new(api_url.parse()?);
    let auth_detail = client.login(username, password).await?;

    // Save API URL to config
    repo.save_config("taiga_api_url", api_url).await?;

    credentials::set_api_token(&auth_detail.auth_token)?;

    // After successful login, fetch user details
    let token = credentials::get_api_token()?;
    let me = client.get_me(&token).await?;

    // Store the authenticated client in Tauri's state
    app_handle.manage(client);

    Ok(me.into())
}

#[tauri::command]
pub fn has_api_token() -> Result<bool> {
    Ok(credentials::get_api_token().is_ok())
}

#[tauri::command]
pub fn logout() -> Result<()> {
    credentials::delete_api_token()
}
