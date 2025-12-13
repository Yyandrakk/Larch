<script lang="ts">
	import type { Tag } from '$lib/types';
	import { Badge } from '$lib/components/ui/badge';

	let { tags }: { tags: Tag[] } = $props();
</script>

<div class="flex flex-wrap gap-2">
	{#each tags as tag (tag.name)}
		<Badge
			variant="secondary"
			style={tag.color && /^#[0-9A-Fa-f]{6}$/.test(tag.color)
				? `background-color: ${tag.color}20; border-color: ${tag.color}; color: ${tag.color}`
				: ''}
			class="text-xs"
		>
			{tag.name}
		</Badge>
	{/each}
</div>
