<script lang="ts">
	import type { Priority } from '$lib/types';
	import * as Select from '$lib/components/ui/select';
	import { Badge } from '$lib/components/ui/badge';
	import { Loader2 } from '@lucide/svelte';

	let {
		currentPriorityId,
		priorities,
		updating = false,
		disabled = false,
		onPriorityChange
	}: {
		currentPriorityId?: number;
		priorities: Priority[];
		updating?: boolean;
		disabled?: boolean;
		onPriorityChange?: (priorityId: number) => void;
	} = $props();

	let currentPriority = $derived(priorities.find((p) => p.id === currentPriorityId));
	let canEdit = $derived(priorities.length > 0 && onPriorityChange !== undefined);

	function handleChange(value: string | undefined) {
		if (value && onPriorityChange) {
			onPriorityChange(parseInt(value, 10));
		}
	}
</script>

<div>
	<span class="text-muted-foreground mb-1 block text-sm">Priority</span>
	{#if canEdit}
		{#key currentPriorityId}
			<Select.Root
				type="single"
				value={currentPriorityId?.toString() ?? ''}
				onValueChange={handleChange}
				disabled={updating || disabled}
			>
				<Select.Trigger
					class="h-8 w-full"
					style={currentPriority?.color ? `border-color: ${currentPriority.color}` : ''}
				>
					{#if updating}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
					{/if}
					{#if currentPriority}
						<span style="color: {currentPriority.color}">{currentPriority.name}</span>
					{:else}
						<span class="text-muted-foreground italic">Not set</span>
					{/if}
				</Select.Trigger>
				<Select.Content>
					{#each priorities as priority (priority.id)}
						<Select.Item value={priority.id.toString()}>
							<div class="flex items-center gap-2">
								<div class="h-3 w-3 rounded-full" style="background-color: {priority.color}"></div>
								<span>{priority.name}</span>
							</div>
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		{/key}
	{:else if currentPriority}
		<Badge
			variant="outline"
			style="border-color: {currentPriority.color}; color: {currentPriority.color}"
		>
			{currentPriority.name}
		</Badge>
	{:else}
		<span class="text-muted-foreground text-sm italic">Not set</span>
	{/if}
</div>
