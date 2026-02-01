use crate::error::Result;
use crate::repositories::Repository;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize)]
struct FilterData {
    #[serde(skip_serializing_if = "Option::is_none")]
    status_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_exclude: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_exclude: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_exclude: Option<bool>,
}

pub async fn sanitize_all_views<R: Repository>(
    repo: &R,
    valid_project_ids: Option<&[i64]>,
    valid_status_ids: Option<&[i64]>,
) -> Result<()> {
    let valid_projects: Option<HashSet<i64>> =
        valid_project_ids.map(|ids| ids.iter().copied().collect());
    let valid_statuses: Option<HashSet<i64>> =
        valid_status_ids.map(|ids| ids.iter().copied().collect());

    let views = repo.list_views().await?;

    for view in views {
        if view.is_system {
            continue;
        }

        let filter_data: FilterData = match serde_json::from_str(&view.filter_data) {
            Ok(data) => data,
            Err(e) => {
                log::warn!("Failed to parse filter_data for view {}: {}", view.id, e);
                continue;
            }
        };

        let mut changed = false;
        let mut new_filter = filter_data;

        if let Some(ref valid_projects) = valid_projects {
            if let Some(ref mut project_ids) = new_filter.project_ids {
                let original_len = project_ids.len();
                project_ids.retain(|id| valid_projects.contains(id));
                if project_ids.len() != original_len {
                    changed = true;
                }
            }
        }

        if let Some(ref valid_statuses) = valid_statuses {
            if let Some(ref mut status_ids) = new_filter.status_ids {
                let original_len = status_ids.len();
                status_ids.retain(|id| valid_statuses.contains(id));
                if status_ids.len() != original_len {
                    changed = true;
                }
            }
        }

        if changed {
            let new_filter_json = serde_json::to_string(&new_filter)
                .expect("Failed to serialize FilterData");
            
            repo.update_view(view.id, &view.name, &new_filter_json).await?;
            log::info!("Sanitized view {}: removed orphan IDs", view.name);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{config, drafts, saved_views};
    use crate::repositories::SqliteRepository;
    use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};

    async fn create_test_db() -> DatabaseConnection {
        let db_url = "sqlite::memory:";
        let conn = Database::connect(db_url).await.unwrap();

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
    async fn test_orphan_removal() {
        let conn = create_test_db().await;
        let repo = SqliteRepository::new(conn);

        let filter_with_orphans = r#"{
            "project_ids": [1, 2, 999, 888],
            "status_ids": [10, 20, 777],
            "assignee_ids": [5, 6]
        }"#;

        let view = repo
            .create_view("Test View", filter_with_orphans, false, false)
            .await
            .unwrap();

        let valid_projects = vec![1, 2];
        let valid_statuses = vec![10];

        sanitize_all_views(&repo, Some(&valid_projects), Some(&valid_statuses))
            .await
            .unwrap();

        let sanitized_view = repo.get_view(view.id).await.unwrap().unwrap();
        let sanitized_filter: FilterData = serde_json::from_str(&sanitized_view.filter_data).unwrap();

        assert_eq!(sanitized_filter.project_ids, Some(vec![1, 2]));
        assert_eq!(sanitized_filter.status_ids, Some(vec![10]));
        assert_eq!(sanitized_filter.assignee_ids, Some(vec![5, 6]));
    }

    #[tokio::test]
    async fn test_system_views_not_sanitized() {
        let conn = create_test_db().await;
        let repo = SqliteRepository::new(conn);

        let filter_with_orphans = r#"{
            "project_ids": [999]
        }"#;

        let system_view = repo
            .create_view("System View", filter_with_orphans, true, false)
            .await
            .unwrap();

        sanitize_all_views(&repo, Some(&[]), Some(&[])).await.unwrap();

        let unchanged_view = repo.get_view(system_view.id).await.unwrap().unwrap();
        assert_eq!(unchanged_view.filter_data, filter_with_orphans);
    }

    #[tokio::test]
    async fn test_valid_ids_preserved() {
        let conn = create_test_db().await;
        let repo = SqliteRepository::new(conn);

        let filter_all_valid = r#"{
            "project_ids": [1, 2],
            "status_ids": [10, 20]
        }"#;

        let view = repo
            .create_view("Valid View", filter_all_valid, false, false)
            .await
            .unwrap();

        sanitize_all_views(&repo, Some(&[1, 2]), Some(&[10, 20]))
            .await
            .unwrap();

        let unchanged_view = repo.get_view(view.id).await.unwrap().unwrap();
        let unchanged_filter: FilterData = serde_json::from_str(&unchanged_view.filter_data).unwrap();

        assert_eq!(unchanged_filter.project_ids, Some(vec![1, 2]));
        assert_eq!(unchanged_filter.status_ids, Some(vec![10, 20]));
    }

    #[tokio::test]
    async fn test_empty_valid_sets_clears_all() {
        let conn = create_test_db().await;
        let repo = SqliteRepository::new(conn);

        let filter_with_ids = r#"{
            "project_ids": [1, 2, 3],
            "status_ids": [10, 20]
        }"#;

        let view = repo
            .create_view("Clear All", filter_with_ids, false, false)
            .await
            .unwrap();

        sanitize_all_views(&repo, Some(&[]), Some(&[])).await.unwrap();

        let sanitized_view = repo.get_view(view.id).await.unwrap().unwrap();
        let sanitized_filter: FilterData = serde_json::from_str(&sanitized_view.filter_data).unwrap();

        assert_eq!(sanitized_filter.project_ids, Some(vec![]));
        assert_eq!(sanitized_filter.status_ids, Some(vec![]));
    }
}
