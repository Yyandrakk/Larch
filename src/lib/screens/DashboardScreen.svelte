<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import {
		CMD_GET_PROJECTS,
		CMD_GET_AGGREGATED_ISSUES,
		CMD_GET_SELECTED_PROJECTS,
		CMD_GET_PROJECT_METADATA
	} from '$lib/commands.svelte';
	import type { Issue, Project, FilterObject, ProjectMetadata } from '$lib/types';
	import IssueTable from '$lib/components/dashboard/IssueTable.svelte';
	import FilterBar from '$lib/components/dashboard/FilterBar.svelte';
	import { IssueDetailSheet } from '$lib/components/issue-detail';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { RefreshCw, Search } from '@lucide/svelte';
	import { toast } from 'svelte-sonner';
	import { t } from 'svelte-i18n';

	let issues = $state<Issue[]>([]);
	let projects = $state<Project[]>([]);
	let metadata = $state<Record<number, ProjectMetadata>>({});
	let filters = $state<FilterObject>({});
	let loading = $state(false);
	let searchQuery = $state('');

	// Issue Detail Sheet state
	let selectedIssueId = $state<number | null>(null);
	let sheetOpen = $state(false);

	let filteredIssues = $derived(
		issues.filter(
			(issue) =>
				searchQuery === '' ||
				issue.subject.toLowerCase().includes(searchQuery.toLowerCase()) ||
				(issue.assigned_to_name &&
					issue.assigned_to_name.toLowerCase().includes(searchQuery.toLowerCase()))
		)
	);

	async function loadData() {
		loading = true;
		try {
			// 1. Fetch all projects (for filter names)
			projects = await invoke(CMD_GET_PROJECTS);

			// 2. Fetch selected project IDs (for initial filter)
			const selectedIds: number[] = await invoke(CMD_GET_SELECTED_PROJECTS);

			// Initialize filters with selected projects if not already set
			if (!filters.project_ids && selectedIds.length > 0) {
				filters.project_ids = selectedIds;
			}

			// 3. Fetch metadata for selected projects
			if (selectedIds.length > 0) {
				metadata = await invoke(CMD_GET_PROJECT_METADATA, { projectIds: selectedIds });
			}

			// 4. Fetch issues
			await refreshIssues();
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
			issues = await invoke(CMD_GET_AGGREGATED_ISSUES, { filters });

			// Refresh metadata if project selection changed
			if (filters.project_ids && filters.project_ids.length > 0) {
				// Optimization: only fetch if we don't have it? For now, fetch to be safe.
				metadata = await invoke(CMD_GET_PROJECT_METADATA, { projectIds: filters.project_ids });
			}

			toast.success(`Loaded ${issues.length} issues`);
		} catch (error) {
			console.error('Failed to fetch issues:', error);
			toast.error($t('errors.unknown'));
		} finally {
			loading = false;
		}
	}

	function handleFilterChange(newFilters: FilterObject) {
		filters = newFilters;
		refreshIssues();
	}

	function handleIssueSelect(issueId: number) {
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

	onMount(() => {
		loadData();
	});
</script>

<div class="bg-background text-foreground flex h-full flex-col">
	<div class="border-b">
		<div class="flex h-16 items-center gap-4 px-4">
			<h2 class="text-lg font-semibold">{$t('dashboard.title')}</h2>

			<div class="relative max-w-md flex-1">
				<Search class="text-muted-foreground absolute top-2.5 left-2.5 h-4 w-4" />
				<Input
					type="search"
					placeholder={$t('dashboard.searchPlaceholder')}
					class="pl-8"
					bind:value={searchQuery}
				/>
			</div>

			<div class="ml-auto flex items-center space-x-4">
				<Button variant="outline" size="icon" onclick={refreshIssues} disabled={loading}>
					<RefreshCw class={loading ? 'animate-spin' : ''} />
				</Button>
			</div>
		</div>
	</div>

	<div class="flex-1 space-y-4 p-8 pt-6">
		<FilterBar {projects} {metadata} {filters} onApply={handleFilterChange} />
		<IssueTable issues={filteredIssues} {projects} onIssueSelect={handleIssueSelect} />
	</div>
</div>

<!-- Issue Detail Sheet -->
<IssueDetailSheet
	bind:issueId={selectedIssueId}
	bind:open={sheetOpen}
	onIssueUpdated={handleIssueUpdated}
/>
