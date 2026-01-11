<script lang="ts">
	import type {
		IssueDetail,
		IssueStatus,
		Member,
		Priority,
		Severity,
		IssueType,
		TagColor,
		Tag,
		HistoryEntry,
		Attachment
	} from '$lib/types';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as Select from '$lib/components/ui/select';
	import { User, Calendar, Clock, AlertTriangle, Loader2 } from '@lucide/svelte';
	import { t } from 'svelte-i18n';
	import PrioritySelector from './PrioritySelector.svelte';
	import SeveritySelector from './SeveritySelector.svelte';
	import TypeSelector from './TypeSelector.svelte';
	import LabelManager from './LabelManager.svelte';
	import AttachmentManager from './AttachmentManager.svelte';
	import ActivityLog from './ActivityLog.svelte';

	let {
		issue,
		statuses = [],
		members = [],
		priorities = [],
		severities = [],
		issueTypes = [],
		tagsColors = [],
		history = [],
		attachments = [],
		attachmentsError = null,
		statusUpdating = false,
		assigneeUpdating = false,
		priorityUpdating = false,
		severityUpdating = false,
		typeUpdating = false,
		tagsUpdating = false,
		attachmentUploading = false,
		disabled = false,
		onStatusChange,
		onAssigneeChange,
		onPriorityChange,
		onSeverityChange,
		onTypeChange,
		onTagsChange,
		onAttachmentUpload,
		onAttachmentDelete,
		onRetryLoadAttachments
	}: {
		issue: IssueDetail;
		statuses?: IssueStatus[];
		members?: Member[];
		priorities?: Priority[];
		severities?: Severity[];
		issueTypes?: IssueType[];
		tagsColors?: TagColor[];
		history?: HistoryEntry[];
		attachments?: Attachment[];
		attachmentsError?: string | null;
		statusUpdating?: boolean;
		assigneeUpdating?: boolean;
		priorityUpdating?: boolean;
		severityUpdating?: boolean;
		typeUpdating?: boolean;
		tagsUpdating?: boolean;
		attachmentUploading?: boolean;
		disabled?: boolean;
		onStatusChange?: (statusId: number) => void;
		onAssigneeChange?: (assigneeId: number | null) => void;
		onPriorityChange?: (priorityId: number) => void;
		onSeverityChange?: (severityId: number) => void;
		onTypeChange?: (typeId: number) => void;
		onTagsChange?: (tags: Tag[]) => void;
		onAttachmentUpload?: (fileName: string, fileData: Uint8Array) => void;
		onAttachmentDelete?: (attachmentId: number) => void;
		onRetryLoadAttachments?: () => void;
	} = $props();

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		if (isNaN(date.getTime())) {
			return 'Invalid date';
		}
		return date.toLocaleDateString(undefined, {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getInitials(name: string): string {
		return name
			.split(' ')
			.filter((n) => n.length > 0)
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}

	function handleStatusChange(value: string | undefined) {
		if (value && onStatusChange) {
			onStatusChange(parseInt(value, 10));
		}
	}

	function handleAssigneeChange(value: string | undefined) {
		if (onAssigneeChange) {
			if (value === 'unassigned') {
				onAssigneeChange(null);
			} else if (value) {
				onAssigneeChange(parseInt(value, 10));
			}
		}
	}

	let canEditStatus = $derived(statuses.length > 0 && onStatusChange !== undefined);
	let canEditAssignee = $derived(members.length > 0 && onAssigneeChange !== undefined);
</script>

<div class="flex h-full flex-col overflow-y-auto">
	<div class="space-y-5 p-4">
		<div>
			<span class="text-muted-foreground mb-1.5 block text-xs font-medium tracking-wide uppercase">
				{$t('issueDetail.status') || 'Status'}
			</span>
			{#if canEditStatus}
				{#key issue.status_id}
					<Select.Root
						type="single"
						value={issue.status_id.toString()}
						onValueChange={handleStatusChange}
						disabled={statusUpdating || disabled}
					>
						<Select.Trigger class="h-9 w-full" style="border-color: {issue.status_color}">
							{#if statusUpdating}
								<Loader2 class="mr-2 h-4 w-4 animate-spin" />
							{/if}
							<span style="color: {issue.status_color}">{issue.status_name}</span>
						</Select.Trigger>
						<Select.Content>
							{#each statuses as status (status.id)}
								<Select.Item value={status.id.toString()}>
									<div class="flex items-center gap-2">
										<div
											class="h-3 w-3 rounded-full"
											style="background-color: {status.color}"
										></div>
										<span>{status.name}</span>
										{#if status.is_closed}
											<span class="text-muted-foreground text-xs">(Closed)</span>
										{/if}
									</div>
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				{/key}
			{:else}
				<div
					class="inline-flex items-center rounded-md border px-2.5 py-1 text-sm font-medium"
					style="border-color: {issue.status_color}; color: {issue.status_color}"
				>
					{issue.status_name}
				</div>
			{/if}
		</div>

		<div>
			<span class="text-muted-foreground mb-1.5 block text-xs font-medium tracking-wide uppercase">
				{$t('issueDetail.assignee') || 'Assignee'}
			</span>
			{#if canEditAssignee}
				{#key issue.assigned_to_id}
					<Select.Root
						type="single"
						value={issue.assigned_to_id ? issue.assigned_to_id.toString() : 'unassigned'}
						onValueChange={handleAssigneeChange}
						disabled={assigneeUpdating || disabled}
					>
						<Select.Trigger class="h-9 w-full">
							{#if assigneeUpdating}
								<Loader2 class="mr-2 h-4 w-4 animate-spin" />
							{/if}
							{#if issue.assigned_to_name}
								<div class="flex items-center gap-2">
									<Avatar.Root class="h-5 w-5">
										{#if issue.assigned_to_photo}
											<Avatar.Image src={issue.assigned_to_photo} alt={issue.assigned_to_name} />
										{/if}
										<Avatar.Fallback class="text-xs">
											{getInitials(issue.assigned_to_name)}
										</Avatar.Fallback>
									</Avatar.Root>
									<span class="truncate">{issue.assigned_to_name}</span>
								</div>
							{:else}
								<span class="text-muted-foreground italic">Unassigned</span>
							{/if}
						</Select.Trigger>
						<Select.Content>
							<Select.Item value="unassigned">
								<span class="text-muted-foreground italic">Unassigned</span>
							</Select.Item>
							{#each members as member (member.id)}
								{#if member.user_id}
									<Select.Item value={member.user_id.toString()}>
										<div class="flex items-center gap-2">
											<Avatar.Root class="h-5 w-5">
												{#if member.photo}
													<Avatar.Image src={member.photo} alt={member.full_name} />
												{/if}
												<Avatar.Fallback class="text-xs">
													{getInitials(member.full_name)}
												</Avatar.Fallback>
											</Avatar.Root>
											<span>{member.full_name}</span>
											<span class="text-muted-foreground text-xs">({member.role_name})</span>
										</div>
									</Select.Item>
								{/if}
							{/each}
						</Select.Content>
					</Select.Root>
				{/key}
			{:else if issue.assigned_to_name}
				<div class="flex items-center gap-2">
					<Avatar.Root class="h-6 w-6">
						{#if issue.assigned_to_photo}
							<Avatar.Image src={issue.assigned_to_photo} alt={issue.assigned_to_name} />
						{/if}
						<Avatar.Fallback class="text-xs">
							{getInitials(issue.assigned_to_name)}
						</Avatar.Fallback>
					</Avatar.Root>
					<span class="text-sm font-medium">{issue.assigned_to_name}</span>
				</div>
			{:else}
				<span class="text-muted-foreground text-sm italic">Unassigned</span>
			{/if}
		</div>

		{#if priorities.length > 0 || issue.priority_id}
			<PrioritySelector
				currentPriorityId={issue.priority_id}
				{priorities}
				updating={priorityUpdating}
				{disabled}
				{onPriorityChange}
			/>
		{/if}

		{#if severities.length > 0 || issue.severity_id}
			<SeveritySelector
				currentSeverityId={issue.severity_id}
				{severities}
				updating={severityUpdating}
				{disabled}
				{onSeverityChange}
			/>
		{/if}

		{#if issueTypes.length > 0 || issue.type_id}
			<TypeSelector
				currentTypeId={issue.type_id}
				{issueTypes}
				updating={typeUpdating}
				{disabled}
				{onTypeChange}
			/>
		{/if}

		<div>
			<span class="text-muted-foreground mb-1.5 block text-xs font-medium tracking-wide uppercase">
				{$t('issueDetail.labels') || 'Labels'}
			</span>
			<LabelManager
				tags={issue.tags}
				availableColors={tagsColors}
				updating={tagsUpdating}
				{disabled}
				{onTagsChange}
			/>
		</div>

		<AttachmentManager
			{attachments}
			{attachmentsError}
			uploading={attachmentUploading}
			{disabled}
			onUpload={onAttachmentUpload}
			onDelete={onAttachmentDelete}
			onRetry={onRetryLoadAttachments}
		/>

		{#if issue.owner_name}
			<div>
				<span
					class="text-muted-foreground mb-1.5 block text-xs font-medium tracking-wide uppercase"
				>
					{$t('issueDetail.reporter') || 'Reporter'}
				</span>
				<div class="flex items-center gap-2">
					<Avatar.Root class="h-5 w-5">
						{#if issue.owner_photo}
							<Avatar.Image src={issue.owner_photo} alt={issue.owner_name} />
						{/if}
						<Avatar.Fallback class="text-xs">
							{getInitials(issue.owner_name)}
						</Avatar.Fallback>
					</Avatar.Root>
					<span class="text-sm">{issue.owner_name}</span>
				</div>
			</div>
		{/if}

		<div class="space-y-2">
			<div class="text-muted-foreground flex items-center gap-2 text-xs">
				<Calendar class="h-3.5 w-3.5" />
				<span>{$t('issueDetail.created') || 'Created'}: {formatDate(issue.created_date)}</span>
			</div>
			<div class="text-muted-foreground flex items-center gap-2 text-xs">
				<Clock class="h-3.5 w-3.5" />
				<span>{$t('issueDetail.updated') || 'Updated'}: {formatDate(issue.modified_date)}</span>
			</div>
		</div>

		{#if issue.due_date}
			<div
				class="flex items-center gap-2 rounded-lg p-2 text-sm {issue.due_date_status === 'past_due'
					? 'bg-destructive/10 text-destructive'
					: 'bg-card/30'}"
			>
				{#if issue.due_date_status === 'past_due'}
					<AlertTriangle class="h-4 w-4" />
				{/if}
				<span>Due: {formatDate(issue.due_date)}</span>
				{#if issue.due_date_status === 'past_due'}
					<span class="font-medium">(Overdue)</span>
				{/if}
			</div>
		{/if}

		{#if issue.is_blocked}
			<div
				class="bg-destructive/10 text-destructive flex items-center gap-2 rounded-lg p-2 text-sm"
			>
				<AlertTriangle class="h-4 w-4" />
				<span class="font-medium">Blocked</span>
				{#if issue.blocked_note}
					<span>- {issue.blocked_note}</span>
				{/if}
			</div>
		{/if}

		{#if issue.total_watchers > 0}
			<div class="text-muted-foreground flex items-center gap-2 text-sm">
				<User class="h-4 w-4" />
				<span>{issue.total_watchers} watcher{issue.total_watchers > 1 ? 's' : ''}</span>
			</div>
		{/if}

		<div>
			<span class="text-muted-foreground mb-2 block text-xs font-medium tracking-wide uppercase">
				{$t('issueDetail.activity') || 'Activity'}
			</span>
			<ActivityLog entries={history} initialLimit={4} />
		</div>
	</div>
</div>
