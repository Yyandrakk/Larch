<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import {
		CMD_GET_ISSUE_DETAIL,
		CMD_GET_ISSUE_HISTORY,
		CMD_CHANGE_ISSUE_STATUS,
		CMD_CHANGE_ISSUE_ASSIGNEE,
		CMD_GET_PROJECT_METADATA,
		CMD_ADD_ISSUE_COMMENT,
		CMD_SAVE_LOCAL_DRAFT,
		CMD_GET_LOCAL_DRAFT,
		CMD_DELETE_LOCAL_DRAFT,
		CMD_COMMIT_ISSUE_DESCRIPTION,
		CMD_CHANGE_ISSUE_PRIORITY,
		CMD_CHANGE_ISSUE_SEVERITY,
		CMD_CHANGE_ISSUE_TYPE,
		CMD_UPDATE_ISSUE_TAGS,
		CMD_UPLOAD_ISSUE_ATTACHMENT,
		CMD_DELETE_ISSUE_ATTACHMENT,
		CMD_GET_ISSUE_ATTACHMENTS
	} from '$lib/commands.svelte';
	import type {
		IssueDetail,
		HistoryEntry,
		IssueStatus,
		ProjectMetadata,
		Member,
		Priority,
		Severity,
		IssueType,
		TagColor,
		Tag,
		Attachment
	} from '$lib/types';
	import * as Sheet from '$lib/components/ui/sheet';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { t } from 'svelte-i18n';
	import { toast } from 'svelte-sonner';
	import { MessageSquare, Edit3, X, Save, Loader2, ChevronRight } from '@lucide/svelte';
	import CommentList from './CommentList.svelte';
	import StatusChip from './StatusChip.svelte';
	import IssueMetadataSidebar from './IssueMetadataSidebar.svelte';
	import { setPendingCommit } from '$lib/stores/pendingClose';

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
	let members = $state<Member[]>([]);
	let priorities = $state<Priority[]>([]);
	let severities = $state<Severity[]>([]);
	let issueTypes = $state<IssueType[]>([]);
	let tagsColors = $state<TagColor[]>([]);
	let attachments = $state<Attachment[]>([]);
	let attachmentsError = $state<string | null>(null);
	let loading = $state(false);
	let statusUpdating = $state(false);
	let assigneeUpdating = $state(false);
	let priorityUpdating = $state(false);
	let severityUpdating = $state(false);
	let typeUpdating = $state(false);
	let tagsUpdating = $state(false);
	let attachmentUploading = $state(false);
	let commentSubmitting = $state(false);
	let commentText = $state('');
	let error = $state<string | null>(null);
	let hasConflict = $state(false);

	let isEditingDescription = $state(false);
	let descriptionDraft = $state('');
	let descriptionSaving = $state(false);
	let hasDraft = $state(false);
	let draftSaveTimeout: ReturnType<typeof setTimeout> | null = null;
	const DRAFT_DEBOUNCE_MS = 2000;

	type SaveStatus = 'saved' | 'saving' | 'collision';
	let saveStatus = $derived<SaveStatus>(
		hasConflict
			? 'collision'
			: statusUpdating ||
				  assigneeUpdating ||
				  priorityUpdating ||
				  severityUpdating ||
				  typeUpdating ||
				  tagsUpdating ||
				  attachmentUploading ||
				  descriptionSaving ||
				  commentSubmitting
				? 'saving'
				: 'saved'
	);

	$effect(() => {
		if (issueId && open) {
			loadIssueData(issueId);
		}
	});

	$effect(() => {
		if (issueId) {
			commentText = '';
			isEditingDescription = false;
			descriptionDraft = '';
			hasDraft = false;
			checkForDraft(issueId);
		}
	});

	$effect(() => {
		if (hasDraft && issueId) {
			setPendingCommit(async () => {
				return await tryCommitDescriptionForClose();
			});
		} else {
			setPendingCommit(null);
		}

		return () => {
			setPendingCommit(null);
		};
	});

	async function loadIssueData(id: number) {
		loading = true;
		error = null;
		issue = null;
		history = [];
		statuses = [];
		members = [];
		priorities = [];
		severities = [];
		issueTypes = [];
		tagsColors = [];
		attachments = [];
		attachmentsError = null;
		hasConflict = false;

		try {
			const [issueResult, historyResult] = await Promise.all([
				invoke<IssueDetail>(CMD_GET_ISSUE_DETAIL, { issueId: id }),
				invoke<HistoryEntry[]>(CMD_GET_ISSUE_HISTORY, { issueId: id })
			]);

			issue = issueResult;
			history = historyResult;

			if (issue.project_id) {
				try {
					const metadata = await invoke<Record<number, ProjectMetadata>>(CMD_GET_PROJECT_METADATA, {
						projectIds: [issue.project_id]
					});
					if (metadata[issue.project_id]) {
						statuses = metadata[issue.project_id].statuses;
						members = metadata[issue.project_id].members;
						priorities = metadata[issue.project_id].priorities || [];
						severities = metadata[issue.project_id].severities || [];
						issueTypes = metadata[issue.project_id].issue_types || [];
						tagsColors = metadata[issue.project_id].tags_colors || [];
					}
				} catch (metaErr) {
					console.warn('Failed to load project metadata:', metaErr);
				}

				await loadAttachments(issue.project_id, id);
			}
		} catch (e) {
			console.error('Failed to load issue:', e);
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

	async function loadAttachments(projectId: number, issueIdParam: number) {
		attachmentsError = null;
		try {
			attachments = await invoke<Attachment[]>(CMD_GET_ISSUE_ATTACHMENTS, {
				projectId,
				issueId: issueIdParam
			});
		} catch (e) {
			console.error('Failed to load attachments:', e);
			attachmentsError = $t('issueDetail.attachmentsError') || 'Failed to load attachments';
		}
	}

	async function reloadHistory() {
		if (!issueId) {
			return;
		}
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
		if (!issue || newStatusId === issue.status_id) {
			return;
		}

		statusUpdating = true;
		hasConflict = false;

		try {
			const updatedIssue = await invoke<IssueDetail>(CMD_CHANGE_ISSUE_STATUS, {
				issueId: issue.id,
				statusId: newStatusId,
				version: issue.version
			});

			issue = updatedIssue;
			const newStatus = statuses.find((s) => s.id === newStatusId);
			toast.success(
				$t('issueDetail.statusUpdated') || `Status updated to ${newStatus?.name || 'new status'}`
			);
			await reloadHistory();
			onIssueUpdated?.();
		} catch (e) {
			console.error('Failed to update status:', e);
			if (isVersionConflict(e)) {
				handleVersionConflict();
			} else {
				toast.error($t('issueDetail.statusUpdateError') || 'Failed to update status');
			}
		} finally {
			statusUpdating = false;
		}
	}

	async function handleAssigneeChange(newAssigneeId: number | null) {
		if (!issue || newAssigneeId === issue.assigned_to_id) {
			return;
		}

		assigneeUpdating = true;
		hasConflict = false;

		try {
			const updatedIssue = await invoke<IssueDetail>(CMD_CHANGE_ISSUE_ASSIGNEE, {
				issueId: issue.id,
				assigneeId: newAssigneeId,
				version: issue.version
			});

			issue = updatedIssue;
			const newAssignee = members.find((m) => m.user_id === newAssigneeId);
			const assigneeName = newAssignee?.full_name || 'Unassigned';
			toast.success($t('issueDetail.assigneeUpdated') || `Assigned to ${assigneeName}`);
			await reloadHistory();
			onIssueUpdated?.();
		} catch (e) {
			console.error('Failed to update assignee:', e);
			if (isVersionConflict(e)) {
				handleVersionConflict();
			} else {
				toast.error($t('issueDetail.assigneeUpdateError') || 'Failed to update assignee');
			}
		} finally {
			assigneeUpdating = false;
		}
	}

	async function handlePriorityChange(newPriorityId: number) {
		if (!issue || newPriorityId === issue.priority_id) {
			return;
		}

		priorityUpdating = true;
		hasConflict = false;

		try {
			const updatedIssue = await invoke<IssueDetail>(CMD_CHANGE_ISSUE_PRIORITY, {
				issueId: issue.id,
				priorityId: newPriorityId,
				version: issue.version
			});

			issue = updatedIssue;
			toast.success($t('issueDetail.priorityUpdated') || 'Priority updated');
			await reloadHistory();
			onIssueUpdated?.();
		} catch (e) {
			console.error('Failed to update priority:', e);
			if (isVersionConflict(e)) {
				handleVersionConflict();
			} else {
				toast.error($t('issueDetail.priorityUpdateError') || 'Failed to update priority');
			}
		} finally {
			priorityUpdating = false;
		}
	}

	async function handleSeverityChange(newSeverityId: number) {
		if (!issue || newSeverityId === issue.severity_id) {
			return;
		}

		severityUpdating = true;
		hasConflict = false;

		try {
			const updatedIssue = await invoke<IssueDetail>(CMD_CHANGE_ISSUE_SEVERITY, {
				issueId: issue.id,
				severityId: newSeverityId,
				version: issue.version
			});

			issue = updatedIssue;
			toast.success($t('issueDetail.severityUpdated') || 'Severity updated');
			await reloadHistory();
			onIssueUpdated?.();
		} catch (e) {
			console.error('Failed to update severity:', e);
			if (isVersionConflict(e)) {
				handleVersionConflict();
			} else {
				toast.error($t('issueDetail.severityUpdateError') || 'Failed to update severity');
			}
		} finally {
			severityUpdating = false;
		}
	}

	async function handleTypeChange(newTypeId: number) {
		if (!issue || newTypeId === issue.type_id) {
			return;
		}

		typeUpdating = true;
		hasConflict = false;

		try {
			const updatedIssue = await invoke<IssueDetail>(CMD_CHANGE_ISSUE_TYPE, {
				issueId: issue.id,
				typeId: newTypeId,
				version: issue.version
			});

			issue = updatedIssue;
			toast.success($t('issueDetail.typeUpdated') || 'Type updated');
			await reloadHistory();
			onIssueUpdated?.();
		} catch (e) {
			console.error('Failed to update type:', e);
			if (isVersionConflict(e)) {
				handleVersionConflict();
			} else {
				toast.error($t('issueDetail.typeUpdateError') || 'Failed to update type');
			}
		} finally {
			typeUpdating = false;
		}
	}

	async function handleTagsChange(newTags: Tag[]) {
		if (!issue) {
			return;
		}

		tagsUpdating = true;
		hasConflict = false;

		try {
			const tagsPayload = newTags.map((t) => [t.name, t.color ?? null] as [string, string | null]);
			const updatedIssue = await invoke<IssueDetail>(CMD_UPDATE_ISSUE_TAGS, {
				issueId: issue.id,
				tags: tagsPayload,
				version: issue.version
			});

			issue = updatedIssue;
			toast.success($t('issueDetail.tagsUpdated') || 'Labels updated');
			await reloadHistory();
			onIssueUpdated?.();
		} catch (e) {
			console.error('Failed to update tags:', e);
			if (isVersionConflict(e)) {
				handleVersionConflict();
			} else {
				toast.error($t('issueDetail.tagsUpdateError') || 'Failed to update labels');
			}
		} finally {
			tagsUpdating = false;
		}
	}

	async function handleAttachmentUpload(fileName: string, fileData: Uint8Array) {
		if (!issue) {
			return;
		}

		attachmentUploading = true;
		hasConflict = false;

		try {
			const newAttachment = await invoke<Attachment>(CMD_UPLOAD_ISSUE_ATTACHMENT, {
				projectId: issue.project_id,
				issueId: issue.id,
				fileName,
				fileData: Array.from(fileData)
			});

			attachments = [...attachments, newAttachment];
			attachmentsError = null;
			toast.success($t('issueDetail.attachmentUploaded') || 'Attachment uploaded');
			onIssueUpdated?.();
		} catch (e) {
			console.error('Failed to upload attachment:', e);
			toast.error($t('issueDetail.attachmentUploadError') || 'Failed to upload attachment');
		} finally {
			attachmentUploading = false;
		}
	}

	async function handleAttachmentDelete(attachmentId: number) {
		if (!issue) {
			return;
		}

		try {
			await invoke(CMD_DELETE_ISSUE_ATTACHMENT, { attachmentId });
			attachments = attachments.filter((a) => a.id !== attachmentId);
			toast.success($t('issueDetail.attachmentDeleted') || 'Attachment deleted');
			onIssueUpdated?.();
		} catch (e) {
			console.error('Failed to delete attachment:', e);
			toast.error($t('issueDetail.attachmentDeleteError') || 'Failed to delete attachment');
		}
	}

	async function handleRetryLoadAttachments() {
		if (!issue) {
			return;
		}
		await loadAttachments(issue.project_id, issue.id);
	}

	async function handleAddComment(text: string) {
		if (!issue || !text.trim()) {
			return;
		}

		commentSubmitting = true;
		hasConflict = false;

		try {
			const updatedIssue = await invoke<IssueDetail>(CMD_ADD_ISSUE_COMMENT, {
				issueId: issue.id,
				comment: text,
				version: issue.version
			});

			issue = updatedIssue;
			await reloadHistory();
			commentText = '';
			toast.success($t('issueDetail.commentAdded') || 'Comment added successfully');
			onIssueUpdated?.();
		} catch (e) {
			console.error('Failed to add comment:', e);
			if (isVersionConflict(e)) {
				handleVersionConflict();
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
		if (!issueId) {
			return;
		}

		if (commentText && commentText.trim()) {
			const confirmed = confirm(
				$t('issueDetail.unsavedComment') || 'You have an unsaved comment. Reload anyway?'
			);
			if (!confirmed) {
				return;
			}
		}

		commentText = '';
		hasConflict = false;
		loadIssueData(issueId);
	}

	let comments = $derived(history.filter((h) => h.comment && !h.is_deleted));

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
		if (!issue) {
			return;
		}
		isEditingDescription = true;
		if (!hasDraft) {
			descriptionDraft = issue.description || '';
		}
	}

	async function cancelEditingDescription() {
		isEditingDescription = false;
		if (draftSaveTimeout) {
			clearTimeout(draftSaveTimeout);
			draftSaveTimeout = null;
		}
	}

	async function discardDraft() {
		if (!issueId) {
			return;
		}
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
		if (!issueId) {
			return;
		}
		try {
			await invoke(CMD_SAVE_LOCAL_DRAFT, {
				relatedId: `issue_${issueId}`,
				draftType: 'description',
				content: descriptionDraft
			});
			hasDraft = true;
		} catch (e) {
			console.error('Failed to save draft:', e);
		}
	}

	async function commitDescription() {
		await executeCommitDescription();
	}

	async function tryCommitDescriptionForClose(): Promise<boolean> {
		if (!issue || !hasDraft) {
			return true;
		}
		return await executeCommitDescription();
	}

	async function executeCommitDescription(): Promise<boolean> {
		if (!issue) {
			return false;
		}

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
			onIssueUpdated?.();

			return true;
		} catch (e) {
			console.error('Failed to commit description:', e);
			if (isVersionConflict(e)) {
				handleVersionConflict();
			} else {
				toast.error($t('issueDetail.descriptionUpdateError') || 'Failed to update description');
			}
			return false;
		} finally {
			descriptionSaving = false;
		}
	}

	function handleVersionConflict() {
		hasConflict = true;
		toast.error(
			$t('issueDetail.versionConflict') ||
				'This issue was modified by someone else. Please refresh and try again.',
			{
				action: {
					label: $t('issueDetail.reload') || 'Refresh',
					onClick: () => handleReloadWithConfirmation()
				}
			}
		);
	}
</script>

<Sheet.Root bind:open>
	<Sheet.Content
		class="flex h-full w-full flex-col overflow-hidden bg-[#111821] sm:max-w-2xl md:max-w-3xl lg:max-w-5xl"
		side="right"
	>
		{#if loading}
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
			<Sheet.Header class="flex-shrink-0 space-y-3 border-b pb-4">
				<div class="flex items-center justify-between pr-8">
					<div class="flex items-center gap-2 text-sm">
						<span class="text-muted-foreground">{issue.project_name}</span>
						<ChevronRight class="text-muted-foreground h-4 w-4" />
						<span class="font-mono font-medium">#{issue.ref_number}</span>
						<Badge variant={issue.is_closed ? 'secondary' : 'default'} class="ml-1 text-xs">
							{issue.is_closed ? '✓ Closed' : '● Open'}
						</Badge>
					</div>
					<StatusChip status={saveStatus} onRefresh={handleReload} />
				</div>
				<Sheet.Title class="text-xl leading-tight font-semibold">
					{issue.subject}
				</Sheet.Title>
			</Sheet.Header>

			<div class="flex min-h-0 flex-1">
				<div class="flex min-w-0 flex-[7] flex-col border-r">
					<div class="flex min-h-0 flex-1 flex-col overflow-y-auto p-6">
						<div class="mb-6">
							<div class="mb-2 flex items-center justify-between">
								<h3 class="text-sm font-medium">
									{$t('issueDetail.description') || 'Description'}
								</h3>
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
								<div class="prose prose-sm dark:prose-invert bg-card/50 max-w-none rounded-lg p-4">
									{@html issue.description_html}
								</div>
							{:else if issue.description}
								<div class="bg-card/50 rounded-lg p-4 text-sm whitespace-pre-wrap">
									{issue.description}
								</div>
							{:else}
								<p class="text-muted-foreground text-sm italic">
									{$t('issueDetail.noDescription') || 'No description provided'}
								</p>
							{/if}
						</div>

						<div class="flex-1">
							<h3 class="mb-2 flex items-center gap-2 text-sm font-medium">
								<MessageSquare class="h-4 w-4" />
								{$t('issueDetail.comments') || 'Comments'} ({comments.length})
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

				<div
					class="border-border dark:border-border flex w-80 flex-shrink-0 flex-col border-l dark:bg-[#111821]"
				>
					<IssueMetadataSidebar
						{issue}
						{statuses}
						{members}
						{priorities}
						{severities}
						{issueTypes}
						{tagsColors}
						{history}
						{attachments}
						{attachmentsError}
						{statusUpdating}
						{assigneeUpdating}
						{priorityUpdating}
						{severityUpdating}
						{typeUpdating}
						{tagsUpdating}
						{attachmentUploading}
						onStatusChange={handleStatusChange}
						onAssigneeChange={handleAssigneeChange}
						onPriorityChange={handlePriorityChange}
						onSeverityChange={handleSeverityChange}
						onTypeChange={handleTypeChange}
						onTagsChange={handleTagsChange}
						onAttachmentUpload={handleAttachmentUpload}
						onAttachmentDelete={handleAttachmentDelete}
						onRetryLoadAttachments={handleRetryLoadAttachments}
					/>
				</div>
			</div>
		{/if}
	</Sheet.Content>
</Sheet.Root>
