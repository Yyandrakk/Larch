use crate::entities::{config, drafts};
use crate::error::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    async fn get_config(&self, key: &str) -> Result<Option<String>>;
    async fn save_config(&self, key: &str, value: &str) -> Result<()>;

    // Draft operations
    async fn save_draft(&self, related_id: &str, draft_type: &str, content: &str) -> Result<()>;
    async fn get_draft(&self, related_id: &str, draft_type: &str) -> Result<Option<String>>;
    async fn delete_draft(&self, related_id: &str, draft_type: &str) -> Result<()>;
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

        if let Some(existing_model) = existing {
            let mut active: config::ActiveModel = existing_model.into();
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

    async fn save_draft(&self, related_id: &str, draft_type: &str, content: &str) -> Result<()> {
        let now = chrono::Utc::now().naive_utc();

        // Find existing draft by related_id and draft_type
        let existing = drafts::Entity::find()
            .filter(drafts::Column::RelatedId.eq(related_id))
            .filter(drafts::Column::DraftType.eq(draft_type))
            .one(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        if let Some(existing_draft) = existing {
            // Update existing draft
            let mut active: drafts::ActiveModel = existing_draft.into();
            active.content = Set(content.to_string());
            active.updated_at = Set(now);
            active
                .update(&self.conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;
        } else {
            // Insert new draft
            let draft = drafts::ActiveModel {
                id: sea_orm::ActiveValue::NotSet,
                related_id: Set(related_id.to_string()),
                draft_type: Set(draft_type.to_string()),
                content: Set(content.to_string()),
                updated_at: Set(now),
            };
            draft
                .insert(&self.conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;
        }

        Ok(())
    }

    async fn get_draft(&self, related_id: &str, draft_type: &str) -> Result<Option<String>> {
        let draft = drafts::Entity::find()
            .filter(drafts::Column::RelatedId.eq(related_id))
            .filter(drafts::Column::DraftType.eq(draft_type))
            .one(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        Ok(draft.map(|d| d.content))
    }

    async fn delete_draft(&self, related_id: &str, draft_type: &str) -> Result<()> {
        drafts::Entity::delete_many()
            .filter(drafts::Column::RelatedId.eq(related_id))
            .filter(drafts::Column::DraftType.eq(draft_type))
            .exec(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
