use crate::entities::{config, drafts, saved_views};
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

    // SavedView operations
    async fn list_views(&self) -> Result<Vec<saved_views::Model>>;
    async fn get_view(&self, id: i32) -> Result<Option<saved_views::Model>>;
    async fn create_view(
        &self,
        name: &str,
        filter_data: &str,
        is_system: bool,
        is_default: bool,
    ) -> Result<saved_views::Model>;
    async fn update_view(&self, id: i32, name: &str, filter_data: &str) -> Result<()>;
    async fn delete_view(&self, id: i32) -> Result<()>;
    async fn touch_view(&self, id: i32) -> Result<()>;
    async fn set_default_view(&self, id: i32) -> Result<()>;
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

    async fn list_views(&self) -> Result<Vec<saved_views::Model>> {
        let views = saved_views::Entity::find()
            .all(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        Ok(views)
    }

    async fn get_view(&self, id: i32) -> Result<Option<saved_views::Model>> {
        let view = saved_views::Entity::find_by_id(id)
            .one(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        Ok(view)
    }

    async fn create_view(
        &self,
        name: &str,
        filter_data: &str,
        is_system: bool,
        is_default: bool,
    ) -> Result<saved_views::Model> {
        let now = chrono::Utc::now().naive_utc();

        if is_default {
            saved_views::Entity::update_many()
                .col_expr(saved_views::Column::IsDefault, sea_orm::sea_query::Expr::value(false))
                .exec(&self.conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;
        }

        let view = saved_views::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: Set(name.to_string()),
            filter_data: Set(filter_data.to_string()),
            is_system: Set(is_system),
            is_default: Set(is_default),
            created_at: Set(now),
            last_used: Set(now),
        };

        let result = view
            .insert(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        Ok(result)
    }

    async fn update_view(&self, id: i32, name: &str, filter_data: &str) -> Result<()> {
        let view = saved_views::Entity::find_by_id(id)
            .one(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        if let Some(existing_view) = view {
            let mut active: saved_views::ActiveModel = existing_view.into();
            active.name = Set(name.to_string());
            active.filter_data = Set(filter_data.to_string());
            active
                .update(&self.conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;
        } else {
            return Err(crate::error::Error::Database(format!("View with id {} not found", id)));
        }

        Ok(())
    }

    async fn delete_view(&self, id: i32) -> Result<()> {
        let view = saved_views::Entity::find_by_id(id)
            .one(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        if let Some(view_model) = view {
            if view_model.is_system {
                return Err(crate::error::Error::Database("Cannot delete system view".to_string()));
            }

            saved_views::Entity::delete_by_id(id)
                .exec(&self.conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;
        } else {
            return Err(crate::error::Error::Database(format!("View with id {} not found", id)));
        }

        Ok(())
    }

    async fn touch_view(&self, id: i32) -> Result<()> {
        let now = chrono::Utc::now().naive_utc();

        let view = saved_views::Entity::find_by_id(id)
            .one(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        if let Some(existing_view) = view {
            let mut active: saved_views::ActiveModel = existing_view.into();
            active.last_used = Set(now);
            active
                .update(&self.conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;
        } else {
            return Err(crate::error::Error::Database(format!("View with id {} not found", id)));
        }

        Ok(())
    }

    async fn set_default_view(&self, id: i32) -> Result<()> {
        saved_views::Entity::update_many()
            .col_expr(saved_views::Column::IsDefault, sea_orm::sea_query::Expr::value(false))
            .exec(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        let view = saved_views::Entity::find_by_id(id)
            .one(&self.conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        if let Some(existing_view) = view {
            let mut active: saved_views::ActiveModel = existing_view.into();
            active.is_default = Set(true);
            active
                .update(&self.conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;
        } else {
            return Err(crate::error::Error::Database(format!("View with id {} not found", id)));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
