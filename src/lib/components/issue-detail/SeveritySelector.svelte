<script lang="ts">
	import type { Severity } from '$lib/types';
	import * as Select from '$lib/components/ui/select';
	import { Badge } from '$lib/components/ui/badge';
	import { Loader2 } from '@lucide/svelte';

	let {
		currentSeverityId,
		severities,
		updating = false,
		disabled = false,
		onSeverityChange
	}: {
		currentSeverityId?: number;
		severities: Severity[];
		updating?: boolean;
		disabled?: boolean;
		onSeverityChange?: (severityId: number) => void;
	} = $props();

	let currentSeverity = $derived(severities.find((s) => s.id === currentSeverityId));
	let canEdit = $derived(severities.length > 0 && onSeverityChange !== undefined);

	function handleChange(value: string | undefined) {
		if (value && onSeverityChange) {
			onSeverityChange(parseInt(value, 10));
		}
	}
</script>

<div>
	<span class="text-muted-foreground mb-1 block text-sm">Severity</span>
	{#if canEdit}
		{#key currentSeverityId}
			<Select.Root
				type="single"
				value={currentSeverityId?.toString() ?? ''}
				onValueChange={handleChange}
				disabled={updating || disabled}
			>
				<Select.Trigger
					class="h-8 w-full"
					style={currentSeverity?.color ? `border-color: ${currentSeverity.color}` : ''}
				>
					{#if updating}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
					{/if}
					{#if currentSeverity}
						<span style="color: {currentSeverity.color}">{currentSeverity.name}</span>
					{:else}
						<span class="text-muted-foreground italic">Not set</span>
					{/if}
				</Select.Trigger>
				<Select.Content>
					{#each severities as severity (severity.id)}
						<Select.Item value={severity.id.toString()}>
							<div class="flex items-center gap-2">
								<div class="h-3 w-3 rounded-full" style="background-color: {severity.color}"></div>
								<span>{severity.name}</span>
							</div>
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		{/key}
	{:else if currentSeverity}
		<Badge
			variant="outline"
			style="border-color: {currentSeverity.color}; color: {currentSeverity.color}"
		>
			{currentSeverity.name}
		</Badge>
	{:else}
		<span class="text-muted-foreground text-sm italic">Not set</span>
	{/if}
</div>
