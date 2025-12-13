<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { CMD_GET_ISSUE_DETAIL, CMD_GET_ISSUE_HISTORY } from '$lib/commands.svelte';
	import type { IssueDetail, HistoryEntry } from '$lib/types';
	import * as Sheet from '$lib/components/ui/sheet';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { t } from 'svelte-i18n';
	import { toast } from 'svelte-sonner';
	import { Paperclip, MessageSquare } from '@lucide/svelte';
	import IssueMetadata from './IssueMetadata.svelte';
	import TagList from './TagList.svelte';
	import AttachmentList from './AttachmentList.svelte';
	import CommentList from './CommentList.svelte';

	let {
		issueId = $bindable<number | null>(null),
		open = $bindable(false)
	}: {
		issueId: number | null;
		open: boolean;
	} = $props();

	let issue = $state<IssueDetail | null>(null);
	let history = $state<HistoryEntry[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);

	// Watch for issueId changes to load data
	$effect(() => {
		if (issueId && open) {
			loadIssueData(issueId);
		}
	});

	async function loadIssueData(id: number) {
		loading = true;
		error = null;
		issue = null;
		history = [];

		try {
			// Fetch issue detail and history in parallel
			const [issueResult, historyResult] = await Promise.all([
				invoke<IssueDetail>(CMD_GET_ISSUE_DETAIL, { issueId: id }),
				invoke<HistoryEntry[]>(CMD_GET_ISSUE_HISTORY, { issueId: id })
			]);

			issue = issueResult;
			history = historyResult;
		} catch (e) {
			console.error('Failed to load issue:', e);
			// Better error serialization
			if (typeof e === 'string') {
				error = e;
			} else if (e instanceof Error) {
				error = e.message;
			} else if (e && typeof e === 'object' && 'message' in e) {
				error = String((e as { message: unknown }).message);
			} else {
				try {
					error = JSON.stringify(e, null, 2);
				} catch {
					error = 'An unknown error occurred';
				}
			}
			toast.error($t('issueDetail.error'));
		} finally {
			loading = false;
		}
	}

	// Filter history to only show comments
	let comments = $derived(
		history.filter((h) => h.entry_type === 'comment' && h.comment && !h.is_deleted)
	);
</script>

<Sheet.Root bind:open>
	<Sheet.Content
		class="flex h-full w-full flex-col overflow-hidden sm:max-w-md md:max-w-lg lg:max-w-2xl"
		side="right"
	>
		{#if loading}
			<!-- Loading State -->
			<Sheet.Header class="flex-shrink-0 border-b pb-4">
				<Skeleton class="h-6 w-32" />
				<Skeleton class="mt-2 h-8 w-full" />
			</Sheet.Header>
			<div class="flex-1 space-y-4 overflow-y-auto p-6">
				<Skeleton class="h-24 w-full" />
				<Skeleton class="h-32 w-full" />
				<Skeleton class="h-48 w-full" />
			</div>
		{:else if error}
			<!-- Error State -->
			<Sheet.Header class="flex-shrink-0 border-b pb-4">
				<Sheet.Title class="text-destructive">{$t('issueDetail.error')}</Sheet.Title>
			</Sheet.Header>
			<div class="flex flex-1 items-center justify-center p-6">
				<div class="space-y-4 text-center">
					<p class="text-muted-foreground max-w-sm text-sm break-words">{error}</p>
					<Button onclick={() => issueId && loadIssueData(issueId)}>
						{$t('common.retry') || 'Retry'}
					</Button>
				</div>
			</div>
		{:else if issue}
			<!-- Issue Detail -->
			<Sheet.Header class="flex-shrink-0 border-b pb-4">
				<div class="flex items-center gap-2">
					<span class="text-muted-foreground font-mono">#{issue.ref_number}</span>
					<Badge variant={issue.is_closed ? 'secondary' : 'default'} class="text-xs">
						{issue.is_closed ? '✓ Closed' : '● Open'}
					</Badge>
				</div>
				<Sheet.Title class="mt-2 text-xl leading-tight font-semibold">
					{issue.subject}
				</Sheet.Title>
			</Sheet.Header>

			<!-- Scrollable content area -->
			<div class="flex-1 overflow-y-auto">
				<div class="space-y-6 p-6">
					<!-- Metadata Section -->
					<IssueMetadata {issue} />

					<!-- Tags -->
					{#if issue.tags.length > 0}
						<div>
							<h3 class="mb-2 text-sm font-medium">Tags</h3>
							<TagList tags={issue.tags} />
						</div>
					{/if}

					<!-- Attachments -->
					{#if issue.attachments.length > 0}
						<div>
							<h3 class="mb-2 flex items-center gap-2 text-sm font-medium">
								<Paperclip class="h-4 w-4" />
								Attachments ({issue.attachments.length})
							</h3>
							<AttachmentList attachments={issue.attachments} />
						</div>
					{/if}

					<!-- Description -->
					<div>
						<h3 class="mb-2 text-sm font-medium">Description</h3>
						{#if issue.description_html}
							<div class="prose prose-sm dark:prose-invert bg-muted/30 max-w-none rounded-lg p-4">
								{@html issue.description_html}
							</div>
						{:else if issue.description}
							<div class="bg-muted/30 rounded-lg p-4 text-sm whitespace-pre-wrap">
								{issue.description}
							</div>
						{:else}
							<p class="text-muted-foreground text-sm italic">
								{$t('issueDetail.noDescription') || 'No description provided'}
							</p>
						{/if}
					</div>

					<!-- Comments -->
					<div>
						<h3 class="mb-2 flex items-center gap-2 text-sm font-medium">
							<MessageSquare class="h-4 w-4" />
							Comments ({comments.length})
						</h3>
						<CommentList {comments} />
					</div>
				</div>
			</div>
		{/if}
	</Sheet.Content>
</Sheet.Root>
