# Saved Views v2.1

## TL;DR

> **Quick Summary**: Implement persistent saved views for the Larch triage dashboard, enabling users to save, switch between, and manage custom filter configurations with a view switcher dropdown and split-button save actions.
>
> **Deliverables**:
>
> - SQLite `saved_views` table with SeaORM entity and repository methods
> - Tauri commands for view CRUD operations
> - ViewSwitcher dropdown component (replaces static title)
> - SaveSplitButton for Save/Save As actions
> - Dirty state detection with visual indicator
> - Keyboard shortcuts (Cmd+S) and UX polish
>
> **Estimated Effort**: Large (3-4 days)
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: M1 (DB) -> M2 (Commands) -> M3 (Frontend State) -> M4 (UI Components) -> M5 (Polish)

---

## Context

### Original Request

Implement Version 2.1: Saved Views as specified in `docs/doc-design.md`. Features include:

1. View Switcher dropdown replacing static "All Issues" title
2. Split Button for Save/Save As in filter bar
3. Dirty State indicator when filters differ from persisted view
4. Backend validation to sanitize orphan project/status IDs
5. System-level "Active Triage" view (locked, non-deletable)

### Interview Summary

**Key Discussions**:

- **Unsaved changes behavior**: Auto-discard silently when switching views (no confirmation dialog)
- **System views**: Only "Active Triage" as system view (no additional presets)
- **View Switcher position**: Replace "All Issues" title in DashboardScreen header area
- **UX Polish scope**: Include keyboard shortcuts (Cmd+S, Cmd+Shift+S) and animated dirty indicator

**Research Findings**:

- Current filter state lives ephemerally in `DashboardScreen.svelte` via `$state<FilterObject>({})`
- Only `selected_projects` currently persisted to `config` table (not full FilterObject)
- UI uses Popover pattern for complex dropdowns (see `AddFilterDropdown.svelte`)
- Repository pattern with async trait + `SqliteRepository` implementation
- FilterObject type synced between TS (`src/lib/types.ts`) and Rust (`src-tauri/src/commands/project_commands.rs`)

### Metis Review

**Identified Gaps** (addressed):

- Migration strategy for existing `selected_projects` config: Will migrate to default view on first run
- JSON normalization rules: Arrays will be sorted, empty arrays treated as undefined
- View Switcher default state: Show "Active Triage" on fresh install
- System view immutability: Force "Save As" when Active Triage is active and dirty
- Keyboard shortcut scope: Dashboard-only (not global)
- Orphan cleanup trigger: On app startup (once per session)

---

## Work Objectives

### Core Objective

Enable users to persist, switch between, and manage named filter configurations ("views") to eliminate manual re-configuration of complex filter combinations for different work contexts.

### Concrete Deliverables

- `src-tauri/src/entities/saved_views.rs` - SeaORM entity
- `src-tauri/src/commands/view_commands.rs` - Tauri CRUD commands
- `src/lib/components/dashboard/ViewSwitcher.svelte` - Dropdown component
- `src/lib/components/dashboard/SaveSplitButton.svelte` - Split button component
- `src/lib/components/dashboard/SaveViewDialog.svelte` - Name input modal
- `src/lib/utils/filterUtils.ts` - Deep equality comparison
- Updated `DashboardScreen.svelte` with view state management

### Definition of Done

- [ ] `just test-rust` passes with new repository tests
- [ ] Views persist across app restarts (verified via SQLite query)
- [ ] Active Triage cannot be deleted (error returned from command)
- [ ] Dirty indicator appears when filters differ from saved view
- [ ] Cmd+S saves current view (when editable and dirty)
- [ ] All i18n strings added to translation files

### Must Have

- View CRUD (create, read, update, delete)
- View switching with auto-discard of unsaved changes
- Dirty state detection with normalized comparison
- System view "Active Triage" (locked/immutable)
- Split button with Save and "Save as new..." actions
- Migration of existing `selected_projects` to default view
- Orphan ID sanitization on app startup

### Must NOT Have (Guardrails)

- NO cloud synchronization (local-first only)
- NO multi-window view synchronization
- NO view sharing/export functionality
- NO additional system view presets (only Active Triage)
- NO drag-to-reorder views (defer to v2.2)
- NO view templates or duplication features
- NO global keyboard shortcuts (Dashboard-only)
- NO confirmation dialogs on view switch (auto-discard confirmed)
- DO NOT use legacy Svelte stores (`writable`, `derived` from svelte/store)
- USE `ButtonGroup` + `DropdownMenu` for split button (shadcn-svelte native components)
- USE `DropdownMenu` for ViewSwitcher (not Popover) for better menu semantics

---

## Verification Strategy (MANDATORY)

### Test Decision

- **Infrastructure exists**: YES - Rust tests via `cargo test`, Svelte via `bun test`
- **User wants tests**: TDD for backend, integration tests for frontend
- **Framework**: Rust `tokio::test`, Frontend `@testing-library/svelte` + Playwright

### Rust Unit Tests

Each repository method requires test coverage:

```rust
#[tokio::test]
async fn test_create_and_retrieve_view() {
    let repo = create_test_repo().await;
    let view = repo.create_view("My View", r#"{"project_ids":[1,2]}"#, false, false).await.unwrap();
    let retrieved = repo.get_view(view.id).await.unwrap().unwrap();
    assert_eq!(retrieved.name, "My View");
}
```

### Frontend State Tests

Dirty state detection via programmatic assertion:

```typescript
test('dirty state after filter mutation', () => {
	const persisted = { project_ids: [1, 2] };
	const current = { project_ids: [1, 2, 3] };
	expect(deepEqual(persisted, current)).toBe(false);
});
```

### Playwright E2E (Optional Polish)

UI behavior verification:

```typescript
test('view switcher shows active view name', async ({ page }) => {
	await page.goto('http://localhost:1420');
	await expect(page.locator('[data-testid="view-switcher-trigger"]')).toHaveText('Active Triage');
});
```

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately):
├── Task 1: Create SavedView SeaORM entity
├── Task 2: Add saved_views table creation to db.rs
├── Task 3: Add TypeScript types for SavedView
├── Task 17: Update AGENTS.md with shadcn-svelte docs reference
└── Task 18: Add missing shadcn-svelte components (ButtonGroup, DropdownMenu)

Wave 2 (After Wave 1):
├── Task 4: Extend Repository trait with view methods
├── Task 5: Implement repository methods in SqliteRepository
├── Task 6: Add migration logic for selected_projects
└── Task 7: Implement orphan sanitization logic

Wave 3 (After Wave 2):
├── Task 8: Create view_commands.rs with Tauri commands
└── Task 9: Register commands in lib.rs and add frontend constants

Wave 4 (After Wave 3 + Task 18):
├── Task 10: Create filterUtils.ts with deep equality
├── Task 11: Build ViewSwitcher component (uses DropdownMenu)
├── Task 12: Build SaveSplitButton component (uses ButtonGroup + DropdownMenu)
└── Task 13: Build SaveViewDialog component

Wave 5 (After Wave 4):
├── Task 14: Integrate view state into DashboardScreen
├── Task 15: Add keyboard shortcuts
└── Task 16: Add animated dirty indicator + polish

Critical Path: Task 1 → Task 4 → Task 8 → Task 14
Parallel Speedup: ~35% faster than sequential
```

### Dependency Matrix

| Task | Depends On        | Blocks         | Can Parallelize With |
| ---- | ----------------- | -------------- | -------------------- |
| 1    | None              | 4, 5, 6        | 2, 3                 |
| 2    | None              | 6              | 1, 3                 |
| 3    | None              | 10, 11, 12, 13 | 1, 2                 |
| 4    | 1                 | 5, 8           | None                 |
| 5    | 4                 | 8              | 6, 7                 |
| 6    | 2, 5              | 8              | 7                    |
| 7    | 5                 | 8              | 6                    |
| 8    | 5, 6, 7           | 14             | 9                    |
| 9    | 8                 | 14             | None                 |
| 10   | 3                 | 14             | 11, 12, 13           |
| 11   | 3                 | 14             | 10, 12, 13           |
| 12   | 3                 | 14             | 10, 11, 13           |
| 13   | 3                 | 14             | 10, 11, 12           |
| 14   | 9, 10, 11, 12, 13 | 15, 16         | None                 |
| 15   | 14                | None           | 16                   |
| 16   | 14                | None           | 15                   |

### Agent Dispatch Summary

| Wave | Tasks          | Recommended Agents                                     |
| ---- | -------------- | ------------------------------------------------------ |
| 1    | 1, 2, 3        | 3x quick agents in parallel                            |
| 2    | 4, 5, 6, 7     | 2x unspecified-low (sequential 4→5, then parallel 6,7) |
| 3    | 8, 9           | 1x unspecified-low (sequential)                        |
| 4    | 10, 11, 12, 13 | 4x visual-engineering in parallel                      |
| 5    | 14, 15, 16     | 1x visual-engineering (sequential), then 2x quick      |

---

## TODOs

- [ ] 1. Create SavedView SeaORM Entity

  **What to do**:
  - Create new file `src-tauri/src/entities/saved_views.rs`
  - Define `Model` struct with fields: `id` (i32 PK), `name` (String), `filter_data` (String/JSON), `is_system` (bool), `is_default` (bool), `last_used` (DateTime), `created_at` (DateTime)
  - Add standard SeaORM derives: `DeriveEntityModel`, `DeriveRelation`
  - Export from `src-tauri/src/entities/mod.rs`

  **Must NOT do**:
  - DO NOT use SeaORM's `Json` column type (use String for SQLite compatibility)
  - DO NOT add foreign key relations (filter_data is self-contained JSON)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single file creation following established entity pattern
  - **Skills**: []
    - No special skills needed - straightforward Rust entity definition

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Tasks 4, 5, 6
  - **Blocked By**: None

  **References**:
  - `src-tauri/src/entities/drafts.rs:1-18` - Entity pattern to follow (DeriveEntityModel, Model struct, Relation enum)
  - `src-tauri/src/entities/config.rs` - Simple key-value entity example
  - `src-tauri/src/entities/mod.rs` - Add new module export here
  - `docs/doc-design.md:103-111` - Schema definition from design doc

  **Acceptance Criteria**:

  ```bash
  # Agent runs:
  cd src-tauri && cargo check
  # Assert: Exit code 0, no compilation errors

  # Agent verifies:
  grep -q "saved_views" src-tauri/src/entities/mod.rs
  # Assert: Exit code 0 (module exported)
  ```

  **Commit**: YES
  - Message: `feat(db): add SavedView SeaORM entity`
  - Files: `src-tauri/src/entities/saved_views.rs`, `src-tauri/src/entities/mod.rs`
  - Pre-commit: `cd src-tauri && cargo check`

---

- [ ] 2. Add saved_views Table Creation to db.rs

  **What to do**:
  - Add table creation statement for `saved_views` entity in `init_db` function
  - Seed "Active Triage" system view if not exists (after table creation)
  - Active Triage filter_data: `{"status_exclude": true}` (exclude closed statuses)

  **Must NOT do**:
  - DO NOT modify existing table creation for `config` or `drafts`
  - DO NOT add complex migration framework (use simple "if not exists" pattern)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Small modification to existing file following established pattern
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Task 6
  - **Blocked By**: None (but needs entity from Task 1 to compile)

  **References**:
  - `src-tauri/src/services/db.rs:24-42` - Existing table creation pattern to follow
  - `src-tauri/src/entities/saved_views.rs` - Entity to create table from (after Task 1)

  **Acceptance Criteria**:

  ```bash
  # Agent runs after Task 1:
  cd src-tauri && cargo check
  # Assert: Exit code 0

  # Integration test (after full build):
  sqlite3 ~/.local/share/larch/larch.db "SELECT name FROM saved_views WHERE is_system = 1;"
  # Assert: Returns "Active Triage"
  ```

  **Commit**: NO (groups with Task 1)

---

- [ ] 3. Add SavedView TypeScript Types

  **What to do**:
  - Add `SavedView` interface to `src/lib/types.ts`
  - Fields: `id`, `name`, `filter_data` (parsed FilterObject or raw string), `is_system`, `is_default`, `last_used`, `created_at`
  - Add `SavedViewInput` type for create/update operations (without id, timestamps)

  **Must NOT do**:
  - DO NOT modify existing `FilterObject` type
  - DO NOT add runtime validation (TypeScript types only)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Small type definition addition
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Tasks 10, 11, 12, 13
  - **Blocked By**: None

  **References**:
  - `src/lib/types.ts:87-94` - Existing FilterObject type to reference
  - `docs/doc-design.md:103-111` - Schema definition

  **Acceptance Criteria**:

  ```bash
  # Agent runs:
  pnpm check
  # Assert: Exit code 0, no TypeScript errors

  # Agent verifies:
  grep -q "interface SavedView" src/lib/types.ts
  # Assert: Exit code 0
  ```

  **Commit**: YES
  - Message: `feat(types): add SavedView TypeScript interface`
  - Files: `src/lib/types.ts`
  - Pre-commit: `pnpm check`

---

- [ ] 4. Extend Repository Trait with View Methods

  **What to do**:
  - Add view methods to `Repository` trait in `src-tauri/src/repositories/mod.rs`:
    - `async fn list_views(&self) -> Result<Vec<saved_views::Model>>`
    - `async fn get_view(&self, id: i32) -> Result<Option<saved_views::Model>>`
    - `async fn create_view(&self, name: &str, filter_data: &str, is_system: bool, is_default: bool) -> Result<saved_views::Model>`
    - `async fn update_view(&self, id: i32, name: &str, filter_data: &str) -> Result<()>`
    - `async fn delete_view(&self, id: i32) -> Result<()>`
    - `async fn touch_view(&self, id: i32) -> Result<()>` (updates last_used)
    - `async fn set_default_view(&self, id: i32) -> Result<()>`
  - Add import for `saved_views` entity

  **Must NOT do**:
  - DO NOT implement methods yet (Task 5)
  - DO NOT modify existing config/draft methods

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Straightforward trait extension following existing patterns
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (after Wave 1)
  - **Blocks**: Task 5, 8
  - **Blocked By**: Task 1

  **References**:
  - `src-tauri/src/repositories/mod.rs:5-14` - Existing Repository trait pattern
  - `src-tauri/src/entities/saved_views.rs` - Entity to use in return types

  **Acceptance Criteria**:

  ```bash
  # This will fail until Task 5 implements the methods:
  cd src-tauri && cargo check 2>&1 | grep -q "not all trait items implemented"
  # Assert: Exit code 0 (expected error - trait extended but not implemented)
  ```

  **Commit**: NO (groups with Task 5)

---

- [ ] 5. Implement Repository Methods for Views

  **What to do**:
  - Implement all view methods in `impl Repository for SqliteRepository`
  - Follow existing patterns for CRUD (see `save_draft`, `get_draft`, `delete_draft`)
  - For `create_view`: auto-generate `created_at` and `last_used` as current timestamp
  - For `touch_view`: only update `last_used` field
  - For `set_default_view`: set `is_default = false` for all other views first, then set target to true
  - Add error for deleting system views in `delete_view`

  **Must NOT do**:
  - DO NOT allow deleting views where `is_system = true`
  - DO NOT add orphan sanitization here (Task 7)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: CRUD implementation following established repository pattern
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (after Task 4)
  - **Blocks**: Task 8
  - **Blocked By**: Task 4

  **References**:
  - `src-tauri/src/repositories/mod.rs:70-106` - save_draft pattern (find existing, update or insert)
  - `src-tauri/src/repositories/mod.rs:119-128` - delete_draft pattern
  - `src-tauri/src/error.rs` - Error types for system view deletion

  **Acceptance Criteria**:

  ```bash
  # Agent runs:
  cd src-tauri && cargo test saved_views
  # Assert: All tests pass

  # Add tests to src-tauri/src/repositories/tests.rs:
  # test_create_view, test_list_views, test_delete_system_view_fails
  ```

  **Commit**: YES
  - Message: `feat(repo): implement SavedView repository methods`
  - Files: `src-tauri/src/repositories/mod.rs`
  - Pre-commit: `cd src-tauri && cargo test`

---

- [ ] 6. Add Migration for Legacy selected_projects

  **What to do**:
  - In `init_db`, after creating tables and seeding Active Triage:
    - Check if `selected_projects` exists in config table
    - If exists AND no non-system default view exists:
      - Parse the JSON array of project IDs
      - Create a new view named "My Projects" with `filter_data: { project_ids: [...] }`
      - Set it as default (`is_default = true`)
    - DO NOT delete the config entry (keep for backwards compatibility)
  - Add idempotency check: only migrate once (check if "My Projects" exists)

  **Must NOT do**:
  - DO NOT delete `selected_projects` from config table
  - DO NOT fail if migration already ran (idempotent)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Database migration logic requiring careful handling
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 7)
  - **Blocks**: Task 8
  - **Blocked By**: Tasks 2, 5

  **References**:
  - `src-tauri/src/services/db.rs:9-44` - init_db function to modify
  - `src-tauri/src/repositories/mod.rs:29-36` - get_config pattern for reading selected_projects

  **Acceptance Criteria**:

  ```rust
  // Add test to src-tauri/src/services/db.rs tests:
  #[tokio::test]
  async fn test_migration_creates_default_view() {
      let db = setup_test_db_with_config("selected_projects", "[1,2,3]").await;
      init_db(&db).await.unwrap();
      let views = list_views(&db).await.unwrap();
      assert!(views.iter().any(|v| v.name == "My Projects" && v.is_default));
  }
  ```

  **Commit**: YES
  - Message: `feat(db): migrate selected_projects to default view`
  - Files: `src-tauri/src/services/db.rs`
  - Pre-commit: `cd src-tauri && cargo test migration`

---

- [ ] 7. Implement Orphan ID Sanitization

  **What to do**:
  - Create function `sanitize_view_filters` that takes valid project_ids and status_ids sets
  - For each view, parse filter_data JSON, remove orphan IDs, re-serialize
  - Call during view list/get operations (lazy sanitization)
  - Add a dedicated command `sanitize_all_views` for explicit cleanup on app startup
  - Log warnings when orphans are found and removed

  **Must NOT do**:
  - DO NOT run sanitization on every view load (expensive) - only on startup
  - DO NOT delete views with orphan IDs (just clean them)
  - DO NOT sanitize system views (Active Triage has no IDs to orphan)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Business logic requiring careful JSON manipulation
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 6)
  - **Blocks**: Task 8
  - **Blocked By**: Task 5

  **References**:
  - `src-tauri/src/commands/project_commands.rs:15-30` - FilterObject struct to parse
  - `docs/doc-design.md:114-116` - Sanitization requirement

  **Acceptance Criteria**:

  ```rust
  #[tokio::test]
  async fn test_orphan_removal() {
      let repo = setup_test_repo().await;
      repo.create_view("Test", r#"{"project_ids":[1,999]}"#, false, false).await.unwrap();

      let valid_projects = vec![1, 2, 3];
      sanitize_all_views(&repo, &valid_projects, &[]).await.unwrap();

      let view = repo.get_view(1).await.unwrap().unwrap();
      let filter: FilterObject = serde_json::from_str(&view.filter_data).unwrap();
      assert_eq!(filter.project_ids, Some(vec![1])); // 999 removed
  }
  ```

  **Commit**: YES
  - Message: `feat(repo): add orphan ID sanitization for saved views`
  - Files: `src-tauri/src/repositories/mod.rs` or new `src-tauri/src/services/view_sanitizer.rs`
  - Pre-commit: `cd src-tauri && cargo test orphan`

---

- [ ] 8. Create Tauri Commands for View Management

  **What to do**:
  - Create `src-tauri/src/commands/view_commands.rs` with commands:
    - `list_views` -> `Vec<SavedView>`
    - `get_view(id: i32)` -> `Option<SavedView>`
    - `create_view(name: String, filter_data: String)` -> `SavedView`
    - `update_view(id: i32, name: String, filter_data: String)` -> `()`
    - `delete_view(id: i32)` -> `()` (errors for system views)
    - `switch_view(id: i32)` -> `SavedView` (touches last_used, returns view)
    - `set_default_view(id: i32)` -> `()`
  - Use `tauri::State<'_, SqliteRepository>` for repository access
  - Add input validation (non-empty name, valid JSON for filter_data)

  **Must NOT do**:
  - DO NOT expose `sanitize_all_views` as user-callable command (internal only)
  - DO NOT allow empty view names

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Standard Tauri command implementation following existing patterns
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 9)
  - **Blocks**: Task 14
  - **Blocked By**: Tasks 5, 6, 7

  **References**:
  - `src-tauri/src/commands/draft_commands.rs:6-28` - Command pattern with repository state
  - `src-tauri/src/commands/project_commands.rs` - FilterObject deserialization pattern
  - `src-tauri/src/error.rs` - Result type and Error handling

  **Acceptance Criteria**:

  ```bash
  # Agent runs:
  cd src-tauri && cargo check
  # Assert: Exit code 0

  # Agent verifies commands exist:
  grep -c "#\[tauri::command\]" src-tauri/src/commands/view_commands.rs
  # Assert: Returns 7 (one per command)
  ```

  **Commit**: YES
  - Message: `feat(commands): add view management Tauri commands`
  - Files: `src-tauri/src/commands/view_commands.rs`, `src-tauri/src/commands/mod.rs`
  - Pre-commit: `cd src-tauri && cargo check`

---

- [ ] 9. Register Commands and Add Frontend Constants

  **What to do**:
  - Add view commands to `generate_handler!` in `src-tauri/src/lib.rs`
  - Add command name constants to `src/lib/commands.svelte.ts`:
    - `CMD_LIST_VIEWS`, `CMD_GET_VIEW`, `CMD_CREATE_VIEW`, `CMD_UPDATE_VIEW`, `CMD_DELETE_VIEW`, `CMD_SWITCH_VIEW`, `CMD_SET_DEFAULT_VIEW`
  - Add i18n keys to `src/lib/locales/en.json` for view-related strings

  **Must NOT do**:
  - DO NOT modify existing command registrations
  - DO NOT add commands that weren't created in Task 8

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple wiring of existing code
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (after Task 8)
  - **Blocks**: Task 14
  - **Blocked By**: Task 8

  **References**:
  - `src-tauri/src/lib.rs` - generate_handler! macro location
  - `src/lib/commands.svelte.ts` - Existing command constants pattern
  - `src/lib/locales/en.json` - i18n string location

  **Acceptance Criteria**:

  ```bash
  # Agent runs full build:
  pnpm tauri build --debug 2>&1 | tail -5
  # Assert: Build succeeds

  # Agent verifies constants:
  grep -c "CMD_.*VIEW" src/lib/commands.svelte.ts
  # Assert: Returns 7
  ```

  **Commit**: YES
  - Message: `feat(commands): register view commands and add frontend constants`
  - Files: `src-tauri/src/lib.rs`, `src/lib/commands.svelte.ts`, `src/lib/locales/en.json`
  - Pre-commit: `pnpm check`

---

- [ ] 10. Create filterUtils.ts with Deep Equality

  **What to do**:
  - Create `src/lib/utils/filterUtils.ts`
  - Implement `deepEqual(a: FilterObject, b: FilterObject): boolean`
  - Normalization rules:
    - Sort all ID arrays before comparison
    - Treat empty array `[]` and `undefined` as equal
    - Treat `false` and `undefined` for exclude flags as equal
    - Use JSON.stringify on normalized objects for comparison
  - Add `normalizeFilter(filter: FilterObject): FilterObject` helper

  **Must NOT do**:
  - DO NOT use external deep-equal library (keep dependencies minimal)
  - DO NOT compare timestamps or non-filter fields

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Small utility function with clear logic
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 11, 12, 13)
  - **Blocks**: Task 14
  - **Blocked By**: Task 3

  **References**:
  - `src/lib/types.ts:87-94` - FilterObject type definition
  - `src/lib/utils.ts` - Existing utils file pattern (or create utils directory)

  **Acceptance Criteria**:

  ```typescript
  // Add tests to src/lib/utils/filterUtils.test.ts:
  import { deepEqual } from './filterUtils';

  test('empty array equals undefined', () => {
  	expect(deepEqual({ project_ids: [] }, {})).toBe(true);
  });

  test('array order independent', () => {
  	expect(deepEqual({ status_ids: [1, 2, 3] }, { status_ids: [3, 1, 2] })).toBe(true);
  });

  test('different values not equal', () => {
  	expect(deepEqual({ project_ids: [1, 2] }, { project_ids: [1, 3] })).toBe(false);
  });
  ```

  ```bash
  # Agent runs:
  bun test filterUtils
  # Assert: All tests pass
  ```

  **Commit**: YES
  - Message: `feat(utils): add filterUtils with deep equality comparison`
  - Files: `src/lib/utils/filterUtils.ts`, `src/lib/utils/filterUtils.test.ts`
  - Pre-commit: `bun test filterUtils`

---

- [ ] 11. Build ViewSwitcher Component

  **What to do**:
  - Create `src/lib/components/dashboard/ViewSwitcher.svelte`
  - Use `DropdownMenu` from shadcn-svelte (better semantics for menu items with actions)
  - Props: `views: SavedView[]`, `currentView: SavedView | null`, `isDirty: boolean`, `onSelectView`, `onDeleteView`
  - Display current view name with dirty indicator (animated dot)
  - List all views sorted by `last_used` (most recent first)
  - Show lock icon for system views, trash icon (on hover) for user views
  - Add `data-testid` attributes for Playwright testing

  **Must NOT do**:
  - DO NOT use Select component (use DropdownMenu for action-oriented menu)
  - DO NOT include "Create New" action (that's in SaveSplitButton)
  - DO NOT allow deleting system views (hide trash icon for is_system: true)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: UI component requiring visual design decisions
  - **Skills**: [`frontend-ui-ux`]
    - Reason: Dropdown UX patterns, hover states, animations

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 10, 12, 13)
  - **Blocks**: Task 14
  - **Blocked By**: Task 3

  **References**:
  - `src/lib/components/ui/dropdown-menu/` - DropdownMenu component (ADD THIS via shadcn-svelte CLI if missing)
  - shadcn-svelte docs: `/llmstxt/shadcn-svelte_llms_txt` - Query for "dropdown menu" examples
  - `@lucide/svelte` - Icons: ChevronDown, Lock, Trash2
  - Stitch screens: "Triage Dashboard Screen" variants for visual reference

  **Acceptance Criteria**:

  ```bash
  # Agent runs:
  pnpm check
  # Assert: No TypeScript errors

  # Agent verifies component structure:
  grep -q "DropdownMenu.Root" src/lib/components/dashboard/ViewSwitcher.svelte
  grep -q "data-testid" src/lib/components/dashboard/ViewSwitcher.svelte
  # Assert: Both exit code 0
  ```

  **Commit**: YES
  - Message: `feat(ui): add ViewSwitcher dropdown component`
  - Files: `src/lib/components/dashboard/ViewSwitcher.svelte`
  - Pre-commit: `pnpm check`

---

- [ ] 12. Build SaveSplitButton Component

  **What to do**:
  - Create `src/lib/components/dashboard/SaveSplitButton.svelte`
  - **USE shadcn-svelte's `ButtonGroup` + `DropdownMenu`** (native split button pattern)
  - Props: `canSave: boolean` (user view active), `isDirty: boolean`, `isSystemView: boolean`, `onSave`, `onSaveAsNew`, `onDelete`
  - Structure:
    ```svelte
    <ButtonGroup.Root>
    	<Button onclick={onSave} disabled={!isDirty || !canSave}>
    		<Save class="mr-1 h-4 w-4" /> Save
    	</Button>
    	<DropdownMenu.Root>
    		<DropdownMenu.Trigger>
    			{#snippet child({ props })}
    				<Button {...props} variant="outline" class="!pl-2">
    					<ChevronDown />
    				</Button>
    			{/snippet}
    		</DropdownMenu.Trigger>
    		<DropdownMenu.Content>
    			<DropdownMenu.Item onclick={onSaveAsNew}>
    				<Plus /> Save as new...
    			</DropdownMenu.Item>
    			{#if !isSystemView}
    				<DropdownMenu.Separator />
    				<DropdownMenu.Item onclick={onDelete} class="text-destructive">
    					<Trash2 /> Delete view
    				</DropdownMenu.Item>
    			{/if}
    		</DropdownMenu.Content>
    	</DropdownMenu.Root>
    </ButtonGroup.Root>
    ```
  - Only visible when `isDirty || canSave`
  - "Delete view" only visible for user views (not system views like Active Triage)

  **Must NOT do**:
  - DO NOT manually compose with rounded-r-none/rounded-l-none (ButtonGroup handles this)
  - DO NOT show "Save" button for system views (only "Save as new..." via dropdown)
  - DO NOT use Popover (use DropdownMenu for menu semantics)
  - DO NOT show "Delete view" for system views (is_system: true)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: shadcn-svelte component composition
  - **Skills**: [`frontend-ui-ux`]
    - Reason: Split button UX, disabled states

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 10, 11, 13)
  - **Blocks**: Task 14
  - **Blocked By**: Task 3

  **References**:
  - shadcn-svelte docs: `/llmstxt/shadcn-svelte_llms_txt` - Query "ButtonGroup split button DropdownMenu"
  - `src/lib/components/ui/button-group/` - ADD THIS via `npx shadcn-svelte@latest add button-group` if missing
  - `src/lib/components/ui/dropdown-menu/` - ADD THIS via `npx shadcn-svelte@latest add dropdown-menu` if missing
  - `@lucide/svelte` - Icons: Save, ChevronDown, Plus, Trash2

  **Acceptance Criteria**:

  ```bash
  # Agent runs:
  pnpm check
  # Assert: No TypeScript errors

  # Agent verifies split button structure:
  grep -q "ButtonGroup.Root" src/lib/components/dashboard/SaveSplitButton.svelte
  grep -q "DropdownMenu.Root" src/lib/components/dashboard/SaveSplitButton.svelte
  # Assert: Both exit code 0
  ```

  **Commit**: YES
  - Message: `feat(ui): add SaveSplitButton with ButtonGroup + DropdownMenu`
  - Files: `src/lib/components/dashboard/SaveSplitButton.svelte`
  - Pre-commit: `pnpm check`
    grep -q "rounded-r-none" src/lib/components/dashboard/SaveSplitButton.svelte
    grep -q "rounded-l-none" src/lib/components/dashboard/SaveSplitButton.svelte

  # Assert: Both exit code 0

  ```

  **Commit**: YES
  - Message: `feat(ui): add SaveSplitButton component`
  - Files: `src/lib/components/dashboard/SaveSplitButton.svelte`
  - Pre-commit: `pnpm check`
  ```

---

- [ ] 13. Build SaveViewDialog Component

  **What to do**:
  - Create `src/lib/components/dashboard/SaveViewDialog.svelte`
  - Use Dialog component from shadcn-svelte
  - Props: `open: boolean` (bindable), `onSave: (name: string) => void`
  - Contains: title, text input for view name, Cancel and Save buttons
  - Input validation: non-empty name, trim whitespace
  - Auto-focus input when dialog opens
  - Close on save success or cancel

  **Must NOT do**:
  - DO NOT add complex validation (just non-empty check)
  - DO NOT add description field (name only for v2.1)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple modal with one input field
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 10, 11, 12)
  - **Blocks**: Task 14
  - **Blocked By**: Task 3

  **References**:
  - `src/lib/components/ui/dialog/` - Dialog component API
  - `src/lib/components/ui/input/` - Input component
  - `src/lib/components/ui/command/command-dialog.svelte` - Dialog usage pattern

  **Acceptance Criteria**:

  ```bash
  # Agent runs:
  pnpm check
  # Assert: No TypeScript errors

  # Agent verifies dialog structure:
  grep -q "Dialog.Root" src/lib/components/dashboard/SaveViewDialog.svelte
  grep -q "bind:open" src/lib/components/dashboard/SaveViewDialog.svelte
  # Assert: Both exit code 0
  ```

  **Commit**: YES
  - Message: `feat(ui): add SaveViewDialog modal component`
  - Files: `src/lib/components/dashboard/SaveViewDialog.svelte`
  - Pre-commit: `pnpm check`

---

- [ ] 14. Integrate View State into DashboardScreen

  **What to do**:
  - Add view state management to `DashboardScreen.svelte`:
    - `let views = $state<SavedView[]>([])` - all views
    - `let currentView = $state<SavedView | null>(null)` - active view
    - `let saveDialogOpen = $state(false)` - save dialog visibility
    - `let deleteDialogOpen = $state(false)` - delete confirmation visibility
    - `let viewToDelete = $state<SavedView | null>(null)` - view pending deletion
  - Add derived states:
    - `let persistedFilters = $derived(...)` - parsed filter from currentView
    - `let isDirty = $derived(...)` - using deepEqual from filterUtils
    - `let canSave = $derived(...)` - currentView exists and is not system
    - `let isSystemView = $derived(...)` - currentView?.is_system === true
  - Load views on mount, call `sanitize_all_views` on startup
  - Replace static "All Issues" title with ViewSwitcher component
  - Add SaveSplitButton to FilterBar (after "Clear All")
  - Implement view switching: update currentView, reset filters to view's filter_data
  - Implement save: update existing view's filter_data
  - Implement save as new: open dialog, create view, switch to it
  - **Implement delete flow**:
    1. User clicks "Delete view" in SaveSplitButton dropdown
    2. Set `viewToDelete = currentView` and `deleteDialogOpen = true`
    3. Show AlertDialog with: "Delete view '{name}'? This cannot be undone."
    4. On confirm: call `delete_view` command, switch to system view ("Active Triage"), show success toast
    5. On cancel: close dialog, clear viewToDelete

  **Must NOT do**:
  - DO NOT prompt before discarding unsaved changes (auto-discard confirmed)
  - DO NOT use legacy Svelte stores (use $state/$derived only)
  - DO NOT store active view ID in database (memory only per session)
  - DO NOT allow deleting system views (button hidden, but backend also validates)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Core integration requiring state orchestration and UI assembly
  - **Skills**: [`frontend-ui-ux`]
    - Reason: Complex state management, UX flow integration

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (critical path)
  - **Blocks**: Tasks 15, 16
  - **Blocked By**: Tasks 9, 10, 11, 12, 13

  **References**:
  - `src/lib/screens/DashboardScreen.svelte:19-27` - Current state declarations
  - `src/lib/screens/DashboardScreen.svelte:48-94` - loadData pattern
  - `src/lib/screens/DashboardScreen.svelte:114-117` - handleFilterChange pattern
  - `src/lib/screens/DashboardScreen.svelte:143-152` - Header area to modify
  - `src/lib/components/dashboard/FilterBar.svelte:294-304` - Location for SaveSplitButton
  - `src/lib/utils/filterUtils.ts` - deepEqual function (Task 10)
  - `src/lib/components/ui/alert-dialog/` - AlertDialog for delete confirmation (Task 18)
  - shadcn-svelte docs: `/llmstxt/shadcn-svelte_llms_txt` - Query "alert dialog confirmation"

  **Acceptance Criteria**:

  ```bash
  # Agent runs:
  pnpm check
  # Assert: No TypeScript errors

  # Agent verifies integration:
  grep -q "ViewSwitcher" src/lib/screens/DashboardScreen.svelte
  grep -q "SaveSplitButton" src/lib/screens/DashboardScreen.svelte
  grep -q "isDirty" src/lib/screens/DashboardScreen.svelte
  grep -q "AlertDialog" src/lib/screens/DashboardScreen.svelte
  grep -q "deleteDialogOpen" src/lib/screens/DashboardScreen.svelte
  # Assert: All exit code 0

  # Manual verification via dev server:
  # 1. Start app: pnpm tauri dev
  # 2. Verify ViewSwitcher shows "Active Triage"
  # 3. Change filter → dirty indicator appears
  # 4. Click "Save as new..." → dialog opens → save → new view in switcher
  # 5. With user view active, click "Delete view" → confirmation dialog appears
  # 6. Confirm delete → view deleted → switches to Active Triage → success toast
  ```

  **Commit**: YES
  - Message: `feat(dashboard): integrate saved views with state management`
  - Files: `src/lib/screens/DashboardScreen.svelte`, `src/lib/components/dashboard/FilterBar.svelte`
  - Pre-commit: `pnpm check`

---

- [ ] 15. Add Keyboard Shortcuts

  **What to do**:
  - Add keydown event handler in `DashboardScreen.svelte`
  - `Cmd+S` (Mac) / `Ctrl+S` (Windows/Linux): Save current view (if editable and dirty)
  - `Cmd+Shift+S` / `Ctrl+Shift+S`: Open "Save as new..." dialog
  - Use `$effect` for event listener lifecycle
  - Prevent default browser save behavior
  - Only active when Dashboard is focused (not during modal dialogs)

  **Must NOT do**:
  - DO NOT make shortcuts global (Dashboard-only)
  - DO NOT save if current view is system view (just open Save As dialog)
  - DO NOT trigger shortcuts when input/textarea is focused

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Small feature addition with clear implementation
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 5 (with Task 16)
  - **Blocks**: None
  - **Blocked By**: Task 14

  **References**:
  - `src/lib/screens/DashboardScreen.svelte` - Component to add handler to
  - Svelte 5 `$effect` documentation for event listener cleanup

  **Acceptance Criteria**:

  ```bash
  # Agent verifies shortcut handler:
  grep -q "metaKey\|ctrlKey" src/lib/screens/DashboardScreen.svelte
  grep -q "e.key === 's'" src/lib/screens/DashboardScreen.svelte
  # Assert: Both exit code 0

  # Manual verification:
  # 1. Open app, modify filter → Cmd+S → view saves (toast appears)
  # 2. On system view, Cmd+S → Save As dialog opens
  # 3. Cmd+Shift+S → Save As dialog opens
  ```

  **Commit**: YES
  - Message: `feat(ux): add Cmd+S keyboard shortcut for saving views`
  - Files: `src/lib/screens/DashboardScreen.svelte`
  - Pre-commit: `pnpm check`

---

- [ ] 16. Add Animated Dirty Indicator + Polish

  **What to do**:
  - In ViewSwitcher, add animated dot next to view name when `isDirty`
  - CSS animation: subtle pulse or fade-in effect
  - Add toast notifications for save/delete actions
  - Add loading states during async operations
  - Ensure all user-facing strings use `$t()` from svelte-i18n
  - Add any missing i18n keys to locale files

  **Must NOT do**:
  - DO NOT add complex animations (keep subtle)
  - DO NOT add sounds or other distracting feedback

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: CSS polish and toast additions
  - **Skills**: [`frontend-ui-ux`]
    - Reason: Animation timing, visual feedback patterns

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 5 (with Task 15)
  - **Blocks**: None
  - **Blocked By**: Task 14

  **References**:
  - `src/lib/components/dashboard/ViewSwitcher.svelte` - Component to enhance
  - `svelte-sonner` - Toast library (already in use)
  - `src/lib/locales/en.json` - i18n strings

  **Acceptance Criteria**:

  ```bash
  # Agent verifies animation:
  grep -q "animate-\|@keyframes\|transition" src/lib/components/dashboard/ViewSwitcher.svelte
  # Assert: Exit code 0

  # Agent verifies toasts:
  grep -q "toast.success\|toast.error" src/lib/screens/DashboardScreen.svelte
  # Assert: Exit code 0

  # Agent verifies i18n:
  grep -q "views\." src/lib/locales/en.json
  # Assert: Exit code 0
  ```

  **Commit**: YES
  - Message: `feat(ux): add animated dirty indicator and toast feedback`
  - Files: `src/lib/components/dashboard/ViewSwitcher.svelte`, `src/lib/screens/DashboardScreen.svelte`, `src/lib/locales/en.json`
  - Pre-commit: `pnpm check`

---

- [ ] 17. Update AGENTS.md with shadcn-svelte Documentation Reference

  **What to do**:
  - Add the shadcn-svelte llms.txt reference to `AGENTS.md` in Section 10 (Libraries Reference)
  - Insert after the `| UI Components | shadcn-svelte, bits-ui | latest |` row:
    ```markdown
    **shadcn-svelte Documentation**: For component API reference and examples, use Context7 with library ID `/llmstxt/shadcn-svelte_llms_txt` or visit https://shadcn-svelte.com/llms.txt
    ```
  - This enables future AI agents to always have access to current component patterns

  **Must NOT do**:
  - DO NOT modify other sections of AGENTS.md
  - DO NOT add other library references (shadcn-svelte only for now)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single-line documentation addition
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Can run anytime (no code dependencies)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `AGENTS.md:309-324` - Section 10: Libraries Reference

  **Acceptance Criteria**:

  ```bash
  # Agent verifies reference added:
  grep -q "llmstxt/shadcn-svelte" AGENTS.md
  # Assert: Exit code 0

  grep -q "shadcn-svelte.com/llms.txt" AGENTS.md
  # Assert: Exit code 0
  ```

  **Commit**: YES
  - Message: `docs: add shadcn-svelte llms.txt reference to AGENTS.md`
  - Files: `AGENTS.md`
  - Pre-commit: None

---

- [ ] 18. Add Missing shadcn-svelte Components (ButtonGroup, DropdownMenu, AlertDialog)

  **What to do**:
  - Check if `button-group` component exists in `src/lib/components/ui/`
  - Check if `dropdown-menu` component exists in `src/lib/components/ui/`
  - Check if `alert-dialog` component exists in `src/lib/components/ui/`
  - If missing, add via shadcn-svelte CLI:
    ```bash
    npx shadcn-svelte@latest add button-group
    npx shadcn-svelte@latest add dropdown-menu
    npx shadcn-svelte@latest add alert-dialog
    ```
  - Verify components are importable after installation
  - AlertDialog is used for delete confirmation in Task 14

  **Must NOT do**:
  - DO NOT manually create these components (use CLI)
  - DO NOT modify existing components

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: CLI command execution
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (before Tasks 11, 12)
  - **Blocks**: Tasks 11, 12
  - **Blocked By**: None

  **References**:
  - `src/lib/components/ui/` - UI component directory
  - shadcn-svelte CLI docs

  **Acceptance Criteria**:

  ```bash
  # Agent verifies components exist:
  ls src/lib/components/ui/button-group/index.ts
  ls src/lib/components/ui/dropdown-menu/index.ts
  ls src/lib/components/ui/alert-dialog/index.ts
  # Assert: All exit code 0

  pnpm check
  # Assert: Exit code 0
  ```

  **Commit**: YES
  - Message: `feat(ui): add button-group, dropdown-menu, and alert-dialog shadcn-svelte components`
  - Files: `src/lib/components/ui/button-group/*`, `src/lib/components/ui/dropdown-menu/*`, `src/lib/components/ui/alert-dialog/*`
  - Pre-commit: `pnpm check`

---

## Commit Strategy

| After Task | Message                                                              | Files                                                                  | Verification |
| ---------- | -------------------------------------------------------------------- | ---------------------------------------------------------------------- | ------------ |
| 1          | `feat(db): add SavedView SeaORM entity`                              | entities/saved_views.rs, mod.rs                                        | cargo check  |
| 3          | `feat(types): add SavedView TypeScript interface`                    | types.ts                                                               | pnpm check   |
| 5          | `feat(repo): implement SavedView repository methods`                 | repositories/mod.rs                                                    | cargo test   |
| 6          | `feat(db): migrate selected_projects to default view`                | services/db.rs                                                         | cargo test   |
| 7          | `feat(repo): add orphan ID sanitization for saved views`             | repositories/mod.rs                                                    | cargo test   |
| 8          | `feat(commands): add view management Tauri commands`                 | commands/view_commands.rs, mod.rs                                      | cargo check  |
| 9          | `feat(commands): register view commands and add frontend constants`  | lib.rs, commands.svelte.ts, en.json                                    | pnpm check   |
| 10         | `feat(utils): add filterUtils with deep equality comparison`         | filterUtils.ts, filterUtils.test.ts                                    | bun test     |
| 11         | `feat(ui): add ViewSwitcher dropdown component`                      | ViewSwitcher.svelte                                                    | pnpm check   |
| 12         | `feat(ui): add SaveSplitButton with ButtonGroup + DropdownMenu`      | SaveSplitButton.svelte                                                 | pnpm check   |
| 13         | `feat(ui): add SaveViewDialog modal component`                       | SaveViewDialog.svelte                                                  | pnpm check   |
| 14         | `feat(dashboard): integrate saved views with state management`       | DashboardScreen.svelte, FilterBar.svelte                               | pnpm check   |
| 15         | `feat(ux): add Cmd+S keyboard shortcut for saving views`             | DashboardScreen.svelte                                                 | pnpm check   |
| 16         | `feat(ux): add animated dirty indicator and toast feedback`          | ViewSwitcher.svelte, DashboardScreen.svelte, en.json                   | pnpm check   |
| 17         | `docs: add shadcn-svelte llms.txt reference to AGENTS.md`            | AGENTS.md                                                              | None         |
| 18         | `feat(ui): add button-group, dropdown-menu, and alert-dialog shadcn` | src/lib/components/ui/button-group/_, dropdown-menu/_, alert-dialog/\_ | pnpm check   |

---

## Success Criteria

### Verification Commands

```bash
# Run all Rust tests
cd src-tauri && cargo test
# Expected: All tests pass, including new saved_views tests

# Run all frontend checks
pnpm check
# Expected: No TypeScript or Svelte errors

# Run frontend tests
bun test
# Expected: filterUtils tests pass

# Full build
pnpm tauri build --debug
# Expected: Build succeeds

# Database verification (after running app once)
sqlite3 ~/.local/share/larch/larch.db "SELECT name, is_system FROM saved_views;"
# Expected: At least "Active Triage" with is_system=1
```

### Final Checklist

- [ ] All "Must Have" features present
- [ ] All "Must NOT Have" guardrails respected
- [ ] All tests pass (`just test`)
- [ ] All checks pass (`just check`)
- [ ] Views persist across app restarts
- [ ] Active Triage cannot be deleted
- [ ] User views can be deleted with confirmation dialog
- [ ] After delete, switches to Active Triage
- [ ] Dirty indicator appears when filters change
- [ ] Keyboard shortcuts work (Cmd+S, Cmd+Shift+S)
- [ ] All strings use i18n ($t)
- [ ] Orphan IDs are sanitized on startup
