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

#[tauri::command]
pub async fn get_taiga_base_url(
    repo: tauri::State<'_, crate::repositories::SqliteRepository>,
) -> Result<String> {
    let api_url = repo
        .get_config("taiga_api_url")
        .await?
        .ok_or_else(|| crate::error::Error::InvalidInput("Taiga API URL not found".to_string()))?;

    // Transform API URL to web URL
    // Cloud: https://api.taiga.io -> https://tree.taiga.io
    // Self-hosted: https://taiga.example.com/api/v1 -> https://taiga.example.com
    let base_url = if let Ok(parsed) = url::Url::parse(&api_url) {
        if parsed.host_str() == Some("api.taiga.io") {
            "https://tree.taiga.io".to_string()
        } else {
            // Strip common API path suffixes
            let mut result = api_url.clone();
            for suffix in &["/api/v1", "/api/v2", "/api"] {
                if result.ends_with(suffix) {
                    result = result.trim_end_matches(suffix).to_string();
                    break;
                }
            }
            result.trim_end_matches('/').to_string()
        }
    } else {
        api_url.trim_end_matches("/api/v1").trim_end_matches('/').to_string()
    };

    Ok(base_url)
}

#[tauri::command]
pub async fn get_taiga_api_url(
    repo: tauri::State<'_, crate::repositories::SqliteRepository>,
) -> Result<String> {
    let api_url = repo
        .get_config("taiga_api_url")
        .await?
        .ok_or_else(|| crate::error::Error::InvalidInput("Taiga API URL not found".to_string()))?;
    Ok(api_url)
}
