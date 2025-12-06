use crate::error::Result;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};
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

    Ok(conn)
}
