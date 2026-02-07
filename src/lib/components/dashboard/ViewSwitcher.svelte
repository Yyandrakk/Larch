<script lang="ts">
	import { fade } from 'svelte/transition';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { Check, ChevronDown, Lock, Trash2 } from '@lucide/svelte';
	import type { SavedView } from '$lib/types';
	import { t } from 'svelte-i18n';

	let {
		views,
		currentView,
		isDirty,
		onSelectView,
		onDeleteView
	}: {
		views: SavedView[];
		currentView: SavedView | null;
		isDirty: boolean;
		onSelectView: (id: number) => void;
		onDeleteView?: (id: number) => void;
	} = $props();

	let sortedViews = $derived(
		[...views].sort((a, b) => {
			if (a.last_used === b.last_used) return 0;
			if (!a.last_used) return 1;
			if (!b.last_used) return -1;
			return new Date(b.last_used).getTime() - new Date(a.last_used).getTime();
		})
	);
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button
				variant="outline"
				class="w-[200px] justify-between"
				{...props}
				data-testid="view-switcher-trigger"
			>
				<div class="flex items-center gap-2 truncate">
					<span class="truncate">
						{currentView ? currentView.name : $t('views.selectView')}
					</span>
					{#if isDirty}
						<div
							transition:fade={{ duration: 200 }}
							class="h-2 w-2 shrink-0 animate-pulse rounded-full bg-orange-500"
							title={$t('views.unsavedChanges')}
						></div>
					{/if}
				</div>
				<ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content class="w-[200px]" align="start">
		<DropdownMenu.Group>
			<DropdownMenu.Label>{$t('views.title')}</DropdownMenu.Label>
			<DropdownMenu.Separator />
			{#each sortedViews as view (view.id)}
				<DropdownMenu.Item
					onclick={() => onSelectView(view.id)}
					data-testid={`view-item-${view.id}`}
					class="group flex items-center justify-between"
				>
					<span class="truncate pr-2 font-medium">
						{view.name}
					</span>
					<div class="flex items-center gap-2">
						{#if view.is_system}
							<Lock class="text-muted-foreground h-3 w-3" />
						{:else if onDeleteView}
							<button
								class="text-muted-foreground hover:text-destructive hidden group-hover:block"
								onclick={(e) => {
									e.stopPropagation();
									onDeleteView(view.id);
								}}
								data-testid={`delete-view-${view.id}`}
								aria-label={$t('common.delete')}
							>
								<Trash2 class="h-3 w-3" />
							</button>
						{/if}
						{#if currentView?.id === view.id}
							<Check class="h-4 w-4" />
						{/if}
					</div>
				</DropdownMenu.Item>
			{/each}
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
