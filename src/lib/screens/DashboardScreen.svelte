<script lang="ts">
	import { onMount } from 'svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import { invoke } from '@tauri-apps/api/core';
	import {
		CMD_GET_PROJECTS,
		CMD_GET_AGGREGATED_ISSUES,
		CMD_GET_SELECTED_PROJECTS,
		CMD_GET_PROJECT_METADATA,
		CMD_GET_ME,
		CMD_LIST_VIEWS,
		CMD_CREATE_VIEW,
		CMD_UPDATE_VIEW,
		CMD_DELETE_VIEW,
		CMD_SWITCH_VIEW,
		CMD_SANITIZE_VIEWS
	} from '$lib/commands.svelte';
	import type { Issue, Project, FilterObject, ProjectMetadata, SavedView } from '$lib/types';
	import IssueTable from '$lib/components/dashboard/IssueTable.svelte';
	import FilterBar from '$lib/components/dashboard/FilterBar.svelte';
	import ViewSwitcher from '$lib/components/dashboard/ViewSwitcher.svelte';
	import SaveViewDialog from '$lib/components/dashboard/SaveViewDialog.svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { deepEqual } from '$lib/utils/filterUtils';
	import { IssueDetailSheet } from '$lib/components/issue-detail';
	import { Search, RefreshCw } from '@lucide/svelte';
	import { toast } from 'svelte-sonner';
	import { t } from 'svelte-i18n';

	let issues = $state<Issue[]>([]);
	let projects = $state<Project[]>([]);
	let selectedProjectIds = $state<number[]>([]);
	let metadata = $state<Record<number, ProjectMetadata>>({});
	let filters = $state<FilterObject>({});
	let loading = $state(false);
	let searchQuery = $state('');
	let currentUserId = $state<number | undefined>(undefined);
	let userInteractedWithProjectFilter = $state(false);

	let previousModifiedDates = $state<Map<number, string | null>>(new Map());
	let changedIssueIds = $state(new SvelteSet<number>());
	let isFirstLoad = $state(true);

	let selectedIssueId = $state<number | null>(null);
	let sheetOpen = $state(false);

	// View State
	let views = $state<SavedView[]>([]);
	let currentView = $state<SavedView | null>(null);
	let saveDialogOpen = $state(false);
	let deleteDialogOpen = $state(false);
	let viewToDelete = $state<SavedView | null>(null);

	let filteredIssues = $derived(
		issues.filter(
			(issue) =>
				searchQuery === '' ||
				issue.subject.toLowerCase().includes(searchQuery.toLowerCase()) ||
				(issue.assigned_to_name &&
					issue.assigned_to_name.toLowerCase().includes(searchQuery.toLowerCase()))
		)
	);

	let activeProjectCount = $derived(filters.project_ids?.length || 0);

	let activeProjects = $derived(
		projects.filter((project) => selectedProjectIds.includes(project.id))
	);

	let isSystemView = $derived(currentView?.is_system ?? false);

	let currentViewFilter = $derived.by(() => {
		if (!currentView) return {};
		try {
			const raw = JSON.parse(currentView.filter_data) as FilterObject;
			return hydrateSystemFilters(raw, currentView.is_system);
		} catch (e) {
			console.error('Failed to parse view filter data', e);
			return {};
		}
	});

	function hydrateSystemFilters(raw: FilterObject, isSystem: boolean): FilterObject {
		if (!isSystem) return raw;

		// Logic for "Active Triage" (system view with exclude=true but no explicit IDs)
		if (raw.status_exclude && (!raw.status_ids || raw.status_ids.length === 0)) {
			const hydrated = { ...raw };

			const targetProjectIds =
				hydrated.project_ids && hydrated.project_ids.length > 0
					? hydrated.project_ids
					: selectedProjectIds;

			const closedStatusIds = new Set<number>();
			targetProjectIds.forEach((pid) => {
				const meta = metadata[pid];
				if (meta) {
					meta.statuses.forEach((status) => {
						if (status.is_closed) {
							closedStatusIds.add(status.id);
						}
					});
				}
			});

			if (closedStatusIds.size > 0) {
				hydrated.status_ids = Array.from(closedStatusIds);
			}

			return hydrated;
		}

		return raw;
	}

	function snapshotIssues(targetIssues: Issue[]) {
		const snapshot = new Map<number, string | null>();
		targetIssues.forEach((issue) => {
			snapshot.set(issue.id, issue.modified_date ?? null);
		});
		return snapshot;
	}

	function computeChangedIds(newIssues: Issue[]) {
		const changed = new Set<number>();
		newIssues.forEach((issue) => {
			const prevDate = previousModifiedDates.get(issue.id);
			if (prevDate === undefined || (issue.modified_date ?? null) !== prevDate) {
				changed.add(issue.id);
			}
		});
		return changed;
	}

	let isDirty = $derived(currentView ? !deepEqual(filters, currentViewFilter) : false);

	let canSave = $derived(!!currentView && !isSystemView);

	async function loadViews(): Promise<SavedView[]> {
		try {
			views = await invoke(CMD_LIST_VIEWS);
			if (currentView) {
				const updated = views.find((v) => v.id === currentView!.id);
				if (updated) currentView = updated;
			}
			return views;
		} catch (error) {
			console.error('Failed to load views:', error);
			toast.error($t('errors.unknown'));
			return [];
		}
	}

	function findDefaultView(loadedViews: SavedView[]): SavedView | null {
		// First try to find the system view (Active Triage)
		const systemView = loadedViews.find((v) => v.is_system);
		if (systemView) return systemView;

		// Fallback to any view marked as default
		const defaultView = loadedViews.find((v) => v.is_default);
		if (defaultView) return defaultView;

		// Last resort: return first view or null
		return loadedViews.length > 0 ? loadedViews[0] : null;
	}

	async function sanitizeViews(projectIds?: number[], statusIds?: number[]) {
		try {
			await invoke(CMD_SANITIZE_VIEWS, {
				validProjectIds: projectIds,
				validStatusIds: statusIds
			});
			await loadViews();
		} catch (error) {
			console.error('Failed to sanitize views:', error);
		}
	}

	async function handleViewSelect(view: SavedView | null) {
		if (!view) {
			currentView = null;
			return;
		}

		changedIssueIds.clear();
		isFirstLoad = true;
		currentView = view;
		try {
			const rawFilters = JSON.parse(view.filter_data) as FilterObject;

			// 1. Determine which projects we need metadata for
			let projectsToLoad: number[] = [];

			if (rawFilters.project_ids && rawFilters.project_ids.length > 0) {
				projectsToLoad = rawFilters.project_ids;
			} else if (view.is_system && rawFilters.status_exclude) {
				// For Active Triage, we need metadata for currently selected projects
				// to calculate closed statuses
				projectsToLoad = selectedProjectIds;
			}

			// 2. Load missing metadata
			if (projectsToLoad.length > 0) {
				const missingProjects = projectsToLoad.filter((id) => !metadata[id]);
				if (missingProjects.length > 0) {
					const newMetadata = await invoke<Record<number, ProjectMetadata>>(
						CMD_GET_PROJECT_METADATA,
						{ projectIds: projectsToLoad }
					);
					metadata = { ...metadata, ...newMetadata };
				}
			}

			// 3. Hydrate filters with metadata (calculate status IDs for system views)
			filters = hydrateSystemFilters(rawFilters, view.is_system);

			await refreshIssues();

			invoke(CMD_SWITCH_VIEW, { id: view.id }).catch(console.error);
		} catch (e) {
			console.error('Failed to apply view', e);
			toast.error($t('errors.unknown'));
		}
	}

	async function handleSave() {
		if (!currentView || !canSave) return;

		try {
			await invoke(CMD_UPDATE_VIEW, {
				id: currentView.id,
				name: currentView.name,
				filterData: JSON.stringify(filters)
			});
			toast.success($t('views.saved'));
			await loadViews();
		} catch (error) {
			console.error('Failed to save view:', error);
			toast.error($t('errors.unknown'));
		}
	}

	function handleSaveAsNew() {
		saveDialogOpen = true;
	}

	async function handleCreateView(name: string) {
		try {
			const newView: SavedView = await invoke(CMD_CREATE_VIEW, {
				name,
				filterData: JSON.stringify(filters)
			});
			toast.success($t('views.created'));
			await loadViews();
			currentView = newView;
			saveDialogOpen = false;
		} catch (error) {
			console.error('Failed to create view:', error);
			toast.error($t('errors.unknown'));
		}
	}

	function handleDelete() {
		if (!currentView) return;
		viewToDelete = currentView;
		deleteDialogOpen = true;
	}

	async function confirmDelete() {
		if (!viewToDelete) return;
		try {
			await invoke(CMD_DELETE_VIEW, { id: viewToDelete.id });
			toast.success($t('views.deleted'));

			// If we deleted the currently selected view, switch to system view (Active Triage)
			if (currentView?.id === viewToDelete.id) {
				await loadViews();
				const systemView = views.find((v) => v.is_system);
				if (systemView) {
					await handleViewSelect(systemView);
				} else {
					currentView = null;
				}
			} else {
				await loadViews();
			}

			deleteDialogOpen = false;
			viewToDelete = null;
		} catch (error) {
			console.error('Failed to delete view:', error);
			toast.error($t('errors.unknown'));
		}
	}

	async function loadData() {
		loading = true;
		try {
			projects = await invoke(CMD_GET_PROJECTS);

			try {
				const me: { id: number } = await invoke(CMD_GET_ME);
				currentUserId = me.id;
			} catch {
				currentUserId = undefined;
			}

			const selectedIds: number[] = await invoke(CMD_GET_SELECTED_PROJECTS);
			selectedProjectIds = selectedIds;

			// Sanitize views using all available projects (not just selected ones)
			// to avoid deleting filters for projects that are just hidden.
			// We don't sanitize statuses as we don't have full metadata.
			const allProjectIds = projects.map((p) => p.id);
			await sanitizeViews(allProjectIds, undefined);

			// Load metadata for selected projects if any
			if (selectedIds.length > 0) {
				metadata = await invoke(CMD_GET_PROJECT_METADATA, { projectIds: selectedIds });
			}

			// Note: We don't call refreshIssues() here anymore.
			// Issues will be loaded after the default view is selected.
		} catch (error) {
			console.error('Failed to load data:', error);
			toast.error($t('errors.unknown'));
		} finally {
			loading = false;
		}
	}

	async function refreshIssues() {
		loading = true;
		try {
			const snapshot = snapshotIssues(issues);
			const fetchedIssues: Issue[] = await invoke(CMD_GET_AGGREGATED_ISSUES, { filters });

			if (isFirstLoad) {
				previousModifiedDates = snapshotIssues(fetchedIssues);
				isFirstLoad = false;
			} else {
				previousModifiedDates = snapshot;
				const newChanged = computeChangedIds(fetchedIssues);
				newChanged.forEach((id) => changedIssueIds.add(id));
			}

			issues = fetchedIssues;

			if (filters.project_ids && filters.project_ids.length > 0) {
				metadata = await invoke(CMD_GET_PROJECT_METADATA, { projectIds: filters.project_ids });
			}

			toast.success($t('dashboard.loadedCount', { values: { count: issues.length } }));
		} catch (error) {
			console.error('Failed to fetch issues:', error);
			toast.error($t('errors.unknown'));
		} finally {
			loading = false;
		}
	}

	function handleFilterChange(newFilters: FilterObject) {
		changedIssueIds.clear();
		isFirstLoad = true;
		filters = newFilters;
		refreshIssues();
	}

	function handleIssueSelect(issueId: number) {
		changedIssueIds.delete(issueId);
		selectedIssueId = issueId;
		sheetOpen = true;
	}

	async function handleIssueUpdated() {
		loading = true;
		try {
			issues = await invoke(CMD_GET_AGGREGATED_ISSUES, { filters });
		} catch (error) {
			console.error('Failed to refresh issues after update:', error);
			toast.error($t('errors.unknown'));
		} finally {
			loading = false;
		}
	}

	$effect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 's') {
				const activeElement = document.activeElement;
				const isInput =
					activeElement instanceof HTMLInputElement ||
					activeElement instanceof HTMLTextAreaElement ||
					activeElement?.closest('[role="dialog"]');

				if (isInput) return;

				e.preventDefault();

				if (e.shiftKey) {
					handleSaveAsNew();
				} else if (isDirty) {
					if (isSystemView) {
						handleSaveAsNew();
					} else {
						handleSave();
					}
				}
			}
		};

		window.addEventListener('keydown', handleKeyDown);
		return () => {
			window.removeEventListener('keydown', handleKeyDown);
		};
	});

	onMount(async () => {
		// Load views and projects first
		const loadedViews = await loadViews();
		await loadData();

		// Then select the default view (which will trigger issue loading)
		const defaultView = findDefaultView(loadedViews);
		if (defaultView) {
			await handleViewSelect(defaultView);
		}
	});
</script>

<div class="flex flex-1 flex-col overflow-hidden">
	<div class="px-6 pt-6 pb-2">
		<div class="mb-6 flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
			<div class="flex flex-1 items-center gap-6">
				<ViewSwitcher
					{views}
					{currentView}
					{isDirty}
					onSelectView={(id) => {
						const view = views.find((v) => v.id === id) || null;
						handleViewSelect(view);
					}}
					onDeleteView={(id) => {
						const view = views.find((v) => v.id === id);
						if (view) {
							viewToDelete = view;
							deleteDialogOpen = true;
						}
					}}
				/>
				<div class="relative hidden w-full max-w-sm sm:flex">
					<Search class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-[#93a9c8]" />
					<input
						type="text"
						placeholder={$t('dashboard.searchPlaceholder')}
						bind:value={searchQuery}
						class="w-full rounded-lg border border-transparent bg-[#1a2433] py-1.5 pr-4 pl-10 text-sm text-white placeholder-[#93a9c8] transition-all focus:border-[#243347] focus:bg-[#243347] focus:ring-0"
					/>
				</div>
			</div>
			<div class="flex items-center gap-3">
				<button
					onclick={refreshIssues}
					disabled={loading}
					class="flex items-center gap-2 rounded-lg border border-[#243347] px-3 py-1.5 text-sm font-medium text-[#93a9c8] transition-colors hover:bg-[#243347] hover:text-white disabled:opacity-50"
				>
					<RefreshCw class="h-4 w-4 {loading ? 'animate-spin' : ''}" />
					{$t('dashboard.refresh')}
				</button>
			</div>
		</div>

		<div class="rounded-xl border border-[#243347] bg-[#161e2a] shadow-sm">
			<FilterBar
				projects={activeProjects}
				{metadata}
				{filters}
				{currentUserId}
				bind:userInteractedWithProjectFilter
				onApply={handleFilterChange}
				{isDirty}
				{canSave}
				{isSystemView}
				onSave={handleSave}
				onSaveAsNew={handleSaveAsNew}
				onDelete={handleDelete}
			/>
		</div>
	</div>

	<div class="flex-1 overflow-auto px-6 pb-6">
		<IssueTable
			{changedIssueIds}
			issues={filteredIssues}
			{projects}
			onIssueSelect={handleIssueSelect}
		/>
	</div>

	<div
		class="flex h-10 items-center justify-between border-t border-[#243347] bg-[#161e2a] px-6 text-xs text-[#93a9c8]"
	>
		<div class="flex items-center gap-2">
			<span>
				{$t('dashboard.showing')}
				{filteredIssues.length}
				{$t('dashboard.of')}
				{issues.length}
				{$t('dashboard.issues')}
			</span>
		</div>
	</div>
</div>

<IssueDetailSheet
	bind:issueId={selectedIssueId}
	bind:open={sheetOpen}
	onIssueUpdated={handleIssueUpdated}
/>

<SaveViewDialog bind:open={saveDialogOpen} onSave={handleCreateView} />

<AlertDialog.Root bind:open={deleteDialogOpen}>
	<AlertDialog.Content class="border-[#243347] bg-[#161e2a]">
		<AlertDialog.Header>
			<AlertDialog.Title>{$t('views.deleteTitle')}</AlertDialog.Title>
			<AlertDialog.Description>
				{#if viewToDelete}
					{$t('views.deleteDescription', { values: { name: viewToDelete.name } })}
				{/if}
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel class="border-[#243347] bg-[#1a2433] hover:bg-[#243347] hover:text-white">
				{$t('common.cancel')}
			</AlertDialog.Cancel>
			<AlertDialog.Action class="bg-red-600 text-white hover:bg-red-700" onclick={confirmDelete}>
				{$t('common.delete')}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
