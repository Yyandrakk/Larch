<script lang="ts">
	import type { IssueDetail } from '$lib/types';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import { User, Calendar, Clock, AlertTriangle } from '@lucide/svelte';

	let { issue }: { issue: IssueDetail } = $props();

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString(undefined, {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}
</script>

<div class="space-y-4">
	<!-- Primary Metadata Grid -->
	<div class="grid grid-cols-2 gap-4 text-sm">
		<!-- Status -->
		<div>
			<span class="text-muted-foreground mb-1 block">Status</span>
			<Badge
				variant="outline"
				style="border-color: {issue.status_color}; color: {issue.status_color}"
			>
				{issue.status_name}
			</Badge>
		</div>

		<!-- Project -->
		<div>
			<span class="text-muted-foreground mb-1 block">Project</span>
			<span class="font-medium">{issue.project_name}</span>
		</div>

		<!-- Type (if available) -->
		{#if issue.type_id}
			<div>
				<span class="text-muted-foreground mb-1 block">Type</span>
				{#if issue.type_name}
					<Badge
						variant="outline"
						style={issue.type_color
							? `border-color: ${issue.type_color}; color: ${issue.type_color}`
							: ''}
					>
						{issue.type_name}
					</Badge>
				{:else}
					<span class="text-muted-foreground">Type #{issue.type_id}</span>
				{/if}
			</div>
		{/if}

		<!-- Priority (if available) -->
		{#if issue.priority_id}
			<div>
				<span class="text-muted-foreground mb-1 block">Priority</span>
				{#if issue.priority_name}
					<Badge
						variant="outline"
						style={issue.priority_color
							? `border-color: ${issue.priority_color}; color: ${issue.priority_color}`
							: ''}
					>
						{issue.priority_name}
					</Badge>
				{:else}
					<span class="text-muted-foreground">Priority #{issue.priority_id}</span>
				{/if}
			</div>
		{/if}

		<!-- Severity (if available) -->
		{#if issue.severity_id}
			<div>
				<span class="text-muted-foreground mb-1 block">Severity</span>
				{#if issue.severity_name}
					<Badge
						variant="outline"
						style={issue.severity_color
							? `border-color: ${issue.severity_color}; color: ${issue.severity_color}`
							: ''}
					>
						{issue.severity_name}
					</Badge>
				{:else}
					<span class="text-muted-foreground">Severity #{issue.severity_id}</span>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Assignee -->
	<div class="bg-muted/30 flex items-center gap-3 rounded-lg p-3">
		<User class="text-muted-foreground h-4 w-4" />
		<span class="text-muted-foreground text-sm">Assigned to:</span>
		{#if issue.assigned_to_name}
			<div class="flex items-center gap-2">
				<Avatar.Root class="h-6 w-6">
					{#if issue.assigned_to_photo}
						<Avatar.Image src={issue.assigned_to_photo} alt={issue.assigned_to_name} />
					{/if}
					<Avatar.Fallback class="text-xs">
						{getInitials(issue.assigned_to_name)}
					</Avatar.Fallback>
				</Avatar.Root>
				<span class="font-medium">{issue.assigned_to_name}</span>
				{#if issue.assigned_to_username}
					<span class="text-muted-foreground">@{issue.assigned_to_username}</span>
				{/if}
			</div>
		{:else}
			<span class="text-muted-foreground italic">Unassigned</span>
		{/if}
	</div>

	<!-- Reporter -->
	{#if issue.owner_name}
		<div class="flex items-center gap-3 text-sm">
			<span class="text-muted-foreground">Created by:</span>
			<div class="flex items-center gap-2">
				<Avatar.Root class="h-5 w-5">
					{#if issue.owner_photo}
						<Avatar.Image src={issue.owner_photo} alt={issue.owner_name} />
					{/if}
					<Avatar.Fallback class="text-xs">
						{getInitials(issue.owner_name)}
					</Avatar.Fallback>
				</Avatar.Root>
				<span>{issue.owner_name}</span>
			</div>
		</div>
	{/if}

	<!-- Dates -->
	<div class="text-muted-foreground flex flex-wrap gap-4 text-sm">
		<div class="flex items-center gap-1">
			<Calendar class="h-4 w-4" />
			<span>Created: {formatDate(issue.created_date)}</span>
		</div>
		<div class="flex items-center gap-1">
			<Clock class="h-4 w-4" />
			<span>Updated: {formatDate(issue.modified_date)}</span>
		</div>
	</div>

	<!-- Due Date (if set) -->
	{#if issue.due_date}
		<div
			class="flex items-center gap-2 rounded-lg p-2 text-sm {issue.due_date_status === 'past_due'
				? 'bg-destructive/10 text-destructive'
				: 'bg-muted/30'}"
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

	<!-- Blocked indicator -->
	{#if issue.is_blocked}
		<div class="bg-destructive/10 text-destructive flex items-center gap-2 rounded-lg p-2 text-sm">
			<AlertTriangle class="h-4 w-4" />
			<span class="font-medium">Blocked</span>
			{#if issue.blocked_note}
				<span>- {issue.blocked_note}</span>
			{/if}
		</div>
	{/if}

	<!-- Watchers -->
	{#if issue.total_watchers > 0}
		<div class="text-muted-foreground flex items-center gap-2 text-sm">
			<span>👁 {issue.total_watchers} watcher{issue.total_watchers > 1 ? 's' : ''}</span>
		</div>
	{/if}
</div>
