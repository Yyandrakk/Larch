use crate::domain::user::User;
use crate::error::Result;
use crate::services::credentials;
use taiga_client::TaigaClient;

#[tauri::command]
pub async fn get_me(client: tauri::State<'_, TaigaClient>) -> Result<User> {
    let token = credentials::get_api_token()?;
    let user = client.get_me(&token).await?;
    Ok(user.into())
}
