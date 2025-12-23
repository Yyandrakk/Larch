<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import {
		CMD_GET_ISSUE_DETAIL,
		CMD_GET_ISSUE_HISTORY,
		CMD_CHANGE_ISSUE_STATUS,
		CMD_GET_PROJECT_METADATA,
		CMD_ADD_ISSUE_COMMENT,
		CMD_SAVE_LOCAL_DRAFT,
		CMD_GET_LOCAL_DRAFT,
		CMD_DELETE_LOCAL_DRAFT,
		CMD_COMMIT_ISSUE_DESCRIPTION
	} from '$lib/commands.svelte';
	import type { IssueDetail, HistoryEntry, IssueStatus, ProjectMetadata } from '$lib/types';
	import * as Sheet from '$lib/components/ui/sheet';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { t } from 'svelte-i18n';
	import { toast } from 'svelte-sonner';
	import { Paperclip, MessageSquare, RefreshCw, Edit3, X, Save, Loader2 } from '@lucide/svelte';
	import IssueMetadata from './IssueMetadata.svelte';
	import TagList from './TagList.svelte';
	import AttachmentList from './AttachmentList.svelte';
	import CommentList from './CommentList.svelte';

	let {
		issueId = $bindable<number | null>(null),
		open = $bindable(false),
		onIssueUpdated
	}: {
		issueId: number | null;
		open: boolean;
		onIssueUpdated?: () => void;
	} = $props();

	let issue = $state<IssueDetail | null>(null);
	let history = $state<HistoryEntry[]>([]);
	let statuses = $state<IssueStatus[]>([]);
	let loading = $state(false);
	let statusUpdating = $state(false);
	let commentSubmitting = $state(false);
	let commentText = $state('');
	let error = $state<string | null>(null);
	let hasConflict = $state(false);

	// Description editing state
	let isEditingDescription = $state(false);
	let descriptionDraft = $state('');
	let descriptionSaving = $state(false);
	let hasDraft = $state(false);
	let draftSaveTimeout: ReturnType<typeof setTimeout> | null = null;
	const DRAFT_DEBOUNCE_MS = 2000;

	// Watch for issueId changes to load data
	$effect(() => {
		if (issueId && open) {
			loadIssueData(issueId);
		}
	});

	// Clear comment text and reset description edit state when issue changes
	$effect(() => {
		if (issueId) {
			commentText = '';
			isEditingDescription = false;
			descriptionDraft = '';
			hasDraft = false;
			// Check for existing draft
			checkForDraft(issueId);
		}
	});

	async function loadIssueData(id: number) {
		loading = true;
		error = null;
		issue = null;
		history = [];
		statuses = [];
		hasConflict = false;

		try {
			// Fetch issue detail and history in parallel
			const [issueResult, historyResult] = await Promise.all([
				invoke<IssueDetail>(CMD_GET_ISSUE_DETAIL, { issueId: id }),
				invoke<HistoryEntry[]>(CMD_GET_ISSUE_HISTORY, { issueId: id })
			]);

			issue = issueResult;
			history = historyResult;

			// Now fetch project metadata for this issue's project
			if (issue.project_id) {
				try {
					const metadata = await invoke<Record<number, ProjectMetadata>>(CMD_GET_PROJECT_METADATA, {
						projectIds: [issue.project_id]
					});
					if (metadata[issue.project_id]) {
						statuses = metadata[issue.project_id].statuses;
					}
				} catch (metaErr) {
					console.warn('Failed to load project metadata:', metaErr);
					// Don't block the UI, just won't have status dropdown
				}
			}
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

	async function reloadHistory() {
		if (!issueId) return;
		try {
			history = await invoke<HistoryEntry[]>(CMD_GET_ISSUE_HISTORY, { issueId });
		} catch (e) {
			console.warn('Failed to reload history:', e);
		}
	}

	function isVersionConflict(e: unknown): boolean {
		const errorStr = typeof e === 'string' ? e : JSON.stringify(e);
		return errorStr.includes('VersionConflict') || errorStr.includes('version conflict');
	}

	async function handleStatusChange(newStatusId: number) {
		if (!issue) return;

		// Don't do anything if selecting the current status
		if (newStatusId === issue.status_id) return;

		statusUpdating = true;
		hasConflict = false;

		try {
			const updatedIssue = await invoke<IssueDetail>(CMD_CHANGE_ISSUE_STATUS, {
				issueId: issue.id,
				statusId: newStatusId,
				version: issue.version
			});

			// Update local issue state with the response
			issue = updatedIssue;

			// Find the status name for the toast
			const newStatus = statuses.find((s) => s.id === newStatusId);
			toast.success(
				$t('issueDetail.statusUpdated') || `Status updated to ${newStatus?.name || 'new status'}`
			);

			// Notify parent to refresh the issues table
			if (onIssueUpdated) {
				onIssueUpdated();
			}
		} catch (e) {
			console.error('Failed to update status:', e);

			if (isVersionConflict(e)) {
				hasConflict = true;
				toast.error(
					$t('issueDetail.versionConflict') ||
						'This issue was modified by someone else. Please reload and try again.',
					{
						action: {
							label: $t('issueDetail.reload') || 'Reload',
							onClick: () => issueId && loadIssueData(issueId)
						}
					}
				);
			} else {
				toast.error($t('issueDetail.statusUpdateError') || 'Failed to update status');
			}
		} finally {
			statusUpdating = false;
		}
	}

	async function handleAddComment(text: string) {
		if (!issue || !text.trim()) return;

		commentSubmitting = true;
		hasConflict = false;

		try {
			const updatedIssue = await invoke<IssueDetail>(CMD_ADD_ISSUE_COMMENT, {
				issueId: issue.id,
				comment: text,
				version: issue.version
			});

			// Update local issue state with the response (includes new version)
			issue = updatedIssue;

			// Reload history to show the new comment
			await reloadHistory();

			// Clear comment text only on success
			commentText = '';

			toast.success($t('issueDetail.commentAdded') || 'Comment added successfully');

			// Notify parent to refresh if needed
			if (onIssueUpdated) {
				onIssueUpdated();
			}
		} catch (e) {
			console.error('Failed to add comment:', e);

			// DON'T clear commentText on error - preserve user's text
			if (isVersionConflict(e)) {
				hasConflict = true;
				toast.error(
					$t('issueDetail.versionConflict') ||
						'This issue was modified by someone else. Please reload and try again.',
					{
						action: {
							label: $t('issueDetail.reload') || 'Reload',
							onClick: () => handleReloadWithConfirmation()
						}
					}
				);
			} else {
				toast.error($t('issueDetail.commentError') || 'Failed to add comment');
			}
		} finally {
			commentSubmitting = false;
		}
	}

	function handleReload() {
		handleReloadWithConfirmation();
	}

	function handleReloadWithConfirmation() {
		if (!issueId) return;

		// If user has unsaved comment, confirm before reloading
		if (commentText && commentText.trim()) {
			const confirmed = confirm(
				$t('issueDetail.unsavedComment') || 'You have an unsaved comment. Reload anyway?'
			);
			if (!confirmed) return;
		}

		commentText = '';
		loadIssueData(issueId);
	}

	// Filter history to only show entries with comments
	// Note: Comments added via PATCH may have entry_type "change" but still contain comment text
	let comments = $derived(history.filter((h) => h.comment && !h.is_deleted));

	// ============================================================================
	// Description Editing Functions
	// ============================================================================

	async function checkForDraft(id: number) {
		try {
			const draft = await invoke<string | null>(CMD_GET_LOCAL_DRAFT, {
				relatedId: `issue_${id}`,
				draftType: 'description'
			});
			if (draft) {
				hasDraft = true;
				descriptionDraft = draft;
			}
		} catch (e) {
			console.warn('Failed to check for draft:', e);
		}
	}

	function startEditingDescription() {
		if (!issue) return;
		isEditingDescription = true;
		// Use existing draft if available, otherwise use current description
		if (!hasDraft) {
			descriptionDraft = issue.description || '';
		}
	}

	async function cancelEditingDescription() {
		isEditingDescription = false;
		// Clear the debounce timeout
		if (draftSaveTimeout) {
			clearTimeout(draftSaveTimeout);
			draftSaveTimeout = null;
		}
	}

	async function discardDraft() {
		if (!issueId) return;
		try {
			await invoke(CMD_DELETE_LOCAL_DRAFT, {
				relatedId: `issue_${issueId}`,
				draftType: 'description'
			});
			hasDraft = false;
			descriptionDraft = issue?.description || '';
			isEditingDescription = false;
		} catch (e) {
			console.error('Failed to discard draft:', e);
		}
	}

	function handleDescriptionInput(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		descriptionDraft = target.value;
		debounceSaveDraft();
	}

	function debounceSaveDraft() {
		if (draftSaveTimeout) {
			clearTimeout(draftSaveTimeout);
		}
		draftSaveTimeout = setTimeout(() => saveDraft(), DRAFT_DEBOUNCE_MS);
	}

	async function saveDraft() {
		if (!issueId) return;
		try {
			await invoke(CMD_SAVE_LOCAL_DRAFT, {
				relatedId: `issue_${issueId}`,
				draftType: 'description',
				content: descriptionDraft
			});
			hasDraft = true;
			console.log('Draft saved');
		} catch (e) {
			console.error('Failed to save draft:', e);
		}
	}

	async function commitDescription() {
		if (!issue) return;

		// First, save the current draft immediately
		if (draftSaveTimeout) {
			clearTimeout(draftSaveTimeout);
			draftSaveTimeout = null;
		}
		await saveDraft();

		descriptionSaving = true;
		hasConflict = false;

		try {
			const updatedIssue = await invoke<IssueDetail>(CMD_COMMIT_ISSUE_DESCRIPTION, {
				issueId: issue.id,
				version: issue.version
			});

			issue = updatedIssue;
			isEditingDescription = false;
			hasDraft = false;
			descriptionDraft = '';

			toast.success($t('issueDetail.descriptionUpdated') || 'Description updated successfully');

			if (onIssueUpdated) {
				onIssueUpdated();
			}
		} catch (e) {
			console.error('Failed to commit description:', e);

			if (isVersionConflict(e)) {
				hasConflict = true;
				toast.error(
					$t('issueDetail.versionConflict') ||
						'This issue was modified by someone else. Please reload and try again.',
					{
						action: {
							label: $t('issueDetail.reload') || 'Reload',
							onClick: () => handleReloadWithConfirmation()
						}
					}
				);
			} else {
				toast.error($t('issueDetail.descriptionUpdateError') || 'Failed to update description');
			}
		} finally {
			descriptionSaving = false;
		}
	}
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
				<div class="flex items-center justify-between pr-8">
					<div class="flex items-center gap-2">
						<span class="text-muted-foreground font-mono">#{issue.ref_number}</span>
						<Badge variant={issue.is_closed ? 'secondary' : 'default'} class="text-xs">
							{issue.is_closed ? '✓ Closed' : '● Open'}
						</Badge>
					</div>
					<Button
						variant="ghost"
						size="icon"
						onclick={handleReload}
						disabled={loading || statusUpdating}
						title={$t('issueDetail.reload') || 'Reload'}
					>
						<RefreshCw class="h-4 w-4" />
					</Button>
				</div>
				<Sheet.Title class="mt-2 text-xl leading-tight font-semibold">
					{issue.subject}
				</Sheet.Title>
			</Sheet.Header>

			<!-- Conflict Warning -->
			{#if hasConflict}
				<div
					class="bg-destructive/10 text-destructive flex items-center justify-between gap-2 px-6 py-3 text-sm"
				>
					<span
						>{$t('issueDetail.versionConflict') || 'This issue was modified by someone else.'}</span
					>
					<Button variant="outline" size="sm" onclick={handleReload}>
						<RefreshCw class="mr-2 h-3 w-3" />
						{$t('issueDetail.reload') || 'Reload'}
					</Button>
				</div>
			{/if}

			<!-- Scrollable content area -->
			<div class="flex-1 overflow-y-auto">
				<div class="space-y-6 p-6">
					<!-- Metadata Section -->
					<IssueMetadata {issue} {statuses} {statusUpdating} onStatusChange={handleStatusChange} />

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
						<div class="mb-2 flex items-center justify-between">
							<h3 class="text-sm font-medium">{$t('issueDetail.description') || 'Description'}</h3>
							{#if !isEditingDescription}
								<div class="flex items-center gap-2">
									{#if hasDraft}
										<Badge variant="secondary" class="text-xs">
											{$t('issueDetail.draftSaved') || 'Draft saved'}
										</Badge>
									{/if}
									<Button
										variant="ghost"
										size="sm"
										class="h-7 gap-1 text-xs"
										onclick={startEditingDescription}
										disabled={loading || statusUpdating}
									>
										<Edit3 class="h-3 w-3" />
										{$t('issueDetail.editDescription') || 'Edit'}
									</Button>
								</div>
							{/if}
						</div>

						{#if isEditingDescription}
							<!-- Edit Mode -->
							<div class="space-y-3">
								<textarea
									value={descriptionDraft}
									oninput={handleDescriptionInput}
									disabled={descriptionSaving}
									class="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[150px] w-full resize-y rounded-lg border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
									rows={6}
									placeholder={$t('issueDetail.noDescription') || 'No description provided'}
								></textarea>
								<div class="flex items-center justify-between">
									<div class="flex gap-2">
										{#if hasDraft}
											<Button
												variant="ghost"
												size="sm"
												onclick={discardDraft}
												disabled={descriptionSaving}
												class="text-destructive hover:text-destructive"
											>
												{$t('issueDetail.discardDraft') || 'Discard draft'}
											</Button>
										{/if}
									</div>
									<div class="flex gap-2">
										<Button
											variant="outline"
											size="sm"
											onclick={cancelEditingDescription}
											disabled={descriptionSaving}
										>
											<X class="mr-1 h-3 w-3" />
											{$t('issueDetail.cancelEdit') || 'Cancel'}
										</Button>
										<Button size="sm" onclick={commitDescription} disabled={descriptionSaving}>
											{#if descriptionSaving}
												<Loader2 class="mr-1 h-3 w-3 animate-spin" />
												{$t('issueDetail.savingDescription') || 'Saving...'}
											{:else}
												<Save class="mr-1 h-3 w-3" />
												{$t('issueDetail.saveDescription') || 'Save'}
											{/if}
										</Button>
									</div>
								</div>
							</div>
						{:else if issue.description_html}
							<!-- View Mode: HTML -->
							<div class="prose prose-sm dark:prose-invert bg-muted/30 max-w-none rounded-lg p-4">
								{@html issue.description_html}
							</div>
						{:else if issue.description}
							<!-- View Mode: Plain text -->
							<div class="bg-muted/30 rounded-lg p-4 text-sm whitespace-pre-wrap">
								{issue.description}
							</div>
						{:else}
							<!-- No description -->
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
						<CommentList
							{comments}
							bind:commentText
							submitting={commentSubmitting}
							onSubmit={handleAddComment}
						/>
					</div>
				</div>
			</div>
		{/if}
	</Sheet.Content>
</Sheet.Root>
