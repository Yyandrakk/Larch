<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { t } from 'svelte-i18n';
	import { Search, Users, Globe, Folder, Loader2 } from '@lucide/svelte';
	import { Switch } from '$lib/components/ui/switch';
	import { toast } from 'svelte-sonner';

	type Project = {
		id: number;
		name: string;
		description: string;
		slug: string;
	};

	let { onContinue } = $props<{ onContinue: () => void }>();

	let projects = $state<Project[]>([]);
	let selectedProjectIds = $state<number[]>([]);
	let isLoading = $state(true);
	let isSaving = $state(false);
	let searchQuery = $state('');

	let filteredProjects = $derived(
		projects.filter(
			(p) =>
				searchQuery === '' ||
				p.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				p.slug.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);

	let selectedCount = $derived(selectedProjectIds.length);

	onMount(async () => {
		try {
			const [allProjects, savedIds] = await Promise.all([
				invoke<Project[]>('get_projects'),
				invoke<number[]>('get_selected_projects')
			]);
			projects = allProjects;
			selectedProjectIds = savedIds;
		} catch (error) {
			console.error('Failed to load projects:', error);
			toast.error($t('errors.unknown'));
		} finally {
			isLoading = false;
		}
	});

	async function handleSave() {
		isSaving = true;
		try {
			const idsToSave = selectedProjectIds.map((id) => Number(id));
			await invoke('save_selected_projects', { projectIds: idsToSave });
			toast.success($t('projects.saved'));
			onContinue();
		} catch (error) {
			console.error('Failed to save selection:', error);
			toast.error($t('errors.unknown'));
		} finally {
			isSaving = false;
		}
	}

	function toggleProject(projectId: number) {
		if (selectedProjectIds.includes(projectId)) {
			selectedProjectIds = selectedProjectIds.filter((id) => id !== projectId);
		} else {
			selectedProjectIds = [...selectedProjectIds, projectId];
		}
	}

	function isSelected(projectId: number): boolean {
		return selectedProjectIds.includes(projectId);
	}

	function getSlugAbbrev(slug: string): string {
		return slug.slice(0, 8).toUpperCase();
	}
</script>

<div class="flex-1 overflow-y-auto px-8 py-8 pb-28">
	<div class="mx-auto w-full max-w-[1024px]">
		<div class="mb-8 flex flex-col gap-1">
			<div class="flex items-center justify-between">
				<h1 class="text-2xl font-bold tracking-tight text-white">{$t('projects.title')}</h1>
				<div
					class="rounded-full border border-[#243347] bg-[#1c2633] px-3 py-1 text-sm font-medium text-[#93a9c8]"
				>
					{selectedCount}
					{$t('projects.selected')}
				</div>
			</div>
			<p class="text-sm text-[#93a9c8]">{$t('projects.subtitle')}</p>
		</div>

		<div class="mb-6 flex rounded-xl border border-[#243347] bg-[#1c2633] p-1.5">
			<div class="relative flex-1">
				<Search class="absolute top-1/2 left-3 h-5 w-5 -translate-y-1/2 text-[#93a9c8]" />
				<input
					type="text"
					placeholder={$t('projects.searchPlaceholder')}
					bind:value={searchQuery}
					class="w-full rounded-lg border-none bg-transparent p-2 pl-10 text-sm text-white placeholder-[#93a9c8] focus:ring-0"
				/>
			</div>
		</div>

		{#if isLoading}
			<div class="flex justify-center py-12">
				<Loader2 class="h-8 w-8 animate-spin text-[#196ee6]" />
			</div>
		{:else if filteredProjects.length === 0}
			<div class="py-12 text-center text-[#93a9c8]">
				{$t('projects.noProjects')}
			</div>
		{:else}
			<div class="flex flex-col gap-3">
				{#each filteredProjects as project (project.id)}
					<button
						type="button"
						class="group flex w-full cursor-pointer items-center gap-5 rounded-lg border border-[#243347] bg-[#1c2633] px-4 py-3 text-left transition-all hover:bg-[#243347]"
						onclick={() => toggleProject(project.id)}
					>
						<div class="shrink-0">
							<div
								class="flex size-12 items-center justify-center rounded-md border border-[#243347] bg-[#2a3649] text-[#93a9c8]"
							>
								<Folder class="h-6 w-6" />
							</div>
						</div>

						<div class="flex min-w-0 flex-1 flex-col justify-center">
							<div class="mb-1 flex items-center gap-3">
								<p
									class="truncate text-base font-semibold text-white transition-colors group-hover:text-[#196ee6]"
								>
									{project.name}
								</p>
								<span
									class="inline-flex items-center rounded border border-slate-600 bg-slate-700/50 px-1.5 py-0.5 text-[10px] font-bold text-[#93a9c8] uppercase"
								>
									{getSlugAbbrev(project.slug)}
								</span>
							</div>
							<div class="flex items-center gap-4 text-xs text-[#93a9c8]">
								<div class="flex items-center gap-1.5">
									<Globe class="h-4 w-4" />
									<span>{$t('projects.public')}</span>
								</div>
								<div class="flex items-center gap-1.5">
									<Users class="h-4 w-4" />
									<span>{$t('projects.team')}</span>
								</div>
							</div>
						</div>

						<div
							class="shrink-0"
							role="presentation"
							onclick={(e) => e.stopPropagation()}
							onkeydown={(e) => e.stopPropagation()}
						>
							<Switch
								checked={isSelected(project.id)}
								onCheckedChange={() => toggleProject(project.id)}
							/>
						</div>
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>

{#if selectedCount > 0}
	<div
		class="pointer-events-none absolute inset-x-0 bottom-8 z-10 flex items-center justify-center px-6"
	>
		<div
			class="pointer-events-auto flex items-center gap-4 rounded-full border border-[#243347] bg-[#1c2633]/90 px-4 py-2 shadow-xl backdrop-blur-md"
		>
			<span class="pl-2 text-sm font-medium text-[#93a9c8]">
				{selectedCount}
				{$t('projects.projectsSelected')}
			</span>
			<div class="h-4 w-px bg-[#243347]"></div>
			<button
				onclick={handleSave}
				disabled={isSaving}
				class="rounded-full bg-[#196ee6] px-5 py-1.5 text-sm font-bold text-white shadow-lg transition-all hover:bg-blue-600 disabled:opacity-50"
			>
				{isSaving ? $t('common.saving') : $t('projects.saveAndContinue')}
			</button>
		</div>
	</div>
{/if}
