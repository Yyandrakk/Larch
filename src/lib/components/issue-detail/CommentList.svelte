<script lang="ts">
	import type { HistoryEntry } from '$lib/types';
	import * as Avatar from '$lib/components/ui/avatar';
	import { t } from 'svelte-i18n';
	import MarkdownEditor from '$lib/components/common/MarkdownEditor.svelte';
	import { Separator } from '$lib/components/ui/separator';
	import { transformImageUrls } from '$lib/utils/image-auth';
	import { sanitizeHtml } from '$lib/sanitize';
	import { getCurrentUser } from '$lib/stores/user.svelte';
	import { CMD_EDIT_ISSUE_COMMENT, CMD_DELETE_ISSUE_COMMENT } from '$lib/commands.svelte';
	import { Pencil, Trash2, Check, X, Loader2 } from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { Button } from '$lib/components/ui/button';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { toast } from 'svelte-sonner';

	let {
		comments = $bindable(),
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

	let editingId = $state<string | null>(null);
	let editingText = $state('');
	let isSubmittingEdit = $state(false);
	let deletingId = $state<string | null>(null);
	let isDeleting = $state(false);

	const currentUser = $derived(getCurrentUser());

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

	function startEdit(comment: HistoryEntry) {
		editingId = comment.id;
		editingText = comment.comment || '';
	}

	function cancelEdit() {
		editingId = null;
		editingText = '';
	}

	async function saveEdit() {
		if (!editingId || !editingText.trim()) return;

		isSubmittingEdit = true;
		try {
			await invoke(CMD_EDIT_ISSUE_COMMENT, {
				commentId: editingId,
				comment: editingText
			});

			// Optimistic update
			const index = comments.findIndex((c) => c.id === editingId);
			if (index !== -1) {
				// We keep the object reference but update properties for Svelte 5 reactivity
				// But wait, `comments` is an array. If it's a bindable prop, we should be careful.
				// However, simplistic update:
				const updated = { ...comments[index] };
				updated.comment = editingText;
				// Clear HTML so it falls back to text or needs re-fetch.
				//Ideally we'd get the new HTML back but for now this works.
				updated.comment_html = undefined;
				updated.is_edited = true;

				const newComments = [...comments];
				newComments[index] = updated;
				comments = newComments;
			}

			toast.success($t('issueDetail.commentUpdated') || 'Comment updated');
			editingId = null;
			editingText = '';
		} catch (error) {
			console.error('Failed to edit comment:', error);
			toast.error($t('errors.unknown'));
		} finally {
			isSubmittingEdit = false;
		}
	}

	function startDelete(id: string) {
		deletingId = id;
	}

	async function confirmDelete() {
		if (!deletingId) return;

		isDeleting = true;
		try {
			await invoke(CMD_DELETE_ISSUE_COMMENT, { commentId: deletingId });

			// Optimistic remove
			comments = comments.filter((c) => c.id !== deletingId);

			toast.success($t('issueDetail.commentDeleted') || 'Comment deleted');
		} catch (error) {
			console.error('Failed to delete comment:', error);
			toast.error($t('errors.unknown'));
		} finally {
			isDeleting = false;
			deletingId = null;
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
			<div
				class="border-border flex gap-3 rounded-lg border p-3 transition-colors {comment.user_id ===
				currentUser?.id
					? 'bg-accent/30'
					: ''}"
			>
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
					<div class="flex items-center justify-between">
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

						{#if comment.user_id === currentUser?.id && editingId !== comment.id}
							<div
								class="flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100"
							>
								<Button
									variant="ghost"
									size="icon"
									class="text-muted-foreground hover:text-foreground h-6 w-6"
									onclick={() => startEdit(comment)}
									title={$t('common.edit') || 'Edit'}
								>
									<Pencil class="h-3.5 w-3.5" />
								</Button>
								<Button
									variant="ghost"
									size="icon"
									class="text-muted-foreground hover:text-destructive h-6 w-6"
									onclick={() => startDelete(comment.id)}
									title={$t('common.delete') || 'Delete'}
								>
									<Trash2 class="h-3.5 w-3.5" />
								</Button>
							</div>
						{/if}
					</div>

					{#if editingId === comment.id}
						<div class="mt-2 space-y-2">
							<textarea
								bind:value={editingText}
								class="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[100px] w-full resize-none rounded-lg border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
								rows={4}
							></textarea>
							<div class="flex justify-end gap-2">
								<Button
									variant="ghost"
									size="sm"
									class="h-8 w-8 p-0"
									onclick={cancelEdit}
									disabled={isSubmittingEdit}
								>
									<X class="h-4 w-4" />
								</Button>
								<Button
									variant="default"
									size="sm"
									class="h-8 w-8 p-0"
									onclick={saveEdit}
									disabled={isSubmittingEdit || !editingText.trim()}
								>
									{#if isSubmittingEdit}
										<Loader2 class="h-4 w-4 animate-spin" />
									{:else}
										<Check class="h-4 w-4" />
									{/if}
								</Button>
							</div>
						</div>
					{:else}
						<div class="group mt-1 text-sm">
							{#if comment.comment_html}
								<div class="prose prose-sm dark:prose-invert max-w-none">
									<!-- eslint-disable-next-line svelte/no-at-html-tags -->
									{@html transformImageUrls(sanitizeHtml(comment.comment_html))}
								</div>
							{:else if comment.comment}
								<p class="whitespace-pre-wrap">{comment.comment}</p>
							{/if}
						</div>
					{/if}
				</div>
			</div>
		{/each}
	</div>
{/if}

<AlertDialog.Root open={!!deletingId} onOpenChange={(open) => !open && (deletingId = null)}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title
				>{$t('issueDetail.deleteCommentTitle') || 'Delete comment?'}</AlertDialog.Title
			>
			<AlertDialog.Description>
				{$t('issueDetail.deleteCommentDescription') ||
					'This action cannot be undone. This will permanently delete your comment.'}
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={isDeleting}>{$t('common.cancel')}</AlertDialog.Cancel>
			<AlertDialog.Action
				class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
				onclick={confirmDelete}
				disabled={isDeleting}
			>
				{#if isDeleting}
					<Loader2 class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				{$t('common.delete')}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
