use crate::error::Result;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, Database, DatabaseConnection,
    EntityTrait, QueryFilter, Schema,
};
use std::fs;
use tauri::AppHandle;
use tauri::Manager;

const DB_NAME: &str = "larch.db";

pub async fn init_db(app: &AppHandle) -> Result<DatabaseConnection> {
    let app_dir = app.path().app_data_dir()?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    let db_path = app_dir.join(DB_NAME);
    let db_url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());

    let conn = Database::connect(&db_url)
        .await
        .map_err(|e| crate::error::Error::Database(e.to_string()))?;

    // Create tables if not exist (Simple migration for now)
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);

    let stmt = schema
        .create_table_from_entity(crate::entities::config::Entity)
        .if_not_exists()
        .to_owned();
    let stmt = builder.build(&stmt);
    conn.execute(stmt)
        .await
        .map_err(|e| crate::error::Error::Database(e.to_string()))?;

    let stmt = schema
        .create_table_from_entity(crate::entities::drafts::Entity)
        .if_not_exists()
        .to_owned();
    let stmt = builder.build(&stmt);
    conn.execute(stmt)
        .await
        .map_err(|e| crate::error::Error::Database(e.to_string()))?;

    let stmt = schema
        .create_table_from_entity(crate::entities::saved_views::Entity)
        .if_not_exists()
        .to_owned();
    let stmt = builder.build(&stmt);
    conn.execute(stmt)
        .await
        .map_err(|e| crate::error::Error::Database(e.to_string()))?;

    use crate::entities::saved_views;

    let existing = saved_views::Entity::find()
        .filter(saved_views::Column::Name.eq("Active Triage"))
        .one(&conn)
        .await
        .map_err(|e| crate::error::Error::Database(e.to_string()))?;

    if existing.is_none() {
        let active_triage = saved_views::ActiveModel {
            name: ActiveValue::Set("Active Triage".to_string()),
            filter_data: ActiveValue::Set("{\"status_exclude\": true}".to_string()),
            is_system: ActiveValue::Set(true),
            is_default: ActiveValue::Set(true),
            last_used: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        active_triage
            .insert(&conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;
    }

    migrate_selected_projects(&conn).await?;

    Ok(conn)
}

async fn migrate_selected_projects(conn: &DatabaseConnection) -> Result<()> {
    use crate::entities::{config, saved_views};

    let selected_projects_config = config::Entity::find_by_id("selected_projects")
        .one(conn)
        .await
        .map_err(|e| crate::error::Error::Database(e.to_string()))?;

    if let Some(config_entry) = selected_projects_config {
        let user_views = saved_views::Entity::find()
            .filter(saved_views::Column::IsSystem.eq(false))
            .all(conn)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))?;

        if user_views.is_empty() {
            let my_projects_exists = saved_views::Entity::find()
                .filter(saved_views::Column::Name.eq("My Projects"))
                .one(conn)
                .await
                .map_err(|e| crate::error::Error::Database(e.to_string()))?;

            if my_projects_exists.is_none() {
                let project_ids: Vec<i64> = serde_json::from_str(&config_entry.value)
                    .map_err(|e| crate::error::Error::Database(format!("Invalid JSON in selected_projects: {}", e)))?;

                let filter_data = serde_json::json!({
                    "project_ids": project_ids
                });

                saved_views::Entity::update_many()
                    .col_expr(saved_views::Column::IsDefault, sea_orm::sea_query::Expr::value(false))
                    .exec(conn)
                    .await
                    .map_err(|e| crate::error::Error::Database(e.to_string()))?;

                let my_projects_view = saved_views::ActiveModel {
                    name: ActiveValue::Set("My Projects".to_string()),
                    filter_data: ActiveValue::Set(filter_data.to_string()),
                    is_system: ActiveValue::Set(false),
                    is_default: ActiveValue::Set(true),
                    last_used: ActiveValue::Set(chrono::Utc::now().naive_utc()),
                    created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
                    ..Default::default()
                };
                my_projects_view
                    .insert(conn)
                    .await
                    .map_err(|e| crate::error::Error::Database(e.to_string()))?;

                log::info!(
                    "Migrated 'selected_projects' to 'My Projects' view. Projects: {:?}",
                    project_ids
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{config, saved_views};
    use sea_orm::{Database, Schema, Set};

    async fn create_test_db() -> DatabaseConnection {
        let db_url = "sqlite::memory:";
        let conn = Database::connect(db_url).await.unwrap();

        let builder = conn.get_database_backend();
        let schema = Schema::new(builder);

        let stmt = schema
            .create_table_from_entity(config::Entity)
            .if_not_exists()
            .to_owned();
        let stmt = builder.build(&stmt);
        conn.execute(stmt).await.unwrap();

        let stmt = schema
            .create_table_from_entity(saved_views::Entity)
            .if_not_exists()
            .to_owned();
        let stmt = builder.build(&stmt);
        conn.execute(stmt).await.unwrap();

        conn
    }

    #[tokio::test]
    async fn test_migration_creates_default_view() {
        let conn = create_test_db().await;

        let config_entry = config::ActiveModel {
            key: Set("selected_projects".to_string()),
            value: Set("[1, 2, 3]".to_string()),
        };
        config_entry.insert(&conn).await.unwrap();

        migrate_selected_projects(&conn).await.unwrap();

        let my_projects = saved_views::Entity::find()
            .filter(saved_views::Column::Name.eq("My Projects"))
            .one(&conn)
            .await
            .unwrap();

        assert!(my_projects.is_some());
        let view = my_projects.unwrap();
        assert_eq!(view.name, "My Projects");
        assert_eq!(view.filter_data, r#"{"project_ids":[1,2,3]}"#);
        assert!(!view.is_system);
        assert!(view.is_default);
    }

    #[tokio::test]
    async fn test_migration_does_not_run_if_user_views_exist() {
        let conn = create_test_db().await;

        let config_entry = config::ActiveModel {
            key: Set("selected_projects".to_string()),
            value: Set("[1, 2, 3]".to_string()),
        };
        config_entry.insert(&conn).await.unwrap();

        let user_view = saved_views::ActiveModel {
            name: Set("Existing User View".to_string()),
            filter_data: Set("{}".to_string()),
            is_system: Set(false),
            is_default: Set(false),
            last_used: Set(chrono::Utc::now().naive_utc()),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        user_view.insert(&conn).await.unwrap();

        migrate_selected_projects(&conn).await.unwrap();

        let my_projects = saved_views::Entity::find()
            .filter(saved_views::Column::Name.eq("My Projects"))
            .one(&conn)
            .await
            .unwrap();

        assert!(my_projects.is_none());
    }

    #[tokio::test]
    async fn test_migration_is_idempotent() {
        let conn = create_test_db().await;

        let config_entry = config::ActiveModel {
            key: Set("selected_projects".to_string()),
            value: Set("[1, 2, 3]".to_string()),
        };
        config_entry.insert(&conn).await.unwrap();

        migrate_selected_projects(&conn).await.unwrap();
        migrate_selected_projects(&conn).await.unwrap();

        let views = saved_views::Entity::find()
            .filter(saved_views::Column::Name.eq("My Projects"))
            .all(&conn)
            .await
            .unwrap();

        assert_eq!(views.len(), 1);
    }

    #[tokio::test]
    async fn test_migration_does_not_run_without_config() {
        let conn = create_test_db().await;

        migrate_selected_projects(&conn).await.unwrap();

        let my_projects = saved_views::Entity::find()
            .filter(saved_views::Column::Name.eq("My Projects"))
            .one(&conn)
            .await
            .unwrap();

        assert!(my_projects.is_none());
    }
}
