# Plan: Add Priority, Severity & Type Filters and Columns to Dashboard

## Objective

Extend the Larch dashboard with three new filter dimensions (Priority, Severity, Type) and two new table columns (combined Priority/Severity column + Type column), enabling developers to perform efficient triage across multiple Taiga projects.

## Context

- **Taiga API confirmed**: `priority`/`exclude_priority`, `severity`/`exclude_severity`, `type`/`exclude_type` — all by numeric ID, same pattern as existing `status`/`exclude_status`
- **API List response**: Returns `priority`, `severity`, `type` as **IDs only** (no `_extra_info`). The frontend must resolve names/colors from `ProjectMetadata` at render time.
- **Metadata already loaded**: `get_project_metadata` already fetches priorities, severities, issue_types per project — no new API endpoints needed.
- **UX Decision**: Priority + Severity combined into one "P / S" column; Type as a separate column. Total: 8 columns.

## Architecture Overview

```
Layer 1: taiga-client (IssueDto)     — Add priority/severity/type ID fields
Layer 2: domain (Issue)              — Add fields + From<IssueDto> mapping
Layer 3: commands (FilterObject)     — Add filter fields + query param building
Layer 4: services (view_sanitizer)   — Add new ID fields to FilterData + sanitize_views
Layer 5: frontend types              — Extend TS Issue + FilterObject interfaces
Layer 6: frontend utils              — Update filterUtils.ts normalization
Layer 7: frontend filters            — 3 new dropdown components + FilterBar + AddFilterDropdown
Layer 8: frontend table              — 2 new columns in IssueTable
Layer 9: i18n                        — Translation keys
Layer 10: tests                      — Rust unit tests for all changed layers
```

## Scope

**IN:**

- Priority, Severity, Type as filterable dimensions with include/exclude toggle
- Combined P/S column and separate Type column in IssueTable
- Filter dropdowns following existing StatusFilterDropdown pattern (unified + project-specific grouping)
- Updated filterUtils for dirty-state detection with new fields
- Updated view_sanitizer FilterData for new fields
- i18n keys for all new UI strings
- Rust unit tests for IssueDto conversion, query param building

**OUT:**

- Column sorting (future feature)
- Column visibility toggle / column picker
- Drag-and-drop column reordering
- Any changes to IssueDetail panel (already has P/S/T)

## Implementation Tasks

### Task 1: Extend `IssueDto` in taiga-client

**File:** `crates/taiga-client/src/models.rs`
**What:** Add `priority`, `severity`, and `type_` fields to `IssueDto` struct.

The Taiga API list endpoint returns these as numeric IDs (confirmed in API docs section 48.45). No `_extra_info` objects are returned for list endpoint — only the GET detail endpoint includes those.

**Changes:**

```rust
// In IssueDto struct (after line 132, after assigned_to_extra_info):
#[serde(default)]
pub priority: Option<i64>,
#[serde(default)]
pub severity: Option<i64>,
#[serde(rename = "type", default)]
pub type_: Option<i64>,
```

**Key details:**

- Use `#[serde(rename = "type")]` because `type` is a Rust keyword (same pattern as `IssueDetailDto` line 192-193)
- Use `#[serde(default)]` to make deserialization resilient if field is missing
- All `Option<i64>` — the API always sends them but defensive coding is good practice

**QA:**

- `cargo check -p taiga-client` must pass
- Verify `#[serde(rename = "type")]` matches `IssueDetailDto` pattern at line 192

### Task 2: Extend Domain `Issue` struct and `From<IssueDto>`

**File:** `src-tauri/src/domain/issue.rs`
**What:** Add priority, severity, type fields (ID only — frontend resolves names from metadata).

**Changes to `Issue` struct** (after `modified_date` field, line 16):

```rust
pub priority: Option<i64>,
pub severity: Option<i64>,
pub issue_type: Option<i64>,
```

**Changes to `From<IssueDto>` impl** (add to the Self block, after `modified_date` mapping, line 38):

```rust
priority: dto.priority,
severity: dto.severity,
issue_type: dto.type_,
```

**Note:** Use `issue_type` (not `type_`) for the domain field name — clearer semantics, avoids keyword confusion.

**Update ALL existing tests in this file** (2 tests at lines 49-110):

- `test_issue_conversion_from_dto` (line 49): Add `priority: Some(3), severity: Some(2), type_: Some(1)` to `IssueDto` literal, assert `issue.priority == Some(3)`, etc.
- `test_issue_conversion_minimal_dto` (line 89): Add `priority: None, severity: None, type_: None` to `IssueDto` literal, assert all None.

**QA:**

- `cd src-tauri && cargo test domain::issue::tests` — all pass
- `cargo clippy -- -D warnings` — no warnings

### Task 3: Extend `FilterObject` and Query Param Building in commands

**File:** `src-tauri/src/commands/project_commands.rs`
**What:** Add priority/severity/type filter fields to `FilterObject` and build corresponding API query params.

**Changes to `FilterObject` struct** (after `project_exclude`, line 17):

```rust
pub priority_ids: Option<Vec<i64>>,
pub priority_exclude: Option<bool>,
pub severity_ids: Option<Vec<i64>>,
pub severity_exclude: Option<bool>,
pub type_ids: Option<Vec<i64>>,
pub type_exclude: Option<bool>,
```

**Changes to `get_aggregated_issues` function** — Add 3 new query param blocks after the `assignee_ids` block (after line 127), following the **exact same pattern** as status (lines 91-105):

```rust
if let Some(priority_ids) = filters.priority_ids {
    if !priority_ids.is_empty() {
        let key = if filters.priority_exclude.unwrap_or(false) {
            "exclude_priority"
        } else {
            "priority"
        };
        let val = priority_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        query_params.push((key.to_string(), val));
    }
}

if let Some(severity_ids) = filters.severity_ids {
    if !severity_ids.is_empty() {
        let key = if filters.severity_exclude.unwrap_or(false) {
            "exclude_severity"
        } else {
            "severity"
        };
        let val = severity_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        query_params.push((key.to_string(), val));
    }
}

if let Some(type_ids) = filters.type_ids {
    if !type_ids.is_empty() {
        let key = if filters.type_exclude.unwrap_or(false) {
            "exclude_type"
        } else {
            "type"
        };
        let val = type_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        query_params.push((key.to_string(), val));
    }
}
```

**Update existing tests** (lines 386-470): The `create_issue_dto` helper at line 389 must add the new fields (`priority: None, severity: None, type_: None`) to the `IssueDto` literal. No new test logic needed — existing sort tests just need valid struct literals.

**QA:**

- `cd src-tauri && cargo test commands::project_commands::tests` — all pass
- Verify API query param keys match Taiga docs exactly: `priority`, `exclude_priority`, `severity`, `exclude_severity`, `type`, `exclude_type`

### Task 4: Extend `view_sanitizer` FilterData

**File:** `src-tauri/src/services/view_sanitizer.rs`
**What:** Add new filter ID fields to `FilterData` struct so saved views with these filters serialize/deserialize correctly. Optionally add sanitization for the new ID types.

**Changes to `FilterData` struct** (after `project_exclude`, line 19):

```rust
#[serde(skip_serializing_if = "Option::is_none")]
priority_ids: Option<Vec<i64>>,
#[serde(skip_serializing_if = "Option::is_none")]
priority_exclude: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
severity_ids: Option<Vec<i64>>,
#[serde(skip_serializing_if = "Option::is_none")]
severity_exclude: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
type_ids: Option<Vec<i64>>,
#[serde(skip_serializing_if = "Option::is_none")]
type_exclude: Option<bool>,
```

**Important:** The `sanitize_all_views` function does NOT need to sanitize these new IDs right now. Priority/severity/type IDs are project-scoped and don't become "orphan" in the same way status IDs can when project membership changes. The sanitizer only needs the `FilterData` struct to have these fields so it can **round-trip** (deserialize → serialize) without losing data. If the struct doesn't have these fields, `serde_json::from_str` would silently drop them during sanitization of other fields, corrupting saved views.

**QA:**

- `cd src-tauri && cargo test services::view_sanitizer::tests` — all existing tests pass
- Verify that a saved view JSON like `{"priority_ids": [1], "status_ids": [10]}` round-trips correctly through FilterData serde

### Task 5: Extend TypeScript Types

**File:** `src/lib/types.ts`
**What:** Add new fields to `Issue` interface and `FilterObject` interface.

**Changes to `Issue` interface** (after `modified_date`, line 13):

```typescript
priority?: number;
severity?: number;
issue_type?: number;
```

**Changes to `FilterObject` interface** (after `assignee_exclude`, line 93):

```typescript
priority_ids?: number[];
priority_exclude?: boolean;
severity_ids?: number[];
severity_exclude?: boolean;
type_ids?: number[];
type_exclude?: boolean;
```

**QA:**

- `pnpm check` passes (no TS errors)
- Verify field names match Rust `FilterObject` field names exactly (serde will use snake_case)

### Task 6: Update `filterUtils.ts`

**File:** `src/lib/utils/filterUtils.ts`
**What:** Add normalization for new filter fields so dirty-state detection works correctly for saved views.

**Changes to `normalizeFilter`** — Add 6 new blocks after `assignee_exclude` processing (after line 31), following exact same pattern:

```typescript
const priority_ids = processArray(filter.priority_ids);
if (priority_ids) normalized.priority_ids = priority_ids;

const priority_exclude = processBoolean(filter.priority_exclude);
if (priority_exclude) normalized.priority_exclude = priority_exclude;

const severity_ids = processArray(filter.severity_ids);
if (severity_ids) normalized.severity_ids = severity_ids;

const severity_exclude = processBoolean(filter.severity_exclude);
if (severity_exclude) normalized.severity_exclude = severity_exclude;

const type_ids = processArray(filter.type_ids);
if (type_ids) normalized.type_ids = type_ids;

const type_exclude = processBoolean(filter.type_exclude);
if (type_exclude) normalized.type_exclude = type_exclude;
```

**Critical:** Without this change, the `isDirty` derived state in `DashboardScreen.svelte` (line 136) would fail to detect changes to new filter fields, causing the "unsaved changes" indicator to not appear.

**QA:**

- `pnpm check` passes
- Manually verify: adding a priority filter → isDirty shows true → saving view → isDirty shows false

### Task 7: Create `PriorityFilterDropdown.svelte`

**File:** `src/lib/components/dashboard/filters/PriorityFilterDropdown.svelte` (NEW)
**What:** Filter dropdown for priorities, following the **exact same pattern** as `StatusFilterDropdown.svelte`.

**Implementation approach:**

- Copy structure from `StatusFilterDropdown.svelte` — it's the closest analogue (items from metadata with color, include/exclude toggle, search, select all, unified/project-specific grouping)
- Props: `metadata`, `projects` (for project-specific grouping), `selectedIds`, `isExclude`, `open`, `onApply`, `customAnchor`
- Group priorities by name across projects (same unified/project-specific pattern as status)
- Use `metadata[pid].priorities` to get available priorities per project
- Show color dot from priority color, name text
- `onApply(ids: number[], exclude: boolean)` callback

**Key differences from StatusFilterDropdown:**

- Header: use `$t('filters.priorityFilter')` with `Flag` icon from lucide
- Search placeholder: `$t('filters.searchPriorities')`
- Data source: `meta.priorities` instead of `meta.statuses`
- No `is_closed` concept — simpler grouping

**QA:**

- Component renders without errors
- Include/exclude toggle works
- Search filters the list
- Select All / Clear work
- Apply sends correct IDs

### Task 8: Create `SeverityFilterDropdown.svelte`

**File:** `src/lib/components/dashboard/filters/SeverityFilterDropdown.svelte` (NEW)
**What:** Filter dropdown for severities, same pattern as PriorityFilterDropdown.

**Same structure as Task 7**, but:

- Header: use `$t('filters.severityFilter')` with `AlertTriangle` icon from lucide
- Search placeholder: `$t('filters.searchSeverities')`
- Data source: `meta.severities`

**QA:** Same as Task 7.

### Task 9: Create `TypeFilterDropdown.svelte`

**File:** `src/lib/components/dashboard/filters/TypeFilterDropdown.svelte` (NEW)
**What:** Filter dropdown for issue types, same pattern as PriorityFilterDropdown.

**Same structure as Task 7**, but:

- Header: use `$t('filters.typeFilter')` with `Tag` icon from lucide
- Search placeholder: `$t('filters.searchTypes')`
- Data source: `meta.issue_types`

**QA:** Same as Task 7.

### Task 10: Update `FilterBar.svelte` — Wire New Filters

**File:** `src/lib/components/dashboard/FilterBar.svelte`
**What:** Register the 3 new filter dropdowns and update all hardcoded filter logic.

**Changes:**

1. **Imports** (after line 12): Add new imports:

```typescript
import PriorityFilterContent from './filters/PriorityFilterDropdown.svelte';
import SeverityFilterContent from './filters/SeverityFilterDropdown.svelte';
import TypeFilterContent from './filters/TypeFilterDropdown.svelte';
```

And icons (add to line 2 imports):

```typescript
import { X, Folder, CircleDot, User, Flag, AlertTriangle, Tag } from '@lucide/svelte';
```

2. **State variables** (after `assigneeDropdownOpen`, line 45): Add:

```typescript
let priorityDropdownOpen = $state(false);
let severityDropdownOpen = $state(false);
let typeDropdownOpen = $state(false);
```

3. **`openedFromAddFilter` type** (line 51): Extend union type:

```typescript
let openedFromAddFilter = $state<
	'project' | 'status' | 'assignee' | 'priority' | 'severity' | 'type' | null
>(null);
```

4. **Has-filter derivations** (after `hasAssigneeFilter`, line 57): Add:

```typescript
let hasPriorityFilter = $derived(filters.priority_ids && filters.priority_ids.length > 0);
let hasSeverityFilter = $derived(filters.severity_ids && filters.severity_ids.length > 0);
let hasTypeFilter = $derived(filters.type_ids && filters.type_ids.length > 0);
```

5. **`activeFilterCount`** (line 59-61): Update to include new filters:

```typescript
let activeFilterCount = $derived(
	(hasProjectFilter ? 1 : 0) +
		(hasStatusFilter ? 1 : 0) +
		(hasAssigneeFilter ? 1 : 0) +
		(hasPriorityFilter ? 1 : 0) +
		(hasSeverityFilter ? 1 : 0) +
		(hasTypeFilter ? 1 : 0)
);
```

6. **Chip value functions**: Add `getPriorityChipValue()`, `getSeverityChipValue()`, `getTypeChipValue()` — follow `getStatusChipValue()` pattern but use `meta.priorities` / `meta.severities` / `meta.issue_types`.

7. **Handle apply/remove functions**: Add 3 pairs following exact pattern of `handleStatusApply`/`removeStatusFilter` (lines 129-161):
   - `handlePriorityApply(ids, exclude)` / `removePriorityFilter()`
   - `handleSeverityApply(ids, exclude)` / `removeSeverityFilter()`
   - `handleTypeApply(ids, exclude)` / `removeTypeFilter()`

8. **Handle select functions** (for AddFilterDropdown): Add `handleSelectPriority()`, `handleSelectSeverity()`, `handleSelectType()` — same pattern as `handleSelectStatus()` (line 183).

9. **Effect cleanup** (line 196-200): Add new dropdown states to the condition:

```typescript
if (!projectDropdownOpen && !statusDropdownOpen && !assigneeDropdownOpen &&
    !priorityDropdownOpen && !severityDropdownOpen && !typeDropdownOpen) {
```

10. **Template**: Add 3 new `Popover.Root` blocks (after assignee Popover, before AddFilterDropdown) — follow exact same pattern as status Popover (lines 233-262). Each with:
    - `FilterChip` with appropriate icon and label
    - The corresponding `XxxFilterContent` component

11. **AddFilterDropdown props**: Pass new `has*Filter` and `onSelect*` props.

**QA:**

- All 6 filter chips appear when active
- Clear All removes all 6
- Adding a filter from dropdown works for all 6
- activeFilterCount reflects all active filters

### Task 11: Update `AddFilterDropdown.svelte`

**File:** `src/lib/components/dashboard/filters/AddFilterDropdown.svelte`
**What:** Add entries for Priority, Severity, Type filters.

**Changes:**

1. **Props** (lines 6-24): Add new props:

```typescript
hasPriorityFilter = false,
hasSeverityFilter = false,
hasTypeFilter = false,
onSelectPriority,
onSelectSeverity,
onSelectType,
```

With types:

```typescript
hasPriorityFilter?: boolean;
hasSeverityFilter?: boolean;
hasTypeFilter?: boolean;
onSelectPriority: () => void;
onSelectSeverity: () => void;
onSelectType: () => void;
```

2. **Icons import** (line 3): Add `Flag, AlertTriangle, Tag` to lucide imports.

3. **`handleSelect` function** (line 26-31): Extend with new types:

```typescript
function handleSelect(type: 'project' | 'status' | 'assignee' | 'priority' | 'severity' | 'type') {
	open = false;
	if (type === 'project') onSelectProject();
	else if (type === 'status') onSelectStatus();
	else if (type === 'assignee') onSelectAssignee();
	else if (type === 'priority') onSelectPriority();
	else if (type === 'severity') onSelectSeverity();
	else if (type === 'type') onSelectType();
}
```

4. **Template**: Add 3 new filter buttons (after assignee button, before the "all active" message). Follow the exact same pattern as existing buttons (lines 51-79):
   - Priority: `Flag` icon with `text-amber-400` hover color, label `$t('filters.priority')`
   - Severity: `AlertTriangle` icon with `text-red-400` hover color, label `$t('filters.severity')`
   - Type: `Tag` icon with `text-cyan-400` hover color, label `$t('filters.type')`

5. **"All filters active" condition** (line 81): Update to include all 6:

```typescript
{#if hasProjectFilter && hasStatusFilter && hasAssigneeFilter && hasPriorityFilter && hasSeverityFilter && hasTypeFilter}
```

**QA:**

- New filter options appear in dropdown when not active
- Selecting each opens the correct filter popover
- "All filters are active" shows when all 6 are active

### Task 12: Update `IssueTable.svelte` — Add New Columns

**File:** `src/lib/components/dashboard/IssueTable.svelte`
**What:** Add combined P/S column and Type column. Resolve names/colors from metadata.

**Changes:**

1. **Props**: Add `metadata` prop:

```typescript
let {
	issues = [],
	projects = [],
	metadata = {},
	changedIssueIds = new SvelteSet<number>(),
	onIssueSelect
}: {
	issues: Issue[];
	projects: Project[];
	metadata: Record<number, ProjectMetadata>;
	changedIssueIds?: SvelteSet<number>;
	onIssueSelect?: (issueId: number) => void;
} = $props();
```

Add import for `ProjectMetadata` type.

2. **Helper functions** — Add metadata resolver functions:

```typescript
function resolvePriority(issue: Issue): { name: string; color: string } | null {
	if (!issue.priority) return null;
	const meta = metadata[issue.project];
	if (!meta) return null;
	const found = meta.priorities.find((p) => p.id === issue.priority);
	return found ? { name: found.name, color: found.color } : null;
}

function resolveSeverity(issue: Issue): { name: string; color: string } | null {
	if (!issue.severity) return null;
	const meta = metadata[issue.project];
	if (!meta) return null;
	const found = meta.severities.find((s) => s.id === issue.severity);
	return found ? { name: found.name, color: found.color } : null;
}

function resolveType(issue: Issue): { name: string; color: string } | null {
	if (!issue.issue_type) return null;
	const meta = metadata[issue.project];
	if (!meta) return null;
	const found = meta.issue_types.find((t) => t.id === issue.issue_type);
	return found ? { name: found.name, color: found.color } : null;
}
```

3. **Table header** — Add 2 new `<th>` elements. Insert **after Status column** (after line 72), **before Project column**:

```svelte
<th
	class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
>
	{$t('table.prioritySeverity')}
</th>
<th
	class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
>
	{$t('table.type')}
</th>
```

4. **Table body** — Add 2 new `<td>` cells after the status cell (after line 140), before the project cell:

**P/S combined column:**

```svelte
<td class="px-2 py-2.5">
	{@const pri = resolvePriority(issue)}
	{@const sev = resolveSeverity(issue)}
	<div class="flex flex-col gap-0.5">
		{#if pri}
			<div class="flex items-center gap-1">
				<span class="size-1.5 rounded-full" style="background-color: {pri.color};"></span>
				<span class="text-[11px] text-[#93a9c8]">{pri.name}</span>
			</div>
		{/if}
		{#if sev}
			<div class="flex items-center gap-1">
				<span class="size-1.5 rounded-full" style="background-color: {sev.color};"></span>
				<span class="text-[11px] text-[#93a9c8]">{sev.name}</span>
			</div>
		{/if}
		{#if !pri && !sev}
			<span class="text-xs text-[#93a9c8]/50">—</span>
		{/if}
	</div>
</td>
```

**Type column:**

```svelte
<td class="px-2 py-2.5">
	{@const tp = resolveType(issue)}
	{#if tp}
		<div
			class="inline-flex items-center gap-1.5 rounded-full border px-2 py-0.5"
			style="border-color: {tp.color}20; background-color: {tp.color}10;"
		>
			<span class="size-1.5 rounded-full" style="background-color: {tp.color};"></span>
			<span class="text-xs font-medium" style="color: {tp.color};">{tp.name}</span>
		</div>
	{:else}
		<span class="text-xs text-[#93a9c8]/50">—</span>
	{/if}
</td>
```

5. **colspan** (line 93): Update from `"6"` to `"8"` for the "no issues" empty state.

**QA:**

- P/S column shows both values stacked when present
- P/S column shows single value when only one is present
- P/S column shows "—" when neither is present
- Type column shows colored badge (same style as Status badge)
- Type column shows "—" when null
- Colors render correctly from metadata

### Task 13: Update `DashboardScreen.svelte` — Pass metadata to IssueTable

**File:** `src/lib/screens/DashboardScreen.svelte`
**What:** Pass `metadata` prop to `IssueTable` component.

**Changes:**
At line 493, add `{metadata}` prop to `<IssueTable>`:

```svelte
<IssueTable
	{changedIssueIds}
	issues={filteredIssues}
	{projects}
	{metadata}
	onIssueSelect={handleIssueSelect}
/>
```

**QA:**

- Table renders with P/S and Type columns populated
- Switching views updates metadata and columns re-render

### Task 14: Add i18n Translation Keys

**File:** `src/lib/locales/en.json`
**What:** Add all new translation keys for filters and table columns.

**Add to `"filters"` section** (after line 139):

```json
"priority": "Priority",
"severity": "Severity",
"type": "Type",
"priorityFilter": "Priority Filter",
"severityFilter": "Severity Filter",
"typeFilter": "Type Filter",
"searchPriorities": "Search priorities...",
"searchSeverities": "Search severities...",
"searchTypes": "Search types...",
"noPriorities": "No priorities found",
"noSeverities": "No severities found",
"noTypes": "No types found"
```

**Add to `"table"` section** (after line 99):

```json
"prioritySeverity": "P / S",
"type": "Type"
```

**QA:**

- All new UI strings use `$t()` — no hardcoded English
- `pnpm check` passes

### Task 15: Add static locale file (if exists)

**File:** `static/locales/en.json` (if this file exists and has content)
**What:** Mirror the same translation keys added in Task 14.

Check if `static/locales/en.json` exists. If it does and contains filter/table keys, add the same new keys. If it doesn't exist or is empty, skip this task.

**QA:**

- Consistency between `src/lib/locales/en.json` and `static/locales/en.json`

## Final Verification Wave

After all tasks are complete, run the full verification suite:

1. **Rust checks:**

   ```bash
   cd src-tauri && cargo clippy -- -D warnings
   cd src-tauri && cargo test
   cd crates/taiga-client && cargo test
   ```

2. **Frontend checks:**

   ```bash
   pnpm check
   pnpm lint
   ```

3. **Functional verification (manual or via `just dev`):**
   - All 6 filters appear in AddFilterDropdown
   - Each filter opens correct dropdown with project-grouped items
   - Include/exclude toggle works for all 3 new filters
   - Filter chips appear in FilterBar with correct labels/values
   - Clear All removes all filters
   - Table shows 8 columns: Issue#, Subject, Status, P/S, Type, Project, Assignee, Last Modified
   - P/S column resolves names+colors from metadata
   - Saving a view with new filters → switching away → switching back restores correctly
   - Dirty state indicator works when changing new filters
