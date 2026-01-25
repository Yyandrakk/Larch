<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Search, Filter, GitMerge, Puzzle, Combine, Ban } from '@lucide/svelte';
	import { t } from 'svelte-i18n';
	import type { IssueStatus, ProjectMetadata, Project } from '$lib/types';

	let {
		metadata = {},
		projects = [],
		selectedIds = [],
		isExclude = false,
		onApply,
		customAnchor = null
	}: {
		metadata: Record<number, ProjectMetadata>;
		projects: Project[];
		selectedIds: number[];
		isExclude: boolean;
		onApply: (ids: number[], exclude: boolean) => void;
		customAnchor?: HTMLElement | null;
	} = $props();

	// Local state for the form - synced from props reactively
	let localSelectedIds = $state<number[]>([]);
	let localExclude = $state(false);
	let searchQuery = $state('');

	$effect(() => {
		localSelectedIds = [...selectedIds];
		localExclude = isExclude;
	});

	interface StatusWithProjects {
		status: IssueStatus;
		projectIds: number[];
		projectNames: string[];
	}

	let groupedStatuses = $derived.by(() => {
		const statusMap = new Map<string, StatusWithProjects>();

		Object.entries(metadata).forEach(([pidStr, meta]) => {
			const pid = parseInt(pidStr);
			const project = projects.find((p) => p.id === pid);
			const projectName = project?.name || `Project ${pid}`;

			meta.statuses.forEach((status) => {
				const key = status.name.toLowerCase();
				if (statusMap.has(key)) {
					const existing = statusMap.get(key)!;
					existing.projectIds.push(pid);
					existing.projectNames.push(projectName);
				} else {
					statusMap.set(key, {
						status: { ...status, color: '#6b7280' },
						projectIds: [pid],
						projectNames: [projectName]
					});
				}
			});
		});

		return Array.from(statusMap.values());
	});

	let unifiedStatuses = $derived(
		groupedStatuses
			.filter((s) => s.projectIds.length > 1)
			.filter((s) => s.status.name.toLowerCase().includes(searchQuery.toLowerCase()))
	);

	let projectSpecificStatuses = $derived(
		groupedStatuses
			.filter((s) => s.projectIds.length === 1)
			.filter((s) => s.status.name.toLowerCase().includes(searchQuery.toLowerCase()))
	);

	function getStatusIdsByName(statusName: string): number[] {
		const ids: number[] = [];
		Object.values(metadata).forEach((meta) => {
			meta.statuses.forEach((s) => {
				if (s.name.toLowerCase() === statusName.toLowerCase()) {
					ids.push(s.id);
				}
			});
		});
		return ids;
	}

	function isStatusSelected(statusName: string): boolean {
		const ids = getStatusIdsByName(statusName);
		return ids.some((id) => localSelectedIds.includes(id));
	}

	function toggleStatus(statusName: string) {
		const ids = getStatusIdsByName(statusName);
		const anySelected = ids.some((id) => localSelectedIds.includes(id));

		if (anySelected) {
			localSelectedIds = localSelectedIds.filter((id) => !ids.includes(id));
		} else {
			localSelectedIds = [...localSelectedIds, ...ids];
		}
	}

	function handleApply() {
		onApply(localSelectedIds, localExclude);
	}

	function handleClear() {
		localSelectedIds = [];
	}

	function getProjectColor(projectName: string): string {
		const colors = [
			'bg-emerald-500',
			'bg-purple-500',
			'bg-blue-500',
			'bg-orange-500',
			'bg-pink-500',
			'bg-cyan-500'
		];
		const hash = projectName.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
		return colors[hash % colors.length];
	}

	let selectedCount = $derived(
		groupedStatuses.filter((s) => isStatusSelected(s.status.name)).length
	);

	let allVisibleStatuses = $derived([...unifiedStatuses, ...projectSpecificStatuses]);

	function getAllVisibleStatusIds() {
		const ids: number[] = [];
		allVisibleStatuses.forEach((s) => {
			ids.push(...getStatusIdsByName(s.status.name));
		});
		return ids;
	}

	let areAllVisibleSelected = $derived.by(() => {
		if (allVisibleStatuses.length === 0) return false;
		return allVisibleStatuses.every((s) => isStatusSelected(s.status.name));
	});

	let isAnyVisibleSelected = $derived.by(() => {
		if (allVisibleStatuses.length === 0) return false;
		return allVisibleStatuses.some((s) => isStatusSelected(s.status.name));
	});

	let masterChecked = $derived(areAllVisibleSelected);
	let masterIndeterminate = $derived(!areAllVisibleSelected && isAnyVisibleSelected);

	function toggleAll() {
		const allIds = getAllVisibleStatusIds();
		if (masterChecked) {
			localSelectedIds = localSelectedIds.filter((id) => !allIds.includes(id));
		} else {
			localSelectedIds = [...new Set([...localSelectedIds, ...allIds])];
		}
	}
</script>

<Popover.Content
	class="flex w-[360px] flex-col overflow-hidden rounded-xl border border-[#2d3540] bg-[#1e2329] p-0 shadow-2xl ring-1 shadow-black/80 ring-white/5"
	align="start"
	sideOffset={8}
	{customAnchor}
>
	<div class="border-b border-[#2d3540] bg-[#111821] p-3">
		<div class="mb-3 flex items-center justify-between">
			<div class="flex items-center gap-2 text-slate-400">
				<Filter class="h-[18px] w-[18px]" />
				<span class="text-xs font-bold tracking-wider uppercase">{$t('filters.statusFilter')}</span>
			</div>
			<div class="flex h-7 items-center rounded-lg bg-[#2d3540] p-0.5">
				<button
					class="flex h-6 w-6 items-center justify-center rounded-md transition-all {!localExclude
						? 'bg-[#196ee6]/20 text-[#196ee6]'
						: 'text-slate-500 hover:bg-white/5 hover:text-slate-300'}"
					onclick={() => (localExclude = false)}
					title={$t('filters.include')}
				>
					<Combine class="h-3.5 w-3.5" />
				</button>
				<button
					class="flex h-6 w-6 items-center justify-center rounded-md transition-all {localExclude
						? 'bg-red-500/20 text-red-400'
						: 'text-slate-500 hover:bg-white/5 hover:text-slate-300'}"
					onclick={() => (localExclude = true)}
					title={$t('filters.exclude')}
				>
					<Ban class="h-3.5 w-3.5" />
				</button>
			</div>
		</div>
		<div class="group relative">
			<Search
				class="absolute top-2.5 left-3 h-4 w-4 text-slate-500 transition-colors {localExclude
					? 'group-focus-within:text-red-400'
					: 'group-focus-within:text-[#196ee6]'}"
			/>
			<input
				type="text"
				placeholder={$t('filters.searchStatuses')}
				bind:value={searchQuery}
				class="w-full rounded-lg border border-[#2d3540] bg-[#1e2329] py-2 pr-3 pl-9 text-sm text-white transition-all placeholder:text-slate-600 focus:ring-1 focus:outline-none {localExclude
					? 'focus:border-red-500/50 focus:ring-red-500/20'
					: 'focus:border-[#196ee6]/50 focus:ring-[#196ee6]/20'}"
			/>
		</div>
	</div>

	<div class="flex items-center gap-3 border-b border-[#2d3540] bg-[#1e2329] px-3 py-2">
		<Checkbox
			checked={masterChecked}
			indeterminate={masterIndeterminate}
			onCheckedChange={toggleAll}
			class="border-slate-600 bg-[#111821] {localExclude ? 'text-red-500' : 'text-[#196ee6]'}"
		/>
		<span class="text-xs font-semibold text-slate-300">{$t('filters.selectAll')}</span>
	</div>

	<div class="custom-scrollbar max-h-[340px] overflow-y-auto bg-[#1e2329]">
		{#if unifiedStatuses.length > 0}
			<div
				class="sticky top-0 z-10 flex items-center justify-between border-b border-[#2d3540]/50 bg-[#1e2329]/95 px-3 py-2 backdrop-blur"
			>
				<div class="flex items-center gap-2">
					<GitMerge class="h-3.5 w-3.5 text-blue-400" />
					<span class="text-xs font-semibold text-white">{$t('filters.unifiedStatuses')}</span>
				</div>
				<span class="text-[10px] text-slate-500 italic">
					{$t('filters.mergedFrom', { values: { count: Object.keys(metadata).length } })}
				</span>
			</div>
			<div class="space-y-0.5 p-1.5">
				{#each unifiedStatuses as item (item.status.name)}
					{@const isSelected = isStatusSelected(item.status.name)}
					<label
						class="group flex cursor-pointer items-center justify-between rounded-lg px-2 py-2 transition-all {isSelected &&
						localExclude
							? 'border border-red-500/20 bg-red-500/10'
							: isSelected
								? 'border border-[#2d3540]/50 bg-[#2d3540]/40'
								: 'hover:bg-[#2d3540]'}"
					>
						<div class="flex items-center gap-3">
							<Checkbox
								checked={isSelected}
								onCheckedChange={() => toggleStatus(item.status.name)}
								class="border-slate-600 bg-[#111821] {localExclude
									? 'text-red-500'
									: 'text-[#196ee6]'}"
							/>
							<div class="flex items-center gap-2">
								<span class="h-2 w-2 rounded-full bg-gray-400"></span>
								<span
									class="text-sm {isSelected
										? 'font-medium text-white'
										: 'text-slate-200 group-hover:text-white'}"
								>
									{item.status.name}
								</span>
							</div>
						</div>
						<div class="flex items-center gap-2">
							<div
								class="flex -space-x-1 transition-opacity {isSelected
									? 'opacity-100'
									: 'opacity-60 group-hover:opacity-100'}"
							>
								{#each item.projectNames.slice(0, 3) as pName}
									<div class="h-4 w-1.5 rounded-full {getProjectColor(pName)}" title={pName}></div>
								{/each}
							</div>
							<span
								class="ml-1 rounded bg-[#2d3540] px-1.5 text-[10px] {isSelected && localExclude
									? 'border border-red-500/20 bg-red-500/20 text-red-300'
									: 'text-slate-500'}"
							>
								{item.projectIds.length === Object.keys(metadata).length
									? $t('filters.allProjects')
									: `${item.projectIds.length} ${$t('filters.proj')}`}
							</span>
						</div>
					</label>
				{/each}
			</div>
		{/if}

		{#if projectSpecificStatuses.length > 0}
			<div
				class="sticky top-0 z-10 mt-1 flex items-center justify-between border-t border-b border-[#2d3540]/50 bg-[#1e2329]/95 px-3 py-2 backdrop-blur"
			>
				<div class="flex items-center gap-2">
					<Puzzle class="h-3.5 w-3.5 text-purple-400" />
					<span class="text-xs font-semibold text-white">{$t('filters.projectSpecific')}</span>
				</div>
				<span class="text-[10px] text-slate-500 italic">{$t('filters.uniqueWorkflows')}</span>
			</div>
			<div class="space-y-0.5 p-1.5">
				{#each projectSpecificStatuses as item (item.status.name + item.projectIds[0])}
					{@const isSelected = isStatusSelected(item.status.name)}
					<label
						class="group flex cursor-pointer items-center justify-between rounded-lg px-2 py-2 transition-all {isSelected &&
						localExclude
							? 'border border-red-500/20 bg-red-500/10'
							: isSelected
								? 'border border-[#2d3540]/50 bg-[#2d3540]/40'
								: 'hover:bg-[#2d3540]'}"
					>
						<div class="flex items-center gap-3">
							<Checkbox
								checked={isSelected}
								onCheckedChange={() => toggleStatus(item.status.name)}
								class="border-slate-600 bg-[#111821] {localExclude
									? 'text-red-500'
									: 'text-[#196ee6]'}"
							/>
							<div class="flex items-center gap-2">
								<span class="h-2 w-2 rounded-full bg-gray-400"></span>
								<span
									class="text-sm {isSelected
										? 'font-medium text-white'
										: 'text-slate-200 group-hover:text-white'}"
								>
									{item.status.name}
								</span>
							</div>
						</div>
						<div class="flex items-center gap-2">
							<span
								class="rounded border px-1.5 py-0.5 text-[10px] font-medium {getProjectColor(
									item.projectNames[0]
								)
									.replace('bg-', 'bg-')
									.replace('-500', '-500/10')} {getProjectColor(item.projectNames[0])
									.replace('bg-', 'text-')
									.replace('-500', '-400')} border-current/20"
							>
								{item.projectNames[0].substring(0, 10)}
							</span>
						</div>
					</label>
				{/each}
			</div>
		{/if}

		{#if unifiedStatuses.length === 0 && projectSpecificStatuses.length === 0}
			<div class="py-8 text-center text-sm text-slate-500">
				{$t('filters.noStatuses')}
			</div>
		{/if}
	</div>

	<div class="flex items-center justify-between border-t border-[#2d3540] bg-[#111821] p-3">
		<span class="text-xs text-slate-500">
			<strong class="text-white">{selectedCount}</strong>
			{localExclude ? $t('filters.excluded') : $t('filters.included')}
		</span>
		<div class="flex items-center gap-2">
			<button
				class="px-3 py-1.5 text-xs font-medium text-slate-400 transition-colors hover:text-white"
				onclick={handleClear}
			>
				{$t('filters.clear')}
			</button>
			<button
				class="rounded-lg px-3 py-1.5 text-xs font-semibold text-white shadow-lg transition-all {localExclude
					? 'bg-red-500 shadow-red-500/20 hover:bg-red-600'
					: 'bg-[#196ee6] shadow-[#196ee6]/20 hover:bg-blue-600'}"
				onclick={handleApply}
			>
				{$t('filters.apply')}
			</button>
		</div>
	</div>
</Popover.Content>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: #3f4a59;
		border-radius: 2px;
	}
</style>
