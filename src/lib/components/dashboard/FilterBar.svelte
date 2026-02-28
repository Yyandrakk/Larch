<script lang="ts">
	import { X, Folder, CircleDot, User, Flag, AlertTriangle, Tag } from '@lucide/svelte';
	import { t } from 'svelte-i18n';
	import { SvelteSet } from 'svelte/reactivity';
	import type { Project, FilterObject, ProjectMetadata } from '$lib/types';
	import * as Popover from '$lib/components/ui/popover';

	import FilterChip from './filters/FilterChip.svelte';
	import AddFilterDropdown from './filters/AddFilterDropdown.svelte';
	import SaveSplitButton from './SaveSplitButton.svelte';
	import ProjectFilterContent from './filters/ProjectFilterDropdown.svelte';
	import StatusFilterContent from './filters/StatusFilterDropdown.svelte';
	import AssigneeFilterContent from './filters/AssigneeFilterDropdown.svelte';
	import PriorityFilterContent from './filters/PriorityFilterDropdown.svelte';
	import SeverityFilterContent from './filters/SeverityFilterDropdown.svelte';
	import TypeFilterContent from './filters/TypeFilterDropdown.svelte';

	let {
		projects = [],
		metadata = {},
		filters = {},
		currentUserId,
		isDirty = false,
		canSave = false,
		isSystemView = false,
		onApply,
		onSave,
		onSaveAsNew,
		onDelete,
		userInteractedWithProjectFilter = $bindable(false)
	}: {
		projects: Project[];
		metadata: Record<number, ProjectMetadata>;
		filters: FilterObject;
		currentUserId?: number;
		isDirty?: boolean;
		canSave?: boolean;
		isSystemView?: boolean;
		onApply: (filters: FilterObject) => void;
		onSave?: () => void;
		onSaveAsNew?: () => void;
		onDelete?: () => void;
		userInteractedWithProjectFilter?: boolean;
	} = $props();

	let addFilterOpen = $state(false);
	let projectDropdownOpen = $state(false);
	let statusDropdownOpen = $state(false);
	let assigneeDropdownOpen = $state(false);
	let priorityDropdownOpen = $state(false);
	let severityDropdownOpen = $state(false);
	let typeDropdownOpen = $state(false);

	// Ref for AddFilterDropdown button to use as anchor for filter popovers
	let addFilterButtonRef = $state<HTMLElement | null>(null);

	// Track which popover was opened from AddFilterDropdown
	let openedFromAddFilter = $state<
		'project' | 'status' | 'assignee' | 'priority' | 'severity' | 'type' | null
	>(null);

	let hasProjectFilter = $derived(
		userInteractedWithProjectFilter && filters.project_ids && filters.project_ids.length > 0
	);
	let hasStatusFilter = $derived(filters.status_ids && filters.status_ids.length > 0);
	let hasAssigneeFilter = $derived(filters.assignee_ids && filters.assignee_ids.length > 0);
	let hasPriorityFilter = $derived(filters.priority_ids && filters.priority_ids.length > 0);
	let hasSeverityFilter = $derived(filters.severity_ids && filters.severity_ids.length > 0);
	let hasTypeFilter = $derived(filters.type_ids && filters.type_ids.length > 0);

	let activeFilterCount = $derived(
		(hasProjectFilter ? 1 : 0) +
			(hasStatusFilter ? 1 : 0) +
			(hasAssigneeFilter ? 1 : 0) +
			(hasPriorityFilter ? 1 : 0) +
			(hasSeverityFilter ? 1 : 0) +
			(hasTypeFilter ? 1 : 0)
	);

	function getProjectChipValue(): string {
		if (!filters.project_ids) return '';
		if (filters.project_ids.length === 1) {
			const project = projects.find((p) => p.id === filters.project_ids![0]);
			return project?.name || '';
		}
		return $t('filters.multiple');
	}

	function getStatusChipValue(): string {
		if (!filters.status_ids) return '';
		const statusNames = new SvelteSet<string>();
		filters.status_ids.forEach((sid) => {
			Object.values(metadata).forEach((meta) => {
				const status = meta.statuses.find((s) => s.id === sid);
				if (status) statusNames.add(status.name);
			});
		});
		if (statusNames.size === 1) {
			return Array.from(statusNames)[0];
		}
		return $t('filters.multiple');
	}

	function getAssigneeChipValue(): string {
		if (!filters.assignee_ids) return '';
		const UNASSIGNED_ID = -1;
		if (filters.assignee_ids.length === 1) {
			if (filters.assignee_ids[0] === UNASSIGNED_ID) {
				return $t('filters.unassigned');
			}
			const memberId = filters.assignee_ids[0];
			for (const meta of Object.values(metadata)) {
				const member = meta.members.find((m) => m.user_id === memberId);
				if (member) {
					if (member.user_id === currentUserId) {
						return $t('filters.me');
					}
					return member.full_name;
				}
			}
		}
		return $t('filters.multiple');
	}

	function getAssigneePhoto(): string | undefined {
		if (!filters.assignee_ids || filters.assignee_ids.length !== 1) return undefined;
		const memberId = filters.assignee_ids[0];
		if (memberId === -1) return undefined;
		for (const meta of Object.values(metadata)) {
			const member = meta.members.find((m) => m.user_id === memberId);
			if (member?.photo) return member.photo;
		}
		return undefined;
	}

	function handleProjectApply(ids: number[], exclude: boolean) {
		userInteractedWithProjectFilter = true;
		projectDropdownOpen = false;
		onApply({
			...filters,
			project_ids: ids.length > 0 ? ids : undefined,
			project_exclude: exclude
		});
	}

	function handleStatusApply(ids: number[], exclude: boolean) {
		statusDropdownOpen = false;
		onApply({
			...filters,
			status_ids: ids.length > 0 ? ids : undefined,
			status_exclude: exclude
		});
	}

	function handleAssigneeApply(ids: number[], exclude: boolean) {
		assigneeDropdownOpen = false;
		onApply({
			...filters,
			assignee_ids: ids.length > 0 ? ids : undefined,
			assignee_exclude: exclude
		});
	}

	function removeProjectFilter() {
		userInteractedWithProjectFilter = false;
		onApply({
			...filters,
			project_ids: undefined,
			project_exclude: false
		});
	}

	function removeStatusFilter() {
		onApply({
			...filters,
			status_ids: undefined,
			status_exclude: false
		});
	}

	function removeAssigneeFilter() {
		onApply({
			...filters,
			assignee_ids: undefined,
			assignee_exclude: false
		});
	}

	function clearAllFilters() {
		userInteractedWithProjectFilter = false;
		onApply({});
	}

	function handleSelectProject() {
		addFilterOpen = false;
		openedFromAddFilter = 'project';
		projectDropdownOpen = true;
	}

	function handleSelectStatus() {
		addFilterOpen = false;
		openedFromAddFilter = 'status';
		statusDropdownOpen = true;
	}

	function handleSelectAssignee() {
		addFilterOpen = false;
		openedFromAddFilter = 'assignee';
		assigneeDropdownOpen = true;
	}

	function getPriorityChipValue(): string {
		if (!filters.priority_ids) return '';
		const priorityNames = new SvelteSet<string>();
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
		const severityNames = new SvelteSet<string>();
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
		const typeNames = new SvelteSet<string>();
		filters.type_ids.forEach((tid) => {
			Object.values(metadata).forEach((meta) => {
				const type = meta.issue_types.find((t) => t.id === tid);
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

	// Clear openedFromAddFilter when popovers close
	$effect(() => {
		if (
			!projectDropdownOpen &&
			!statusDropdownOpen &&
			!assigneeDropdownOpen &&
			!priorityDropdownOpen &&
			!severityDropdownOpen &&
			!typeDropdownOpen
		) {
			openedFromAddFilter = null;
		}
	});
</script>

<div class="flex min-h-[52px] flex-wrap items-center gap-2 p-2">
	<Popover.Root bind:open={projectDropdownOpen}>
		<Popover.Trigger>
			{#snippet child({ props })}
				{#if hasProjectFilter}
					<div {...props}>
						<FilterChip
							label={$t('filters.project')}
							value={getProjectChipValue()}
							isExclude={filters.project_exclude ?? false}
							icon={Folder}
							onRemove={removeProjectFilter}
						/>
					</div>
				{:else}
					<span {...props} class="hidden"></span>
				{/if}
			{/snippet}
		</Popover.Trigger>
		<ProjectFilterContent
			{projects}
			selectedIds={filters.project_ids || []}
			isExclude={filters.project_exclude ?? false}
			onApply={handleProjectApply}
			customAnchor={!hasProjectFilter && openedFromAddFilter === 'project'
				? addFilterButtonRef
				: null}
		/>
	</Popover.Root>

	<Popover.Root bind:open={statusDropdownOpen}>
		<Popover.Trigger>
			{#snippet child({ props })}
				{#if hasStatusFilter}
					<div {...props}>
						<FilterChip
							label={$t('filters.status')}
							value={getStatusChipValue()}
							isExclude={filters.status_exclude ?? false}
							icon={CircleDot}
							onRemove={removeStatusFilter}
						/>
					</div>
				{:else}
					<span {...props} class="hidden"></span>
				{/if}
			{/snippet}
		</Popover.Trigger>
		<StatusFilterContent
			{metadata}
			{projects}
			selectedIds={filters.status_ids || []}
			isExclude={filters.status_exclude ?? false}
			open={statusDropdownOpen}
			onApply={handleStatusApply}
			customAnchor={!hasStatusFilter && openedFromAddFilter === 'status'
				? addFilterButtonRef
				: null}
		/>
	</Popover.Root>

	<Popover.Root bind:open={assigneeDropdownOpen}>
		<Popover.Trigger>
			{#snippet child({ props })}
				{#if hasAssigneeFilter}
					<div {...props}>
						<FilterChip
							label={$t('filters.assignee')}
							value={getAssigneeChipValue()}
							isExclude={filters.assignee_exclude ?? false}
							icon={User}
							avatarUrl={getAssigneePhoto()}
							onRemove={removeAssigneeFilter}
						/>
					</div>
				{:else}
					<span {...props} class="hidden"></span>
				{/if}
			{/snippet}
		</Popover.Trigger>
		<AssigneeFilterContent
			{metadata}
			selectedIds={filters.assignee_ids || []}
			isExclude={filters.assignee_exclude ?? false}
			{currentUserId}
			open={assigneeDropdownOpen}
			onApply={handleAssigneeApply}
			customAnchor={!hasAssigneeFilter && openedFromAddFilter === 'assignee'
				? addFilterButtonRef
				: null}
		/>
	</Popover.Root>

	<Popover.Root bind:open={priorityDropdownOpen}>
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
			customAnchor={!hasTypeFilter && openedFromAddFilter === 'type' ? addFilterButtonRef : null}
		/>
	</Popover.Root>

	<AddFilterDropdown
		bind:open={addFilterOpen}
		bind:buttonRef={addFilterButtonRef}
		{hasProjectFilter}
		{hasStatusFilter}
		{hasAssigneeFilter}
		{hasPriorityFilter}
		{hasSeverityFilter}
		{hasTypeFilter}
		onSelectProject={handleSelectProject}
		onSelectStatus={handleSelectStatus}
		onSelectAssignee={handleSelectAssignee}
		onSelectPriority={handleSelectPriority}
		onSelectSeverity={handleSelectSeverity}
		onSelectType={handleSelectType}
	/>

	<div class="ml-auto flex items-center gap-2">
		{#if activeFilterCount > 0}
			<div class="flex items-center gap-2 border-r border-[#2d3540] pr-2">
				<button
					class="flex items-center gap-1 rounded-lg px-3 py-1.5 text-xs font-medium text-slate-500 transition-colors hover:bg-[#2d3540] hover:text-red-400"
					onclick={clearAllFilters}
				>
					{$t('filters.clearAll')}
					<X class="h-3.5 w-3.5" />
				</button>
			</div>
		{/if}

		<SaveSplitButton
			{isDirty}
			{canSave}
			{isSystemView}
			onSave={onSave ?? (() => {})}
			onSaveAsNew={onSaveAsNew ?? (() => {})}
			onDelete={onDelete ?? (() => {})}
		/>
	</div>
</div>
