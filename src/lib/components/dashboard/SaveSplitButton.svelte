<script lang="ts">
	import { t } from 'svelte-i18n';
	import { Save, ChevronDown, Plus, Trash2 } from '@lucide/svelte';
	import * as ButtonGroup from '$lib/components/ui/button-group';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

	let {
		canSave,
		isDirty,
		isSystemView,
		onSave,
		onSaveAsNew,
		onDelete
	}: {
		canSave: boolean;
		isDirty: boolean;
		isSystemView: boolean;
		onSave: () => void;
		onSaveAsNew: () => void;
		onDelete: () => void;
	} = $props();
</script>

{#if isSystemView}
	<!-- For system views (Active Triage), show only "Save as new" button -->
	<Button variant="outline" onclick={onSaveAsNew} class="gap-2">
		<Plus class="h-4 w-4" />
		{$t('views.saveAs')}
	</Button>
{:else}
	<!-- For user views, show the full split button -->
	<ButtonGroup.Root>
		<Button variant="outline" disabled={!canSave || !isDirty} onclick={onSave} class="gap-2">
			<Save class="h-4 w-4" />
			{$t('views.save')}
		</Button>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger
				class={buttonVariants({ variant: 'outline', size: 'icon' }) + ' px-2'}
				aria-label="More options"
			>
				<ChevronDown class="h-4 w-4" />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="end">
				<DropdownMenu.Item onclick={onSaveAsNew}>
					<Plus class="mr-2 h-4 w-4" />
					{$t('views.saveAs')}
				</DropdownMenu.Item>
				<DropdownMenu.Separator />
				<DropdownMenu.Item
					onclick={onDelete}
					class="text-destructive focus:bg-destructive/10 focus:text-destructive"
				>
					<Trash2 class="mr-2 h-4 w-4" />
					{$t('views.delete')}
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</ButtonGroup.Root>
{/if}
