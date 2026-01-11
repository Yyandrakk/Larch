use crate::error::Result;
use keyring::Entry;
use secrecy::Secret;

const SERVICE_NAME: &str = "larch-app";
const USER_NAME: &str = "taiga-api-token";
const REFRESH_TOKEN_USER: &str = "taiga-refresh-token";

pub fn set_api_token(token: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, USER_NAME)?;
    entry.set_password(token)?;
    Ok(())
}

pub fn get_api_token() -> Result<Secret<String>> {
    let entry = Entry::new(SERVICE_NAME, USER_NAME)?;
    let secret = entry.get_password()?;
    Ok(Secret::new(secret))
}

pub fn delete_api_token() -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, USER_NAME)?;
    entry.delete_password().ok();
    Ok(())
}

pub fn set_refresh_token(token: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, REFRESH_TOKEN_USER)?;
    entry.set_password(token)?;
    Ok(())
}

pub fn get_refresh_token() -> Result<Secret<String>> {
    let entry = Entry::new(SERVICE_NAME, REFRESH_TOKEN_USER)?;
    let secret = entry.get_password()?;
    Ok(Secret::new(secret))
}

pub fn delete_refresh_token() -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, REFRESH_TOKEN_USER)?;
    entry.delete_password().ok();
    Ok(())
}
