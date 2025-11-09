use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub photo: Option<String>,
    pub big_photo: Option<String>,
    pub gravatar_id: String,
    pub is_active: bool,
}

impl From<taiga_client::models::Me> for User {
    fn from(me: taiga_client::models::Me) -> Self {
        Self {
            id: me.id,
            username: me.username,
            full_name: me.full_name,
            email: me.email,
            photo: me.photo,
            big_photo: me.big_photo,
            gravatar_id: me.gravatar_id,
            is_active: me.is_active,
        }
    }
}
