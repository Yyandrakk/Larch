use crate::domain::user::User;
use crate::error::Result;
use crate::repositories::Repository;
use crate::services::credentials;
use secrecy::ExposeSecret;
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

    repo.save_config("taiga_api_url", api_url).await?;

    credentials::set_api_token(&auth_detail.auth_token)?;

    if let Some(refresh) = &auth_detail.refresh {
        credentials::set_refresh_token(refresh)?;
    }

    let token = credentials::get_api_token()?;
    let me = client.get_me(&token).await?;

    app_handle.manage(client);

    Ok(me.into())
}

#[tauri::command]
pub fn has_api_token() -> Result<bool> {
    Ok(credentials::get_api_token().is_ok())
}

#[tauri::command]
pub fn logout() -> Result<()> {
    credentials::delete_api_token()?;
    credentials::delete_refresh_token()?;
    Ok(())
}

#[tauri::command]
pub async fn refresh_token(client: tauri::State<'_, TaigaClient>) -> Result<()> {
    let refresh = credentials::get_refresh_token()?;
    let new_tokens = client.refresh_token(refresh.expose_secret()).await?;

    credentials::set_api_token(&new_tokens.auth_token)?;
    credentials::set_refresh_token(&new_tokens.refresh)?;

    log::info!("Token refreshed successfully");
    Ok(())
}
