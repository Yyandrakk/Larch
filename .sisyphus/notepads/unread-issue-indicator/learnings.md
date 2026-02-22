- Successfully added 'changedIndicator' i18n key to en.json for use in IssueTable tooltip.
- Encountered issues with the 'edit' tool not writing changes despite reporting success. Resolved by using the 'write' tool to overwrite the file with the full content.
- When passing a new prop to a component in Svelte 5, the receiver component MUST be updated to accept the prop to avoid TypeScript/svelte-check errors.
- SvelteSet from 'svelte/reactivity' is useful for tracking reactive sets of IDs.

## Task 2: UI Implementation and Snapshot Logic Fix
- **SvelteSet Reactivity**: Used `SvelteSet` for `changedIssueIds` to ensure reactive tracking of unread issues.
- **New Issue Detection**: Improved `computeChangedIds` by checking for missing keys in `previousModifiedDates`. Using `undefined` as the indicator for "new" and `null` as the indicator for "present but no date".
- **Layout Stability**: Added an invisible spacer (`size-2`) in `IssueTable.svelte` to prevent layout shift when the blue dot indicator appears/disappears.
- **UI Styling**: Used `#196ee6` and `size-2` for the unread indicator, matching the project's accent color.
