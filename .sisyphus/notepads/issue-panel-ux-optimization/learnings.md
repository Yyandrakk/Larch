## Add subject field to PatchIssueRequest

- Added `subject: Option<String>` to `PatchIssueRequest` struct in `crates/taiga-client/src/models.rs`.
- Used `#[serde(skip_serializing_if = "Option::is_none")]` to maintain consistency with other optional fields.
- This change enables editing the issue title (subject) via the Taiga API PATCH endpoint.
- Verified that the `taiga-client` package compiles successfully after the change.

## Task 2: Add history endpoints to taiga-client

**Completed**: Added `edit_issue_comment` and `delete_issue_comment` methods to `TaigaClient`

**Implementation details**:

- Edit endpoint: PATCH `/api/v1/history/issue/{issue_id}/{comment_id}` with JSON body `{"comment": "new text"}`
- Delete endpoint: POST `/api/v1/history/issue/{issue_id}/{comment_id}/delete_comment`
- Both methods return `Result<(), TaigaClientError>` following the pattern of `delete_issue_attachment`
- Error handling follows existing patterns: NOT_FOUND, UNAUTHORIZED, and generic AuthFailed
- Accept NO_CONTENT status for delete operation (similar to attachment deletion)

**Pattern observations**:

- All public API methods use docstring format: `/// Description` + `/// HTTP_METHOD /endpoint/path`
- DELETE-like operations check for both `is_success()` and `StatusCode::NO_CONTENT`
- Mutation endpoints that don't return data use `Result<(), TaigaClientError>` signature
- `comment_id` is `String` type (not `i64`), consistent with Taiga API history entry IDs

**Verification**: `cargo check -p taiga-client` passes ✓

## Task 3: Create Tauri commands for title and comment management

**Completed**: Fixed compilation errors and added three new Tauri commands

**Implementation details**:

1. **Fix compilation errors**: Added `subject: None` to all 8 `PatchIssueRequest` initializers in `issue_commands.rs`
   - This was required after Task 1 added the `subject` field to the struct
   - All existing commands (status, comment, description, assignee, priority, severity, type, tags) were updated

2. **New commands**:
   - `change_issue_subject`: Uses `PatchIssueRequest` with `subject: Some(...)` field
   - `edit_issue_comment`: Calls `client.edit_issue_comment()` directly (no version field needed)
   - `delete_issue_comment`: Calls `client.delete_issue_comment()` directly

3. **Architectural patterns followed**:
   - All commands follow the async retry pattern: inner `fetch()` function + outer match for Unauthorized
   - Token refresh on 401: `token_refresh::refresh_token(&client).await?`
   - Docstrings match existing style: `/// Description` + `/// Uses optimistic locking...` where applicable
   - Return types: `Result<IssueDetail>` for mutations that return updated issue, `Result<()>` for operations with no response

4. **Registration**:
   - Commands added to `tauri::generate_handler![]` in `src-tauri/src/lib.rs`
   - Constants exported in `src/lib/commands.svelte.ts`:
     - `CMD_CHANGE_ISSUE_SUBJECT = 'change_issue_subject'`
     - `CMD_EDIT_ISSUE_COMMENT = 'edit_issue_comment'`
     - `CMD_DELETE_ISSUE_COMMENT = 'delete_issue_comment'`

**Key differences from other commands**:

- Edit/delete comment DO NOT use `PatchIssueRequest` (they use dedicated history endpoints)
- Edit/delete comment DO NOT use optimistic locking (no version parameter)
- Change subject DOES use `PatchIssueRequest` and optimistic locking (consistent with description editing)

**Verification**: `just check-rust` passes ✓

**Commit**: `feat(commands): add change_issue_subject and comment management commands`

- Implemented title editing in IssueDetailSheet.svelte using conditional rendering and Svelte 5 state.
- Used `CMD_CHANGE_ISSUE_SUBJECT` command.
- Verified with `pnpm check`.

## Comment Cards Redesign & Edit/Delete Functionality

- **Pattern**: Used `HistoryEntry` type for comments, leveraging `user_id` to determine ownership.
- **UI**: Added conditional styling `bg-accent/30` for own comments to distinguish them visually.
- **Interaction**: Implemented inline editing using a `textarea` (instead of full `MarkdownEditor` to keep it lightweight for small edits) and `AlertDialog` for delete confirmation.
- **Optimistic Updates**: Implemented optimistic UI updates for both edit and delete actions to ensure responsiveness, updating the local `comments` array immediately while the API call is in flight.
- **Gotcha**: `CMD_EDIT_ISSUE_COMMENT` and `CMD_DELETE_ISSUE_COMMENT` were present in `commands.svelte.ts` but LSP sometimes reported them as missing. Verified existence manually.
- **Dependency**: Needed `@lucide/svelte` for icons.
- **Verification**: Verified via `pnpm check`. E2E tests are missing in the repo, so visual/manual verification would be next step if environment allowed.
