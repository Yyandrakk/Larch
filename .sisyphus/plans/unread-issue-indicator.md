# Plan: Unread/Changed Issue Indicator After Refresh

## Metadata

- **Goal**: Show a blue dot indicator next to issues that have changed since the user last saw them, triggered by the refresh action.
- **Approach**: Frontend-only, in-memory. No backend/Rust changes required.
- **Scope IN**: Blue dot indicator in IssueTable, snapshot comparison logic in DashboardScreen, mark-as-read on row click, i18n strings, tooltip.
- **Scope OUT**: Persistent read/unread tracking (DB), "Mark all as read" button, backend changes, notification badges outside the table.

## Architecture Overview

```
DashboardScreen.svelte
├── previousModifiedDates: Map<number, string>  (snapshot BEFORE refresh)
├── changedIssueIds: SvelteSet<number>           (computed AFTER refresh)
├── isFirstLoad: boolean                          (guards first load from false positives)
│
├── refreshIssues()  → snapshots → fetches → computes diff → updates changedIssueIds
├── handleIssueSelect(id) → changedIssueIds.delete(id)
│
└── IssueTable.svelte
    ├── prop: changedIssueIds: SvelteSet<number>
    └── renders blue dot when changedIssueIds.has(issue.id)
```

## Key Technical Decisions

1. **SvelteSet from `svelte/reactivity`** — NOT `$state(new Set())`. Mutations (`.add()`, `.delete()`) on plain `Set` inside `$state()` are silently non-reactive in Svelte 5. `SvelteSet` makes `.add()` and `.delete()` natively reactive.
2. **Snapshot via `Map<number, string>`** — Before each refresh, copy current `modified_date` values. After refresh, compare. New issues (not in snapshot) are also flagged.
3. **First load guard** — `isFirstLoad` flag prevents all issues from being marked "changed" on initial data load.
4. **Filter/view changes reset the changed set** — When the user changes filters or switches views, the changed set is cleared and the snapshot is re-seeded from the new result set. This avoids false positives from comparing unrelated issue sets.

## File Change Summary

| File                                             | Action | Description                                                     |
| ------------------------------------------------ | ------ | --------------------------------------------------------------- |
| `src/lib/screens/DashboardScreen.svelte`         | MODIFY | Add snapshot + diff logic, pass `changedIssueIds` to IssueTable |
| `src/lib/components/dashboard/IssueTable.svelte` | MODIFY | Accept new prop, render blue dot indicator                      |
| `src/lib/locales/en.json`                        | MODIFY | Add `table.changedIndicator` i18n key                           |

---

## Tasks

### TASK 1: Add snapshot state and diff logic to DashboardScreen.svelte

**File**: `src/lib/screens/DashboardScreen.svelte`

**What to do**:

1. **Add import** at the top of the `<script>` block (after existing imports, around line 27):

```typescript
import { SvelteSet } from 'svelte/reactivity';
```

2. **Add new state variables** after the existing state declarations (after line 40, near the other `$state` declarations):

```typescript
// Unread/changed tracking
let previousModifiedDates = $state(new Map<number, string | undefined>());
let changedIssueIds = $state(new SvelteSet<number>());
let isFirstLoad = $state(true);
```

3. **Create a helper function** to snapshot current issues. Place it after `handleIssueUpdated` (after line 345):

```typescript
function snapshotIssues(issueList: Issue[]): Map<number, string | undefined> {
	const snapshot = new Map<number, string | undefined>();
	for (const issue of issueList) {
		snapshot.set(issue.id, issue.modified_date);
	}
	return snapshot;
}

function computeChangedIds(
	oldSnapshot: Map<number, string | undefined>,
	newIssues: Issue[]
): Set<number> {
	const changed = new Set<number>();
	for (const issue of newIssues) {
		const previousDate = oldSnapshot.get(issue.id);
		if (previousDate === undefined) {
			// New issue not seen before — mark as changed
			changed.add(issue.id);
		} else if (previousDate !== issue.modified_date) {
			// Modified date differs — mark as changed
			changed.add(issue.id);
		}
	}
	return changed;
}
```

4. **Modify `refreshIssues()`** (currently at line 307). Replace the function body to add snapshot/diff logic:

```typescript
async function refreshIssues() {
	loading = true;
	try {
		// Snapshot BEFORE fetching (skip on first load)
		if (!isFirstLoad) {
			previousModifiedDates = snapshotIssues(issues);
		}

		issues = await invoke(CMD_GET_AGGREGATED_ISSUES, { filters });

		if (filters.project_ids && filters.project_ids.length > 0) {
			metadata = await invoke(CMD_GET_PROJECT_METADATA, { projectIds: filters.project_ids });
		}

		// Compute changed IDs (skip on first load)
		if (!isFirstLoad) {
			const newChangedIds = computeChangedIds(previousModifiedDates, issues);
			// Merge with existing changed IDs (preserve unread from previous refreshes)
			const merged = new SvelteSet<number>(changedIssueIds);
			for (const id of newChangedIds) {
				merged.add(id);
			}
			changedIssueIds = merged;
		} else {
			// First load: seed snapshot, mark nothing as changed
			previousModifiedDates = snapshotIssues(issues);
			isFirstLoad = false;
		}

		toast.success($t('dashboard.loadedCount', { values: { count: issues.length } }));
	} catch (error) {
		console.error('Failed to fetch issues:', error);
		toast.error($t('errors.unknown'));
	} finally {
		loading = false;
	}
}
```

5. **Modify `handleFilterChange()`** (currently at line 325). When filters change, reset the changed set and re-seed:

```typescript
function handleFilterChange(newFilters: FilterObject) {
	filters = newFilters;
	// Reset changed indicators when filters change (different issue set)
	changedIssueIds = new SvelteSet<number>();
	isFirstLoad = true;
	refreshIssues();
}
```

6. **Modify `handleViewSelect()`** (currently at line 156). Add reset before `refreshIssues()` is called inside the function. After line 190 (`filters = hydrateSystemFilters(...)`) and before `await refreshIssues()`:

```typescript
// Reset changed indicators when switching views
changedIssueIds = new SvelteSet<number>();
isFirstLoad = true;
```

7. **Modify `handleIssueSelect()`** (currently at line 330). Add mark-as-read logic:

```typescript
function handleIssueSelect(issueId: number) {
	changedIssueIds.delete(issueId);
	selectedIssueId = issueId;
	sheetOpen = true;
}
```

8. **Update the `<IssueTable>` usage** (currently at line 452). Pass the new prop:

```svelte
<IssueTable
	issues={filteredIssues}
	{projects}
	{changedIssueIds}
	onIssueSelect={handleIssueSelect}
/>
```

**QA Scenarios**:

- First load: NO blue dots appear on any issue.
- Click refresh: Issues with changed `modified_date` show blue dot. Unchanged issues do not.
- New issue appears after refresh: Shows blue dot.
- Click an issue row: Blue dot disappears for that issue only.
- Click refresh again: Previously read issues stay without dot (unless changed again). New changes get dots.
- Switch view or change filter: All dots clear, no dots on fresh load.
- Issue removed after refresh (no longer in results): No error or stale dot.

---

### TASK 2: Add blue dot indicator to IssueTable.svelte

**File**: `src/lib/components/dashboard/IssueTable.svelte`

**What to do**:

1. **Update imports** — Add SvelteSet import at the top of the script:

```typescript
import { SvelteSet } from 'svelte/reactivity';
```

2. **Update props** to accept `changedIssueIds`. Replace the current destructuring (lines 6-14):

```typescript
let {
	issues = [],
	projects = [],
	changedIssueIds = new SvelteSet<number>(),
	onIssueSelect
}: {
	issues: Issue[];
	projects: Project[];
	changedIssueIds: SvelteSet<number>;
	onIssueSelect?: (issueId: number) => void;
} = $props();
```

3. **Add the blue dot** inside the issue ID `<td>` cell. Replace the current cell content (line 108-110):

Current:

```svelte
<td class="px-2 py-2.5 font-mono text-xs text-[#93a9c8]">
	#{issue.id}
</td>
```

Replace with:

```svelte
<td class="px-2 py-2.5 font-mono text-xs text-[#93a9c8]">
	<div class="flex items-center gap-1.5">
		{#if changedIssueIds.has(issue.id)}
			<span class="size-2 shrink-0 rounded-full bg-[#196ee6]" title={$t('table.changedIndicator')}
			></span>
		{:else}
			<span class="size-2 shrink-0"></span>
		{/if}
		#{issue.id}
	</div>
</td>
```

**Design Notes**:

- The blue dot uses `#196ee6` — the existing accent blue already used in the app (visible in initials fallback avatar, hover text color).
- `size-2` = 8px diameter dot. Small and non-intrusive.
- An invisible spacer of the same size (`{:else}` branch) prevents layout shift when dots appear/disappear.
- `shrink-0` prevents the dot from being squeezed by flex layout.

**QA Scenarios**:

- Blue dot appears left of `#issueId` for changed issues.
- No layout shift between changed and unchanged rows (spacer ensures consistent width).
- Dot has a tooltip on hover showing the i18n string.
- Dot color matches the app's accent blue.
- When `changedIssueIds` is empty (first load, after view switch), no dots appear.

---

### TASK 3: Add i18n translation key

**File**: `src/lib/locales/en.json`

**What to do**:

Add a new key inside the existing `"table"` section (after line 98, which is `"unassigned": "Unassigned"`):

```json
"changedIndicator": "Modified since last refresh"
```

The full `"table"` section should look like:

```json
"table": {
    "issue": "Issue",
    "subject": "Subject",
    "status": "Status",
    "project": "Project",
    "assignedTo": "Assignee",
    "unassigned": "Unassigned",
    "changedIndicator": "Modified since last refresh"
}
```

**QA Scenarios**:

- Hover over a blue dot: Tooltip shows "Modified since last refresh".
- Valid JSON (no trailing comma issues).

---

## Final Verification Wave

After all tasks are complete, run these checks:

1. **`pnpm check`** — Ensure svelte-check + tsc pass with no errors.
2. **`pnpm lint`** — Ensure no formatting/linting violations.
3. **`pnpm build`** — Ensure the app builds successfully.
4. **Manual QA flow**:
   - Open app → Dashboard loads → NO blue dots.
   - Click "Refresh" → If any issues changed on the Taiga server, blue dots appear.
   - Click on a blue-dotted row → Sheet opens, dot disappears.
   - Switch view → All dots cleared.
   - Change filter → All dots cleared.
