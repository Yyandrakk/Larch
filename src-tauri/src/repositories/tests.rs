use super::*;
use crate::entities::{config, drafts};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};

// Helper to create an in-memory DB with schema
async fn create_test_db() -> DatabaseConnection {
    let db_url = "sqlite::memory:";
    let conn = Database::connect(db_url).await.unwrap();

    // Create tables
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);

    let stmt = schema.create_table_from_entity(config::Entity).to_owned();
    let stmt = builder.build(&stmt);
    conn.execute(stmt).await.unwrap();

    let stmt = schema.create_table_from_entity(drafts::Entity).to_owned();
    let stmt = builder.build(&stmt);
    conn.execute(stmt).await.unwrap();

    conn
}

#[tokio::test]
async fn test_save_and_get_config() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    repo.save_config("test_key", "test_value").await.unwrap();
    let value = repo.get_config("test_key").await.unwrap();
    assert_eq!(value, Some("test_value".to_string()));

    // Test overwrite
    repo.save_config("test_key", "new_value").await.unwrap();
    let value = repo.get_config("test_key").await.unwrap();
    assert_eq!(value, Some("new_value".to_string()));
}

#[tokio::test]
async fn test_get_config_missing() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    let value = repo.get_config("missing_key").await.unwrap();
    assert_eq!(value, None);
}

#[tokio::test]
async fn test_draft_lifecycle() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    let related_id = "issue_123";
    let draft_type = "comment";
    let content = "Draft Content";

    // 1. Save Draft
    repo.save_draft(related_id, draft_type, content)
        .await
        .unwrap();

    // 2. Get Draft
    let fetched = repo.get_draft(related_id, draft_type).await.unwrap();
    assert_eq!(fetched, Some(content.to_string()));

    // 3. Update Draft
    let new_content = "Updated Content";
    repo.save_draft(related_id, draft_type, new_content)
        .await
        .unwrap();
    let fetched = repo.get_draft(related_id, draft_type).await.unwrap();
    assert_eq!(fetched, Some(new_content.to_string()));

    // 4. Delete Draft
    repo.delete_draft(related_id, draft_type).await.unwrap();
    let fetched = repo.get_draft(related_id, draft_type).await.unwrap();
    assert_eq!(fetched, None);
}
