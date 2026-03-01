### Patterns and Conventions

- Added `priority`, `severity`, and `type_` fields to `IssueDto` to match `IssueDetailDto`.
- Used `#[serde(rename = "type")]` for `type_` to avoid Rust keyword collision.
- Used `#[serde(default)]` for resilient deserialization as API list responses might not always include these fields or they might be null/missing.

### Successful Approaches

- Sequential verification with `cargo check -p taiga-client` from the `src-tauri` directory (as the root doesn't have a workspace Cargo.toml).

## 2026-02-28

- The 'Edit' tool might report success but fail to actually modify the file. Always verify with 'Read'.
- 'sed' is a reliable alternative for simple insertions when 'Edit' is flaky.
- Extended `Issue` struct and its `From<IssueDto>` implementation to include `priority`, `severity`, and `issue_type`.
- All manual `IssueDto` initializations in tests must now include the new fields (`priority`, `severity`, `type_`) to satisfy the compiler.
- The domain field is named `issue_type` to avoid conflict with the Rust keyword `type`, while the DTO uses `type_`.

## Extended FilterObject and Query Params

- Added , , , , , to struct.
- Updated to construct Taiga API query params for these new filter dimensions.
- Verified with existing tests.

## Extended FilterObject and Query Params

- Added priority_ids, priority_exclude, severity_ids, severity_exclude, type_ids, type_exclude to FilterObject struct.
- Updated get_aggregated_issues to construct Taiga API query params for these new filter dimensions.
- Verified with existing tests.

## Pattern: Extending FilterObject

- When adding new filter fields to , ensure both the struct definition and the query param building logic in are updated.
- Use for ID fields and for exclude fields.
- For the Taiga API, ID fields should be comma-separated strings.
- Be careful with tool if multiple calls are made rapidly; or with can be more reliable for full-file overwrites if fails to reflect changes.

## Pattern: Extending FilterObject

- When adding new filter fields to FilterObject, ensure both the struct definition and the query param building logic in get_aggregated_issues are updated.
- Use Option<Vec<i64>> for ID fields and Option<bool> for exclude fields.
- For the Taiga API, ID fields should be comma-separated strings.
- Be careful with edit tool if multiple calls are made rapidly; write or bash with heredoc can be more reliable for full-file overwrites if edit fails to reflect changes.

### Task 4: Extend view_sanitizer FilterData

- When adding new fields to a struct that is used for serialization/deserialization (like `FilterData`), ensure that all new fields are optional if you want to maintain backward compatibility with existing data.
- Using `#[serde(skip_serializing_if = "Option::is_none")]` is a good practice to keep the serialized JSON clean.
- If the `edit` tool consistently fails to apply changes despite reporting success, using the `write` tool with the full file content is a reliable fallback for smaller files.
- Frontend types in `src/lib/types.ts` must mirror backend domain models to ensure consistency in Tauri command payloads.
- `FilterObject` uses `snake_case` for fields, following the backend's naming convention for filter data.
- `Issue` interface includes `issue_type` instead of just `type` to avoid ambiguity and match backend field naming.

## Severity Filter Dropdown

- Created `SeverityFilterDropdown.svelte` closely following `StatusFilterDropdown.svelte`.
- Replaced `IssueStatus` with `Severity` type, and data source is `meta.severities`.
- Retained the split between Unified and Project Specific elements.
- Replaced `Filter` lucide icon with `AlertTriangle`.
- Updated translations keys mapping (statusFilter -> severityFilter, searchStatuses -> searchSeverities, etc.).
- Explicitly rendered `item.severity.color` using the style attribute for the color dot, falling back to `#9ca3af` (gray-400 equivalent), rather than overriding it forcefully in the derived data structure like was done in `StatusFilterDropdown.svelte`.
- For custom filter dropdowns (like PriorityFilterDropdown), we should maintain consistency with the existing patterns (e.g., StatusFilterDropdown) by using the same UI structure: Popover.Content, Checkbox with indeterminate state, Search input, and grouping entities into Unified and Project-Specific lists.
- Entities with color attributes (e.g., Priority) should prioritize using their metadata color directly in the UI instead of hardcoded placeholder colors, ensuring a seamless visual integration.
  \n- Created TypeFilterDropdown.svelte by cloning the pattern from StatusFilterDropdown.svelte, replacing 'status' with 'issue_type', and adapting UI labels/icons.
- Task 10: Wired new Priority, Severity, and Type filters into FilterBar.svelte.
- Handled UI dropdowns mirroring existing Status and Assignee implementations.
- Noticed Svelte 5 linter prefers SvelteSet instead of built-in Set even for temporary variables that don't need reactivity, patched this.
- AddFilterDropdown.svelte temporarily patched to support the new props to satisfy Svelte typescript compiler.
  Task 11 completed: Added Priority, Severity, and Type options to AddFilterDropdown.svelte menu.
- In Svelte 5, `{@const}` tags must be placed as an immediate child of a block (`{#each}`, `{#if}`, etc.). They cannot be nested inside standard HTML elements like `<div>` or `<td>`. Place them at the top of the block where they belong.
- `Record<number, ProjectMetadata>` is passed to resolve missing priorities/severities/types that Taiga's Issue API skips on the main issue model. Using optional chaining like `metadata[issue.project].priorities?.find(...)` handles edge cases safely when metadata is loading.
### Task 13: Pass metadata to IssueTable in DashboardScreen.svelte
- Successfully wired the 'metadata' prop from DashboardScreen to IssueTable.
- This allows IssueTable to resolve Priority, Severity, and Type names/colors correctly.
- Used 'write' tool after 'edit' tool failed to apply changes multiple times (likely due to indentation or multiline range issues).
- When the 'edit' tool reports success but the file content doesn't seem to change, verify with 'cat' or 'read' again. If it still fails, 'write' is a reliable fallback for small files.
