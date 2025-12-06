<script lang="ts">
	import FilterPopover from './FilterPopover.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { X } from '@lucide/svelte';
	import type { Project, FilterObject, ProjectMetadata } from '$lib/types';

	let {
		projects = [],
		metadata = {},
		filters = {},
		onApply
	} = $props<{
		projects: Project[];
		metadata: Record<number, ProjectMetadata>;
		filters: FilterObject;
		onApply: (filters: FilterObject) => void;
	}>();

	function removeProjectFilter(id: number) {
		const newIds = filters.project_ids?.filter((pid: number) => pid !== id);
		onApply({
			...filters,
			project_ids: newIds && newIds.length > 0 ? newIds : undefined
		});
	}

	function clearFilters() {
		onApply({});
	}

	let activeFilterCount = $derived(
		(filters.project_ids?.length || 0) +
			(filters.status_ids?.length || 0) +
			(filters.assignee_ids?.length || 0)
	);
</script>

<div class="flex items-center justify-between p-4">
	<div class="flex flex-1 items-center space-x-2">
		<FilterPopover {projects} {metadata} {filters} {onApply} />
		{#if activeFilterCount > 0}
			<Button variant="ghost" onclick={clearFilters} class="h-8 px-2 lg:px-3">
				Reset
				<X class="ml-2 h-4 w-4" />
			</Button>
		{/if}
		<div class="flex gap-2">
			{#if filters.project_ids}
				{#each filters.project_ids as pid}
					{@const project = projects.find((p: Project) => p.id === pid)}
					{#if project}
						<Badge variant="secondary" class="rounded-sm px-1 font-normal">
							{project.name}
							<button
								class="ring-offset-background focus:ring-ring ml-1 rounded-full outline-none focus:ring-2 focus:ring-offset-2"
								onclick={() => removeProjectFilter(pid)}
							>
								<X class="text-muted-foreground hover:text-foreground h-3 w-3" />
							</button>
						</Badge>
					{/if}
				{/each}
			{/if}
		</div>
	</div>
</div>
