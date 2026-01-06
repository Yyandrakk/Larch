<script lang="ts">
	import type { Attachment } from '$lib/types';
	import { Button } from '$lib/components/ui/button';
	import { Paperclip, Upload, Trash2, Download, Loader2, FileIcon } from '@lucide/svelte';
	import { t } from 'svelte-i18n';

	let {
		attachments,
		uploading = false,
		disabled = false,
		onUpload,
		onDelete
	}: {
		attachments: Attachment[];
		uploading?: boolean;
		disabled?: boolean;
		onUpload?: (fileName: string, fileData: Uint8Array) => void;
		onDelete?: (attachmentId: number) => void;
	} = $props();

	let fileInput: HTMLInputElement;
	let deletingId = $state<number | null>(null);

	function handleFileSelect(e: Event) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];
		if (!file || !onUpload) return;

		const reader = new FileReader();
		reader.onload = () => {
			const arrayBuffer = reader.result as ArrayBuffer;
			const uint8Array = new Uint8Array(arrayBuffer);
			onUpload(file.name, uint8Array);
		};
		reader.readAsArrayBuffer(file);

		target.value = '';
	}

	async function handleDelete(attachmentId: number) {
		if (!onDelete || deletingId !== null) return;
		deletingId = attachmentId;
		try {
			onDelete(attachmentId);
		} finally {
			deletingId = null;
		}
	}

	function handleDownload(attachment: Attachment) {
		window.open(attachment.url, '_blank');
	}

	let canEdit = $derived(onUpload !== undefined || onDelete !== undefined);
</script>

<div class="space-y-2">
	<div class="flex items-center justify-between">
		<h4 class="flex items-center gap-2 text-sm font-medium">
			<Paperclip class="h-4 w-4" />
			{$t('issueDetail.attachments') || 'Attachments'} ({attachments.length})
		</h4>

		{#if onUpload && !disabled}
			<Button
				variant="ghost"
				size="sm"
				class="h-7 gap-1 text-xs"
				onclick={() => fileInput.click()}
				disabled={uploading}
			>
				{#if uploading}
					<Loader2 class="h-3 w-3 animate-spin" />
					{$t('issueDetail.uploading') || 'Uploading...'}
				{:else}
					<Upload class="h-3 w-3" />
					{$t('issueDetail.upload') || 'Upload'}
				{/if}
			</Button>
			<input type="file" class="hidden" bind:this={fileInput} onchange={handleFileSelect} />
		{/if}
	</div>

	{#if attachments.length === 0}
		<p class="text-muted-foreground text-sm italic">
			{$t('issueDetail.noAttachments') || 'No attachments'}
		</p>
	{:else}
		<div class="space-y-1.5">
			{#each attachments as attachment (attachment.id)}
				<div class="bg-muted/30 flex items-center justify-between gap-2 rounded-lg p-2">
					<div class="flex min-w-0 items-center gap-2">
						{#if attachment.is_image && attachment.thumbnail_url}
							<img
								src={attachment.thumbnail_url}
								alt={attachment.name}
								class="h-10 w-10 rounded object-cover"
							/>
						{:else}
							<div class="bg-muted flex h-10 w-10 items-center justify-center rounded">
								<FileIcon class="text-muted-foreground h-5 w-5" />
							</div>
						{/if}

						<div class="min-w-0 flex-1">
							<p class="truncate text-sm font-medium">{attachment.name}</p>
							<p class="text-muted-foreground text-xs">{attachment.size_display}</p>
						</div>
					</div>

					<div class="flex items-center gap-1">
						<Button
							variant="ghost"
							size="icon"
							class="h-7 w-7"
							onclick={() => handleDownload(attachment)}
							title={$t('issueDetail.download') || 'Download'}
						>
							<Download class="h-3.5 w-3.5" />
						</Button>

						{#if onDelete && !disabled}
							<Button
								variant="ghost"
								size="icon"
								class="text-destructive hover:text-destructive h-7 w-7"
								onclick={() => handleDelete(attachment.id)}
								disabled={deletingId === attachment.id}
								title={$t('issueDetail.delete') || 'Delete'}
							>
								{#if deletingId === attachment.id}
									<Loader2 class="h-3.5 w-3.5 animate-spin" />
								{:else}
									<Trash2 class="h-3.5 w-3.5" />
								{/if}
							</Button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
