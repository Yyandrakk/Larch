use crate::error::{Error, Result};
use crate::services::credentials;
use secrecy::ExposeSecret;
use taiga_client::TaigaClient;

pub async fn refresh_token(client: &TaigaClient) -> Result<()> {
    let refresh = credentials::get_refresh_token()?;
    let new_tokens = client.refresh_token(refresh.expose_secret()).await?;

    credentials::set_api_token(&new_tokens.auth_token)?;
    credentials::set_refresh_token(&new_tokens.refresh)?;

    log::info!("Token refreshed successfully");
    Ok(())
}

pub fn is_unauthorized(err: &Error) -> bool {
    matches!(err, Error::Unauthorized)
}
