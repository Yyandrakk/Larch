use crate::entities::config;
use crate::error::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    async fn get_config(&self, key: &str) -> Result<Option<String>>;
    async fn save_config(&self, key: &str, value: &str) -> Result<()>;
}

#[derive(Clone)]
pub struct SqliteRepository {
    conn: DatabaseConnection,
}

impl SqliteRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl Repository for SqliteRepository {
    async fn get_config(&self, key: &str) -> Result<Option<String>> {
        let config = config::Entity::find_by_id(key)
            .one(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        Ok(config.map(|c| c.value))
    }

    async fn save_config(&self, key: &str, value: &str) -> Result<()> {
        let config = config::ActiveModel {
            key: Set(key.to_string()),
            value: Set(value.to_string()),
        };

        // Insert or Update (Upsert)
        // SeaORM doesn't have a direct "upsert" for all DBs, but for SQLite ON CONFLICT works.
        // For simplicity, we'll try to find it first.

        let existing = config::Entity::find_by_id(key)
            .one(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        if let Some(_) = existing {
            let mut active: config::ActiveModel = existing.unwrap().into();
            active.value = Set(value.to_string());
            active
                .update(&self.conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;
        } else {
            config
                .insert(&self.conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;
        }

        Ok(())
    }
}
