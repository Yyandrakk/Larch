
## shadcn-svelte Documentation
- Updated `AGENTS.md` to include a reference to `llms.txt` for `shadcn-svelte`.
- Context7 library ID for `shadcn-svelte` documentation is `/llmstxt/shadcn-svelte_llms_txt`.

- Added SavedView and SavedViewInput TypeScript interfaces to src/lib/types.ts following the schema defined in docs/doc-design.md and user instructions.
- SavedView includes id, name, filter_data (as string), is_system, is_default, last_used, and created_at.
- SavedViewInput omits id and timestamps.

## SavedView Entity Implementation
- Established the `SavedView` entity following the SeaORM pattern used in `drafts.rs`.
- Used `String` for `filter_data` to maintain SQLite compatibility, as per project guidelines.
- Successfully exported the new entity from `src-tauri/src/entities/mod.rs`.
- Verified the implementation with `cargo check`.
- Added missing shadcn-svelte components: button-group, dropdown-menu, alert-dialog.
- Used 'npx shadcn-svelte@latest add ... -y -o' to bypass interactive prompts in CLI environments, then restored existing files (button, separator) that were modified by the CLI to ensure no side effects on existing code.
- Verified with 'pnpm check' which ensures Svelte 5 runes and TypeScript types are correct for the newly added components.

## DB Initialization & Seeding (2026-02-01)
- Successfully added `saved_views` table creation to `src-tauri/src/services/db.rs` using `create_table_from_entity`.
- Implemented seeding for the "Active Triage" system view. 
- Used `chrono::Utc::now().naive_utc()` to match SeaORM's `DateTime` (NaiveDateTime) requirement in the entity.
- Verified that `filter_data` for "Active Triage" is set to `{"status_exclude": true}`.
- Followed the project's preference for minimal comments by removing the newly added seeding comment.

## Repository Trait Extension (2026-02-01)
- Extended `Repository` trait in `src-tauri/src/repositories/mod.rs` with SavedView CRUD methods.
- Added import for `saved_views` entity to enable `saved_views::Model` return types.
- Followed existing pattern of section comments (e.g., `// Draft operations`) to group related methods.
- Methods include: `list_views`, `get_view`, `create_view`, `update_view`, `delete_view`, `touch_view`, `set_default_view`.
- As expected, `cargo check` now fails on `SqliteRepository` implementation due to missing trait implementations (to be addressed in Task 5).
- Committed with semantic style: `feat(repo): extend Repository trait with SavedView methods`.

## Repository Implementation (2026-02-01)
- Implemented all SavedView repository methods in `SqliteRepository` following established CRUD patterns.
- `create_view`: Auto-generates `created_at` and `last_used` with `chrono::Utc::now().naive_utc()`. Clears all `is_default` flags before setting if `is_default=true` is requested.
- `touch_view`: Updates ONLY the `last_used` field using ActiveModel pattern.
- `set_default_view`: First clears all defaults using `update_many().col_expr()`, then sets target view to default.
- `delete_view`: Validates `is_system` flag and returns `Error::Database("Cannot delete system view")` if true.
- Added comprehensive test coverage: `test_create_view`, `test_list_views`, `test_delete_system_view_fails`, `test_update_view`, `test_touch_view`, `test_set_default_view`.
- Updated `create_test_db()` helper to include `saved_views::Entity` table creation.
- All tests pass: 9 repository tests executed successfully.
- Committed with: `feat(repo): implement SavedView repository methods`.

## Migration Implementation (2026-02-01)
- Added `migrate_selected_projects` function to `src-tauri/src/services/db.rs` for legacy config migration.
- Migration logic checks three conditions: (1) `selected_projects` config exists, (2) no user views exist, (3) "My Projects" view doesn't exist.
- Used `serde_json::from_str` to parse the JSON array from `selected_projects` config value.
- Constructed `filter_data` as `{"project_ids": [1, 2, 3]}` using `serde_json::json!` macro.
- Cleared all `is_default` flags before setting the new view as default to maintain data integrity.
- Migration is fully idempotent: checks for existing "My Projects" view to prevent duplicates.
- Added comprehensive test coverage: 4 test cases covering all scenarios (creation, user views exist, idempotency, no config).
- All tests pass (22 total tests in the project).
- No clippy warnings.
