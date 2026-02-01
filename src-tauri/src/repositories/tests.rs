use super::*;
use crate::entities::{config, drafts, saved_views};
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

    let stmt = schema.create_table_from_entity(saved_views::Entity).to_owned();
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

#[tokio::test]
async fn test_create_view() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    let view = repo
        .create_view("My View", r#"{"status_exclude": true}"#, false, false)
        .await
        .unwrap();

    assert_eq!(view.name, "My View");
    assert_eq!(view.filter_data, r#"{"status_exclude": true}"#);
    assert!(!view.is_system);
    assert!(!view.is_default);
}

#[tokio::test]
async fn test_list_views() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    repo.create_view("View 1", "{}", false, false)
        .await
        .unwrap();
    repo.create_view("View 2", "{}", false, false)
        .await
        .unwrap();

    let views = repo.list_views().await.unwrap();
    assert_eq!(views.len(), 2);
}

#[tokio::test]
async fn test_delete_system_view_fails() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    let system_view = repo
        .create_view("System View", "{}", true, false)
        .await
        .unwrap();

    let result = repo.delete_view(system_view.id).await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Cannot delete system view"));
}

#[tokio::test]
async fn test_update_view() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    let view = repo
        .create_view("Original", "{}", false, false)
        .await
        .unwrap();

    repo.update_view(view.id, "Updated", r#"{"new": true}"#)
        .await
        .unwrap();

    let updated = repo.get_view(view.id).await.unwrap().unwrap();
    assert_eq!(updated.name, "Updated");
    assert_eq!(updated.filter_data, r#"{"new": true}"#);
}

#[tokio::test]
async fn test_touch_view() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    let view = repo
        .create_view("View", "{}", false, false)
        .await
        .unwrap();
    let original_last_used = view.last_used;

    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    repo.touch_view(view.id).await.unwrap();

    let touched = repo.get_view(view.id).await.unwrap().unwrap();
    assert!(touched.last_used > original_last_used);
}

#[tokio::test]
async fn test_set_default_view() {
    let conn = create_test_db().await;
    let repo = SqliteRepository::new(conn);

    let view1 = repo
        .create_view("View 1", "{}", false, true)
        .await
        .unwrap();
    let view2 = repo
        .create_view("View 2", "{}", false, false)
        .await
        .unwrap();

    assert!(repo.get_view(view1.id).await.unwrap().unwrap().is_default);

    repo.set_default_view(view2.id).await.unwrap();

    assert!(!repo.get_view(view1.id).await.unwrap().unwrap().is_default);
    assert!(repo.get_view(view2.id).await.unwrap().unwrap().is_default);
}
