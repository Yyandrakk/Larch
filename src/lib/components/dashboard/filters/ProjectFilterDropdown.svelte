<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Search, FolderOpen, Combine, Ban } from '@lucide/svelte';
	import { t } from 'svelte-i18n';
	import type { Project } from '$lib/types';

	let {
		projects = [],
		selectedIds = [],
		isExclude = false,
		onApply,
		customAnchor = null
	}: {
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

	// Sync local state when props change
	$effect(() => {
		localSelectedIds = [...selectedIds];
		localExclude = isExclude;
	});

	let filteredProjects = $derived(
		projects.filter((p) => p.name.toLowerCase().includes(searchQuery.toLowerCase()))
	);

	function toggleProject(id: number) {
		if (localSelectedIds.includes(id)) {
			localSelectedIds = localSelectedIds.filter((i) => i !== id);
		} else {
			localSelectedIds = [...localSelectedIds, id];
		}
	}

	function handleApply() {
		onApply(localSelectedIds, localExclude);
	}

	function handleClear() {
		localSelectedIds = [];
	}

	// Generate a color from project name/slug for visual distinction
	function getProjectColor(project: Project): string {
		const colors = [
			'bg-emerald-500',
			'bg-purple-500',
			'bg-blue-500',
			'bg-orange-500',
			'bg-pink-500',
			'bg-cyan-500',
			'bg-amber-500',
			'bg-indigo-500'
		];
		const hash = project.name.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
		return colors[hash % colors.length];
	}

	// Generate project slug abbreviation
	function getProjectSlug(project: Project): string {
		if (project.slug) {
			return project.slug.substring(0, 3).toUpperCase();
		}
		return project.name.substring(0, 3).toUpperCase();
	}
</script>

<Popover.Content
	class="flex w-[340px] flex-col overflow-hidden rounded-xl border border-[#2d3540] bg-[#1e2329] p-0 shadow-2xl ring-1 shadow-black/80 ring-white/5"
	align="start"
	sideOffset={8}
	{customAnchor}
>
	<div class="border-b border-[#2d3540] bg-[#111821] p-3">
		<div class="mb-3 flex items-center justify-between">
			<div class="flex items-center gap-2 text-slate-400">
				<FolderOpen class="h-[18px] w-[18px]" />
				<span class="text-xs font-bold tracking-wider uppercase">{$t('filters.projectFilter')}</span
				>
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
				placeholder={$t('filters.searchProjects')}
				bind:value={searchQuery}
				class="w-full rounded-lg border border-[#2d3540] bg-[#1e2329] py-2 pr-3 pl-9 text-sm text-white transition-all placeholder:text-slate-600 focus:ring-1 focus:outline-none {localExclude
					? 'focus:border-red-500/50 focus:ring-red-500/20'
					: 'focus:border-[#196ee6]/50 focus:ring-[#196ee6]/20'}"
			/>
		</div>
	</div>

	<div class="custom-scrollbar max-h-[300px] space-y-0.5 overflow-y-auto bg-[#1e2329] p-1.5">
		{#each filteredProjects as project (project.id)}
			{@const isSelected = localSelectedIds.includes(project.id)}
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
						onCheckedChange={() => toggleProject(project.id)}
						class="border-slate-600 bg-[#111821] {localExclude ? 'text-red-500' : 'text-[#196ee6]'}"
					/>
					<div class="flex items-center gap-2">
						<span
							class="h-2 w-2 rounded-full {getProjectColor(project)} {isSelected
								? 'shadow-[0_0_8px_rgba(16,185,129,0.3)]'
								: ''}"
						></span>
						<span
							class="text-sm font-medium {isSelected
								? 'text-white'
								: 'text-slate-300 group-hover:text-white'}"
						>
							{project.name}
						</span>
					</div>
				</div>
				<div class="flex items-center gap-2">
					<span
						class="rounded bg-[#2d3540] px-1.5 font-mono text-[10px] {isSelected
							? 'text-slate-400'
							: 'text-slate-600'}"
					>
						{getProjectSlug(project)}
					</span>
				</div>
			</label>
		{/each}
		{#if filteredProjects.length === 0}
			<div class="py-4 text-center text-sm text-slate-500">
				{$t('projects.noProjects')}
			</div>
		{/if}
	</div>

	<div class="flex items-center justify-between border-t border-[#2d3540] bg-[#111821] p-3">
		<span class="text-xs text-slate-500">
			<strong class="text-white">{localSelectedIds.length}</strong>
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
