<script lang="ts">
	import type { Tag, TagColor } from '$lib/types';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Popover from '$lib/components/ui/popover';
	import { Plus, X, Loader2 } from '@lucide/svelte';
	import { t } from 'svelte-i18n';

	let {
		tags = [],
		availableColors = [],
		updating = false,
		disabled = false,
		onTagsChange
	}: {
		tags?: Tag[];
		availableColors?: TagColor[];
		updating?: boolean;
		disabled?: boolean;
		onTagsChange?: (tags: Tag[]) => void;
	} = $props();

	let newTagName = $state('');
	let popoverOpen = $state(false);

	function getTagColor(tag: Tag): string {
		if (tag.color) return tag.color;
		const found = availableColors.find((tc) => tc.name === tag.name);
		return found?.color || '#6b7280';
	}

	function handleRemoveTag(tagName: string) {
		if (!onTagsChange || updating || disabled) return;
		const newTags = tags.filter((t) => t.name !== tagName);
		onTagsChange(newTags);
	}

	function handleAddTag() {
		if (!onTagsChange || updating || disabled || !newTagName.trim()) return;

		const trimmedName = newTagName.trim();
		if (tags.some((t) => t.name.toLowerCase() === trimmedName.toLowerCase())) {
			newTagName = '';
			return;
		}

		const existingColor = availableColors.find(
			(tc) => tc.name.toLowerCase() === trimmedName.toLowerCase()
		);
		const newTag: Tag = {
			name: trimmedName,
			color: existingColor?.color || undefined
		};

		onTagsChange([...tags, newTag]);
		newTagName = '';
		popoverOpen = false;
	}

	function handleSelectExistingTag(tagColor: TagColor) {
		if (!onTagsChange || updating || disabled) return;
		if (tags.some((t) => t.name.toLowerCase() === tagColor.name.toLowerCase())) {
			return;
		}

		const newTag: Tag = {
			name: tagColor.name,
			color: tagColor.color || undefined
		};

		onTagsChange([...tags, newTag]);
		popoverOpen = false;
	}

	let availableToAdd = $derived(
		availableColors.filter(
			(tc) => !tags.some((t) => t.name.toLowerCase() === tc.name.toLowerCase())
		)
	);

	let canEdit = $derived(onTagsChange !== undefined);
</script>

<div class="space-y-2">
	<div class="flex flex-wrap gap-1.5">
		{#each tags as tag (tag.name)}
			<Badge
				variant="secondary"
				class="flex items-center gap-1 pr-1"
				style="background-color: {getTagColor(tag)}20; border-color: {getTagColor(
					tag
				)}; color: {getTagColor(tag)}"
			>
				<span class="text-xs">{tag.name}</span>
				{#if canEdit && !disabled}
					<button
						type="button"
						class="hover:bg-foreground/10 ml-0.5 rounded-full p-0.5"
						onclick={() => handleRemoveTag(tag.name)}
						disabled={updating}
					>
						<X class="h-3 w-3" />
					</button>
				{/if}
			</Badge>
		{/each}

		{#if canEdit && !disabled}
			<Popover.Root bind:open={popoverOpen}>
				<Popover.Trigger>
					<Button variant="ghost" size="sm" class="h-6 gap-1 px-2 text-xs" disabled={updating}>
						{#if updating}
							<Loader2 class="h-3 w-3 animate-spin" />
						{:else}
							<Plus class="h-3 w-3" />
						{/if}
						{$t('issueDetail.addLabel') || 'Add'}
					</Button>
				</Popover.Trigger>
				<Popover.Content class="w-64 p-3" align="start">
					<div class="space-y-3">
						<div class="flex gap-2">
							<Input
								type="text"
								placeholder={$t('issueDetail.newLabelPlaceholder') || 'New label...'}
								class="h-8 text-sm"
								bind:value={newTagName}
								onkeydown={(e) => e.key === 'Enter' && handleAddTag()}
							/>
							<Button size="sm" class="h-8" onclick={handleAddTag} disabled={!newTagName.trim()}>
								{$t('common.add') || 'Add'}
							</Button>
						</div>

						{#if availableToAdd.length > 0}
							<div class="space-y-1">
								<span class="text-muted-foreground text-xs">
									{$t('issueDetail.existingLabels') || 'Existing labels'}
								</span>
								<div class="flex max-h-32 flex-wrap gap-1 overflow-y-auto">
									{#each availableToAdd as tagColor (tagColor.name)}
										<button
											type="button"
											class="hover:bg-muted rounded px-2 py-1 text-xs transition-colors"
											style="color: {tagColor.color || '#6b7280'}"
											onclick={() => handleSelectExistingTag(tagColor)}
										>
											{tagColor.name}
										</button>
									{/each}
								</div>
							</div>
						{/if}
					</div>
				</Popover.Content>
			</Popover.Root>
		{/if}
	</div>

	{#if tags.length === 0 && !canEdit}
		<span class="text-muted-foreground text-sm italic">
			{$t('issueDetail.noLabels') || 'No labels'}
		</span>
	{/if}
</div>
