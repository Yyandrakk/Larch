<script lang="ts">
	import type { IssueType } from '$lib/types';
	import * as Select from '$lib/components/ui/select';
	import { Badge } from '$lib/components/ui/badge';
	import { Loader2 } from '@lucide/svelte';

	let {
		currentTypeId,
		issueTypes,
		updating = false,
		disabled = false,
		onTypeChange
	}: {
		currentTypeId?: number;
		issueTypes: IssueType[];
		updating?: boolean;
		disabled?: boolean;
		onTypeChange?: (typeId: number) => void;
	} = $props();

	let currentType = $derived(issueTypes.find((t) => t.id === currentTypeId));
	let canEdit = $derived(issueTypes.length > 0 && onTypeChange !== undefined);

	function handleChange(value: string | undefined) {
		if (value && onTypeChange) {
			onTypeChange(parseInt(value, 10));
		}
	}
</script>

<div>
	<span class="text-muted-foreground mb-1 block text-sm">Type</span>
	{#if canEdit}
		{#key currentTypeId}
			<Select.Root
				type="single"
				value={currentTypeId?.toString() ?? ''}
				onValueChange={handleChange}
				disabled={updating || disabled}
			>
				<Select.Trigger
					class="h-8 w-full"
					style={currentType?.color ? `border-color: ${currentType.color}` : ''}
				>
					{#if updating}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
					{/if}
					{#if currentType}
						<span style="color: {currentType.color}">{currentType.name}</span>
					{:else}
						<span class="text-muted-foreground italic">Not set</span>
					{/if}
				</Select.Trigger>
				<Select.Content>
					{#each issueTypes as issueType (issueType.id)}
						<Select.Item value={issueType.id.toString()}>
							<div class="flex items-center gap-2">
								<div class="h-3 w-3 rounded-full" style="background-color: {issueType.color}"></div>
								<span>{issueType.name}</span>
							</div>
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		{/key}
	{:else if currentType}
		<Badge variant="outline" style="border-color: {currentType.color}; color: {currentType.color}">
			{currentType.name}
		</Badge>
	{:else}
		<span class="text-muted-foreground text-sm italic">Not set</span>
	{/if}
</div>
