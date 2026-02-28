import re

with open('src/lib/components/dashboard/FilterBar.svelte', 'r') as f:
    content = f.read()

# 1. Icons
content = content.replace(
    "import { X, Folder, CircleDot, User } from '@lucide/svelte';",
    "import { X, Folder, CircleDot, User, Flag, AlertTriangle, Tag } from '@lucide/svelte';"
)

# 2. Filter Content Imports
content = content.replace(
    "import AssigneeFilterContent from './filters/AssigneeFilterDropdown.svelte';",
    "import AssigneeFilterContent from './filters/AssigneeFilterDropdown.svelte';\n\timport PriorityFilterContent from './filters/PriorityFilterDropdown.svelte';\n\timport SeverityFilterContent from './filters/SeverityFilterDropdown.svelte';\n\timport TypeFilterContent from './filters/TypeFilterDropdown.svelte';"
)

# 3. State variables
content = content.replace(
    "let assigneeDropdownOpen = $state(false);",
    "let assigneeDropdownOpen = $state(false);\n\tlet priorityDropdownOpen = $state(false);\n\tlet severityDropdownOpen = $state(false);\n\tlet typeDropdownOpen = $state(false);"
)

# 4. openedFromAddFilter
content = content.replace(
    "let openedFromAddFilter = $state<'project' | 'status' | 'assignee' | null>(null);",
    "let openedFromAddFilter = $state<'project' | 'status' | 'assignee' | 'priority' | 'severity' | 'type' | null>(null);"
)

# 5. Derived states
content = content.replace(
    "let hasAssigneeFilter = $derived(filters.assignee_ids && filters.assignee_ids.length > 0);",
    "let hasAssigneeFilter = $derived(filters.assignee_ids && filters.assignee_ids.length > 0);\n\tlet hasPriorityFilter = $derived(filters.priority_ids && filters.priority_ids.length > 0);\n\tlet hasSeverityFilter = $derived(filters.severity_ids && filters.severity_ids.length > 0);\n\tlet hasTypeFilter = $derived(filters.type_ids && filters.type_ids.length > 0);"
)

# 6. activeFilterCount
content = content.replace(
    "let activeFilterCount = $derived(\n\t\t(hasProjectFilter ? 1 : 0) + (hasStatusFilter ? 1 : 0) + (hasAssigneeFilter ? 1 : 0)\n\t);",
    "let activeFilterCount = $derived(\n\t\t(hasProjectFilter ? 1 : 0) +\n\t\t\t(hasStatusFilter ? 1 : 0) +\n\t\t\t(hasAssigneeFilter ? 1 : 0) +\n\t\t\t(hasPriorityFilter ? 1 : 0) +\n\t\t\t(hasSeverityFilter ? 1 : 0) +\n\t\t\t(hasTypeFilter ? 1 : 0)\n\t);"
)

# 7. Helper functions
helpers = """
	function getPriorityChipValue(): string {
		if (!filters.priority_ids) return '';
		const priorityNames = new Set<string>();
		filters.priority_ids.forEach((pid) => {
			Object.values(metadata).forEach((meta) => {
				const priority = meta.priorities.find((p) => p.id === pid);
				if (priority) priorityNames.add(priority.name);
			});
		});
		if (priorityNames.size === 1) {
			return Array.from(priorityNames)[0];
		}
		return $t('filters.multiple');
	}

	function getSeverityChipValue(): string {
		if (!filters.severity_ids) return '';
		const severityNames = new Set<string>();
		filters.severity_ids.forEach((sid) => {
			Object.values(metadata).forEach((meta) => {
				const severity = meta.severities.find((s) => s.id === sid);
				if (severity) severityNames.add(severity.name);
			});
		});
		if (severityNames.size === 1) {
			return Array.from(severityNames)[0];
		}
		return $t('filters.multiple');
	}

	function getTypeChipValue(): string {
		if (!filters.type_ids) return '';
		const typeNames = new Set<string>();
		filters.type_ids.forEach((tid) => {
			Object.values(metadata).forEach((meta) => {
				const type = meta.types.find((t) => t.id === tid);
				if (type) typeNames.add(type.name);
			});
		});
		if (typeNames.size === 1) {
			return Array.from(typeNames)[0];
		}
		return $t('filters.multiple');
	}

	function handlePriorityApply(ids: number[], exclude: boolean) {
		priorityDropdownOpen = false;
		onApply({
			...filters,
			priority_ids: ids.length > 0 ? ids : undefined,
			priority_exclude: exclude
		});
	}

	function handleSeverityApply(ids: number[], exclude: boolean) {
		severityDropdownOpen = false;
		onApply({
			...filters,
			severity_ids: ids.length > 0 ? ids : undefined,
			severity_exclude: exclude
		});
	}

	function handleTypeApply(ids: number[], exclude: boolean) {
		typeDropdownOpen = false;
		onApply({
			...filters,
			type_ids: ids.length > 0 ? ids : undefined,
			type_exclude: exclude
		});
	}

	function removePriorityFilter() {
		onApply({
			...filters,
			priority_ids: undefined,
			priority_exclude: false
		});
	}

	function removeSeverityFilter() {
		onApply({
			...filters,
			severity_ids: undefined,
			severity_exclude: false
		});
	}

	function removeTypeFilter() {
		onApply({
			...filters,
			type_ids: undefined,
			type_exclude: false
		});
	}

	function handleSelectPriority() {
		addFilterOpen = false;
		openedFromAddFilter = 'priority';
		priorityDropdownOpen = true;
	}

	function handleSelectSeverity() {
		addFilterOpen = false;
		openedFromAddFilter = 'severity';
		severityDropdownOpen = true;
	}

	function handleSelectType() {
		addFilterOpen = false;
		openedFromAddFilter = 'type';
		typeDropdownOpen = true;
	}
"""

content = content.replace(
    "\n\t// Clear openedFromAddFilter when popovers close",
    helpers + "\n\t// Clear openedFromAddFilter when popovers close"
)

# 8. Effect update
content = content.replace(
    "if (!projectDropdownOpen && !statusDropdownOpen && !assigneeDropdownOpen) {",
    "if (\n\t\t\t!projectDropdownOpen &&\n\t\t\t!statusDropdownOpen &&\n\t\t\t!assigneeDropdownOpen &&\n\t\t\t!priorityDropdownOpen &&\n\t\t\t!severityDropdownOpen &&\n\t\t\t!typeDropdownOpen\n\t\t) {"
)

# 9. Popovers
popovers = """	<Popover.Root bind:open={priorityDropdownOpen}>
		<Popover.Trigger>
			{#snippet child({ props })}
				{#if hasPriorityFilter}
					<div {...props}>
						<FilterChip
							label={$t('filters.priority')}
							value={getPriorityChipValue()}
							isExclude={filters.priority_exclude ?? false}
							icon={Flag}
							onRemove={removePriorityFilter}
						/>
					</div>
				{:else}
					<span {...props} class="hidden"></span>
				{/if}
			{/snippet}
		</Popover.Trigger>
		<PriorityFilterContent
			{metadata}
			{projects}
			selectedIds={filters.priority_ids || []}
			isExclude={filters.priority_exclude ?? false}
			open={priorityDropdownOpen}
			onApply={handlePriorityApply}
			customAnchor={!hasPriorityFilter && openedFromAddFilter === 'priority'
				? addFilterButtonRef
				: null}
		/>
	</Popover.Root>

	<Popover.Root bind:open={severityDropdownOpen}>
		<Popover.Trigger>
			{#snippet child({ props })}
				{#if hasSeverityFilter}
					<div {...props}>
						<FilterChip
							label={$t('filters.severity')}
							value={getSeverityChipValue()}
							isExclude={filters.severity_exclude ?? false}
							icon={AlertTriangle}
							onRemove={removeSeverityFilter}
						/>
					</div>
				{:else}
					<span {...props} class="hidden"></span>
				{/if}
			{/snippet}
		</Popover.Trigger>
		<SeverityFilterContent
			{metadata}
			{projects}
			selectedIds={filters.severity_ids || []}
			isExclude={filters.severity_exclude ?? false}
			open={severityDropdownOpen}
			onApply={handleSeverityApply}
			customAnchor={!hasSeverityFilter && openedFromAddFilter === 'severity'
				? addFilterButtonRef
				: null}
		/>
	</Popover.Root>

	<Popover.Root bind:open={typeDropdownOpen}>
		<Popover.Trigger>
			{#snippet child({ props })}
				{#if hasTypeFilter}
					<div {...props}>
						<FilterChip
							label={$t('filters.type')}
							value={getTypeChipValue()}
							isExclude={filters.type_exclude ?? false}
							icon={Tag}
							onRemove={removeTypeFilter}
						/>
					</div>
				{:else}
					<span {...props} class="hidden"></span>
				{/if}
			{/snippet}
		</Popover.Trigger>
		<TypeFilterContent
			{metadata}
			{projects}
			selectedIds={filters.type_ids || []}
			isExclude={filters.type_exclude ?? false}
			open={typeDropdownOpen}
			onApply={handleTypeApply}
			customAnchor={!hasTypeFilter && openedFromAddFilter === 'type'
				? addFilterButtonRef
				: null}
		/>
	</Popover.Root>

"""

content = content.replace(
    "\t<AddFilterDropdown\n\t\tbind:open={addFilterOpen}",
    popovers + "\t<AddFilterDropdown\n\t\tbind:open={addFilterOpen}"
)

# 10. AddFilterDropdown props
content = content.replace(
    "\n\t\t{hasAssigneeFilter}\n\t\tonSelectProject={handleSelectProject}",
    "\n\t\t{hasAssigneeFilter}\n\t\t{hasPriorityFilter}\n\t\t{hasSeverityFilter}\n\t\t{hasTypeFilter}\n\t\tonSelectProject={handleSelectProject}"
)

content = content.replace(
    "\t\tonSelectAssignee={handleSelectAssignee}\n\t/>",
    "\t\tonSelectAssignee={handleSelectAssignee}\n\t\tonSelectPriority={handleSelectPriority}\n\t\tonSelectSeverity={handleSelectSeverity}\n\t\tonSelectType={handleSelectType}\n\t/>"
)

with open('src/lib/components/dashboard/FilterBar.svelte', 'w') as f:
    f.write(content)

print("Patch applied")
