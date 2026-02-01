<script lang="ts">
	import type { HistoryEntry } from '$lib/types';
	import * as Avatar from '$lib/components/ui/avatar';
	import { t } from 'svelte-i18n';
	import MarkdownEditor from '$lib/components/common/MarkdownEditor.svelte';
	import { Separator } from '$lib/components/ui/separator';
	import { transformImageUrls } from '$lib/utils/image-auth';
	import DOMPurify from 'dompurify';

	let {
		comments,
		commentText = $bindable(''),
		submitting = false,
		onSubmit,
		onUpload
	}: {
		comments: HistoryEntry[];
		commentText?: string;
		submitting?: boolean;
		onSubmit?: (text: string) => void;
		onUpload?: (file: File) => Promise<string | undefined>;
	} = $props();

	function getInitials(name: string): string {
		return name
			.split(' ')
			.filter((n) => n.length > 0)
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}

	function formatRelativeDate(dateStr: string): string {
		const date = new Date(dateStr);
		if (isNaN(date.getTime())) {
			return 'Invalid date';
		}

		const now = new Date();
		const diffMs = now.getTime() - date.getTime();

		// Handle future dates
		if (diffMs < 0) {
			const absDiff = Math.abs(diffMs);
			const diffSecs = Math.floor(absDiff / 1000);
			const diffMins = Math.floor(diffSecs / 60);
			const diffHours = Math.floor(diffMins / 60);
			const diffDays = Math.floor(diffHours / 24);

			if (diffDays > 0) return `in ${diffDays}d`;
			if (diffHours > 0) return `in ${diffHours}h`;
			if (diffMins > 0) return `in ${diffMins}m`;
			return 'in the future';
		}

		const diffSecs = Math.floor(diffMs / 1000);
		const diffMins = Math.floor(diffSecs / 60);
		const diffHours = Math.floor(diffMins / 60);
		const diffDays = Math.floor(diffHours / 24);

		if (diffDays > 30) {
			return date.toLocaleDateString();
		} else if (diffDays > 0) {
			return `${diffDays}d ago`;
		} else if (diffHours > 0) {
			return `${diffHours}h ago`;
		} else if (diffMins > 0) {
			return `${diffMins}m ago`;
		} else {
			return 'just now';
		}
	}

	function handleSubmit(text: string) {
		if (onSubmit) {
			onSubmit(text);
		}
	}
</script>

<!-- Comment Input at top -->
{#if onSubmit}
	<div class="mb-4">
		<MarkdownEditor
			bind:value={commentText}
			placeholder={$t('issueDetail.commentPlaceholder') ||
				'Write a comment... (Markdown supported)'}
			disabled={false}
			{submitting}
			onSubmit={handleSubmit}
			{onUpload}
		/>
	</div>
	<Separator class="my-4" />
{/if}

<!-- Existing Comments -->
{#if comments.length === 0}
	<div class="text-muted-foreground py-8 text-center">
		<p class="text-sm italic">{$t('issueDetail.noComments') || 'No comments yet'}</p>
	</div>
{:else}
	<div class="space-y-4">
		{#each comments as comment (comment.id)}
			<div class="flex gap-3">
				<!-- Avatar -->
				<Avatar.Root class="h-8 w-8 flex-shrink-0">
					{#if comment.user_photo}
						<Avatar.Image src={comment.user_photo} alt={comment.user_name} />
					{/if}
					<Avatar.Fallback class="text-xs">
						{getInitials(comment.user_name)}
					</Avatar.Fallback>
				</Avatar.Root>

				<!-- Comment Content -->
				<div class="min-w-0 flex-1">
					<!-- Header -->
					<div class="flex items-center gap-2 text-sm">
						<span class="font-medium">{comment.user_name}</span>
						<span class="text-muted-foreground">@{comment.user_username}</span>
						<span class="text-muted-foreground">·</span>
						<span class="text-muted-foreground">
							{formatRelativeDate(comment.created_at)}
						</span>
						{#if comment.is_edited}
							<span class="text-muted-foreground text-xs italic">(edited)</span>
						{/if}
					</div>

					<!-- Comment Body -->
					<div class="mt-1 text-sm">
						{#if comment.comment_html}
							<div class="prose prose-sm dark:prose-invert max-w-none">
								{@html transformImageUrls(DOMPurify.sanitize(comment.comment_html))}
							</div>
						{:else if comment.comment}
							<p class="whitespace-pre-wrap">{comment.comment}</p>
						{/if}
					</div>
				</div>
			</div>
		{/each}
	</div>
{/if}
