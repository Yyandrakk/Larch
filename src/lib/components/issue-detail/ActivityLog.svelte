<script lang="ts">
	import type { HistoryEntry } from '$lib/types';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import { ChevronDown, MessageSquare, GitCommit } from '@lucide/svelte';
	import { t } from 'svelte-i18n';

	let {
		entries,
		initialLimit = 4
	}: {
		entries: HistoryEntry[];
		initialLimit?: number;
	} = $props();

	let showAll = $state(false);

	let visibleEntries = $derived(showAll ? entries : entries.slice(0, initialLimit));
	let hasMore = $derived(entries.length > initialLimit);

	function getInitials(name: string): string {
		return name
			.split(' ')
			.filter((n) => n.length > 0)
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}

	function formatRelativeTime(dateStr: string): string {
		const date = new Date(dateStr);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffMins = Math.floor(diffMs / 60000);
		const diffHours = Math.floor(diffMs / 3600000);
		const diffDays = Math.floor(diffMs / 86400000);

		if (diffMins < 1) return $t('activity.justNow');
		if (diffMins < 60) return $t('activity.minutesAgo', { values: { count: diffMins } });
		if (diffHours < 24) return $t('activity.hoursAgo', { values: { count: diffHours } });
		if (diffDays < 7) return $t('activity.daysAgo', { values: { count: diffDays } });

		return date.toLocaleDateString(undefined, {
			month: 'short',
			day: 'numeric'
		});
	}

	function formatChangeDescription(entry: HistoryEntry): string {
		if (!entry.changes || entry.changes.length === 0) return '';

		const descriptions = entry.changes.map((change) => {
			const field = change.field.replace(/_/g, ' ');
			if (change.old_value && change.new_value) {
				return `${field}: ${change.old_value} → ${change.new_value}`;
			} else if (change.new_value) {
				return `${field}: set to ${change.new_value}`;
			} else if (change.old_value) {
				return `${field}: removed ${change.old_value}`;
			}
			return `${field} changed`;
		});

		return descriptions.join(', ');
	}
</script>

<div class="space-y-3">
	{#if visibleEntries.length === 0}
		<p class="text-muted-foreground text-sm italic">
			{$t('issueDetail.noActivity') || 'No activity yet'}
		</p>
	{:else}
		{#each visibleEntries as entry (entry.id)}
			<div class="flex gap-3">
				<Avatar.Root class="h-7 w-7 flex-shrink-0">
					{#if entry.user_photo}
						<Avatar.Image src={entry.user_photo} alt={entry.user_name} />
					{/if}
					<Avatar.Fallback class="text-xs">
						{getInitials(entry.user_name)}
					</Avatar.Fallback>
				</Avatar.Root>

				<div class="min-w-0 flex-1">
					<div class="flex items-center gap-2">
						<span class="truncate text-sm font-medium">{entry.user_name}</span>
						<span class="text-muted-foreground text-xs">
							{formatRelativeTime(entry.created_at)}
						</span>
					</div>

					{#if entry.entry_type === 'comment' && entry.comment}
						<div class="mt-1 flex items-start gap-1.5">
							<MessageSquare class="text-muted-foreground mt-0.5 h-3.5 w-3.5 flex-shrink-0" />
							<p class="text-muted-foreground line-clamp-2 text-sm">
								{entry.comment}
							</p>
						</div>
					{:else if entry.entry_type === 'change' && entry.changes && entry.changes.length > 0}
						<div class="mt-1 flex items-start gap-1.5">
							<GitCommit class="text-muted-foreground mt-0.5 h-3.5 w-3.5 flex-shrink-0" />
							<p class="text-muted-foreground line-clamp-2 text-sm">
								{formatChangeDescription(entry)}
							</p>
						</div>
					{/if}
				</div>
			</div>
		{/each}

		{#if hasMore && !showAll}
			<Button
				variant="ghost"
				size="sm"
				class="w-full gap-1 text-xs"
				onclick={() => (showAll = true)}
			>
				<ChevronDown class="h-3.5 w-3.5" />
				{$t('issueDetail.showMore') || 'Show more'} ({entries.length - initialLimit} more)
			</Button>
		{/if}
	{/if}
</div>
