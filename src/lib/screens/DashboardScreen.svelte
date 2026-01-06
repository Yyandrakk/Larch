<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import {
		CMD_GET_PROJECTS,
		CMD_GET_AGGREGATED_ISSUES,
		CMD_GET_SELECTED_PROJECTS,
		CMD_GET_PROJECT_METADATA,
		CMD_GET_ME
	} from '$lib/commands.svelte';
	import type { Issue, Project, FilterObject, ProjectMetadata } from '$lib/types';
	import IssueTable from '$lib/components/dashboard/IssueTable.svelte';
	import FilterBar from '$lib/components/dashboard/FilterBar.svelte';
	import { IssueDetailSheet } from '$lib/components/issue-detail';
	import { Search, RefreshCw } from '@lucide/svelte';
	import { toast } from 'svelte-sonner';
	import { t } from 'svelte-i18n';

	let issues = $state<Issue[]>([]);
	let projects = $state<Project[]>([]);
	let metadata = $state<Record<number, ProjectMetadata>>({});
	let filters = $state<FilterObject>({});
	let loading = $state(false);
	let searchQuery = $state('');
	let currentUserId = $state<number | undefined>(undefined);
	let userInteractedWithProjectFilter = $state(false);

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

	let activeProjectCount = $derived(filters.project_ids?.length || 0);

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

			if (!filters.project_ids && selectedIds.length > 0) {
				filters.project_ids = selectedIds;
			}

			if (selectedIds.length > 0) {
				metadata = await invoke(CMD_GET_PROJECT_METADATA, { projectIds: selectedIds });

				if (!filters.status_ids) {
					const closedStatusIds = new Set<number>();
					Object.values(metadata).forEach((meta) => {
						meta.statuses.forEach((status) => {
							if (status.is_closed) {
								closedStatusIds.add(status.id);
							}
						});
					});

					if (closedStatusIds.size > 0) {
						filters.status_ids = Array.from(closedStatusIds);
						filters.status_exclude = true;
					}
				}
			}

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

			if (filters.project_ids && filters.project_ids.length > 0) {
				metadata = await invoke(CMD_GET_PROJECT_METADATA, { projectIds: filters.project_ids });
			}

			toast.success(`${$t('dashboard.loaded')} ${issues.length}`);
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

<div class="flex flex-1 flex-col overflow-hidden">
	<div class="px-6 pt-6 pb-2">
		<div class="mb-6 flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
			<div class="flex flex-1 items-center gap-6">
				<div>
					<h2 class="text-2xl font-bold tracking-tight text-white">{$t('dashboard.allIssues')}</h2>
					<p class="mt-1 text-sm text-[#93a9c8]">
						{$t('dashboard.subtitlePrefix')}
						{activeProjectCount}
						{$t('dashboard.subtitleSuffix')}
					</p>
				</div>
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
				{projects}
				{metadata}
				{filters}
				{currentUserId}
				bind:userInteractedWithProjectFilter
				onApply={handleFilterChange}
			/>
		</div>
	</div>

	<div class="flex-1 overflow-auto px-6 pb-6">
		<IssueTable issues={filteredIssues} {projects} onIssueSelect={handleIssueSelect} />
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
