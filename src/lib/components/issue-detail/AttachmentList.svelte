<script lang="ts">
	import type { Attachment } from '$lib/types';
	import { Button } from '$lib/components/ui/button';
	import { File, Image, Download, ExternalLink } from '@lucide/svelte';
	import { openUrl } from '@tauri-apps/plugin-opener';

	let { attachments }: { attachments: Attachment[] } = $props();

	async function openAttachment(url: string) {
		try {
			await openUrl(url);
		} catch (e) {
			console.error('Failed to open attachment URL:', e);
		}
	}
</script>

<div class="space-y-2">
	{#each attachments as attachment (attachment.id)}
		<div
			class="bg-secondary/50 hover:bg-secondary/70 group flex items-center gap-3 rounded-lg p-3 transition-colors"
		>
			<!-- Icon/Thumbnail -->
			{#if attachment.is_image && attachment.thumbnail_url}
				<img
					src={attachment.thumbnail_url}
					alt={attachment.name}
					class="h-10 w-10 rounded object-cover"
				/>
			{:else if attachment.is_image}
				<div class="bg-secondary flex h-10 w-10 items-center justify-center rounded">
					<Image class="text-muted-foreground h-5 w-5" />
				</div>
			{:else}
				<div class="bg-secondary flex h-10 w-10 items-center justify-center rounded">
					<File class="text-muted-foreground h-5 w-5" />
				</div>
			{/if}

			<!-- File Info -->
			<div class="min-w-0 flex-1">
				<p class="truncate text-sm font-medium">{attachment.name}</p>
				<p class="text-muted-foreground text-xs">{attachment.size_display}</p>
			</div>

			<!-- Actions -->
			<Button
				variant="ghost"
				size="icon"
				class="opacity-0 transition-opacity group-hover:opacity-100 focus-visible:opacity-100"
				onclick={() => openAttachment(attachment.url)}
				aria-label="Open attachment in new tab"
			>
				<ExternalLink class="h-4 w-4" />
			</Button>
		</div>
	{/each}
</div>
