<script lang="ts">
	import type { HistoryEntry } from '$lib/types';
	import * as Avatar from '$lib/components/ui/avatar';
	import { t } from 'svelte-i18n';

	let { comments }: { comments: HistoryEntry[] } = $props();

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}

	function formatRelativeDate(dateStr: string): string {
		const date = new Date(dateStr);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
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
</script>

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
								{@html comment.comment_html}
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
