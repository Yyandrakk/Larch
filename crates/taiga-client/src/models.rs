use serde::{Deserialize, Serialize};

// Represents the JSON body for a password-based authentication request.
#[derive(Debug, Serialize)]
pub struct LoginRequest<'a> {
    #[serde(rename = "type")]
    pub r#type: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

// Represents the successful authentication response from the Taiga API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthDetail {
    pub id: i64,
    pub username: String,
    pub auth_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Me {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub photo: Option<String>,
    pub big_photo: Option<String>,
    pub gravatar_id: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserShort {
    pub id: i64,
    pub username: String,
    pub full_name_display: String,
    pub photo: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectDto {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub owner: UserShort,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueDto {
    pub id: i64,
    pub subject: String,
    pub project: i64,
    pub status: i64,
    pub owner: Option<i64>,
    pub assigned_to: Option<i64>,
}
