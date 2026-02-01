<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { ChevronDown, Lock, Check, Trash2 } from '@lucide/svelte';
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
		onSelectView: (id: string) => void;
		onDeleteView?: (id: string) => void;
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
				<span class="truncate">
					{currentView ? currentView.name : $t('dashboard.views.select_view')}
					{#if isDirty}
						<span class="text-muted-foreground ml-1">*</span>
					{/if}
				</span>
				<ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content class="w-[200px]" align="start">
		<DropdownMenu.Group>
			<DropdownMenu.Label>{$t('dashboard.views.title')}</DropdownMenu.Label>
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
