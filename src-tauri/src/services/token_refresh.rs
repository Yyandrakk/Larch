use crate::error::{Error, Result};
use crate::services::credentials;
use secrecy::ExposeSecret;
use taiga_client::TaigaClient;

pub async fn refresh_token(client: &TaigaClient) -> Result<()> {
    let refresh = credentials::get_refresh_token()?;
    let new_tokens = client.refresh_token(refresh.expose_secret()).await?;

    // Persist refresh token first to avoid stale refresh token if set_api_token succeeds but set_refresh_token fails
    credentials::set_refresh_token(&new_tokens.refresh)?;
    credentials::set_api_token(&new_tokens.auth_token)?;

    log::info!("Token refreshed successfully");
    Ok(())
}

pub fn is_unauthorized(err: &Error) -> bool {
    matches!(err, Error::Unauthorized)
}
