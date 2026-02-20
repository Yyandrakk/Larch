<script lang="ts">
	import type { Issue, Project } from '$lib/types';
	import { t } from 'svelte-i18n';
	import { UserPlus, Clock } from '@lucide/svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';

	let {
		issues = [],
		projects = [],
		onIssueSelect
	}: {
		issues: Issue[];
		projects: Project[];
		onIssueSelect?: (issueId: number) => void;
	} = $props();

	function getProjectName(id: number): string {
		return projects.find((p: Project) => p.id === id)?.name || `Project ${id}`;
	}

	function handleRowClick(issueId: number) {
		if (onIssueSelect) {
			onIssueSelect(issueId);
		}
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

	function formatDate(dateStr: string | undefined): string {
		if (!dateStr) return '';
		try {
			const date = new Date(dateStr);
			if (isNaN(date.getTime())) return '';
			return date.toLocaleDateString(undefined, {
				year: 'numeric',
				month: 'short',
				day: 'numeric'
			});
		} catch {
			return '';
		}
	}
</script>

<table class="w-full border-collapse text-left">
	<thead class="sticky top-0 z-10 bg-[#111821]">
		<tr>
			<th
				class="w-24 border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('table.issue')}
			</th>
			<th
				class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('table.subject')}
			</th>
			<th
				class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('table.status')}
			</th>
			<th
				class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('table.project')}
			</th>
			<th
				class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('table.assignedTo')}
			</th>
			<th
				class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('dashboard.lastModified')}
			</th>
		</tr>
	</thead>
	<tbody class="text-sm">
		{#if issues.length === 0}
			<tr>
				<td colspan="6" class="h-24 text-center text-[#93a9c8]">
					{$t('dashboard.noIssues')}
				</td>
			</tr>
		{:else}
			{#each issues as issue (issue.id)}
				<tr
					class="table-row-hover group cursor-pointer border-b border-[#243347]/50 transition-colors"
					role="button"
					tabindex={0}
					onclick={() => handleRowClick(issue.id)}
					onkeydown={(e) => {
						if (e.key === 'Enter' || e.key === ' ') {
							e.preventDefault();
							handleRowClick(issue.id);
						}
					}}
				>
					<td class="px-2 py-2.5 font-mono text-xs text-[#93a9c8]">
						#{issue.id}
					</td>
					<td
						class="px-2 py-2.5 font-medium text-white transition-colors group-hover:text-[#196ee6]"
					>
						{issue.subject}
					</td>
					<td class="px-2 py-2.5">
						<div
							class="inline-flex items-center gap-1.5 rounded-full border px-2 py-0.5"
							style="border-color: {issue.status_color}20; background-color: {issue.status_color}10;"
						>
							<span class="size-1.5 rounded-full" style="background-color: {issue.status_color};"
							></span>
							<span class="text-xs font-medium" style="color: {issue.status_color};">
								{issue.status_name || issue.status}
							</span>
						</div>
					</td>
					<td class="px-2 py-2.5">
						<div class="flex items-center gap-1.5">
							<span class="text-xs text-[#93a9c8]">{getProjectName(issue.project)}</span>
						</div>
					</td>
					<td class="px-2 py-2.5">
						{#if issue.assigned_to_name}
							<div class="flex items-center gap-2">
								{#if issue.assigned_to_photo}
									<Tooltip.Root>
										<Tooltip.Trigger>
											<div
												class="size-6 rounded-full bg-cover bg-center bg-no-repeat ring-1 ring-[#243347]"
												style="background-image: url({issue.assigned_to_photo});"
											></div>
										</Tooltip.Trigger>
										<Tooltip.Content>
											<p>{issue.assigned_to_name}</p>
										</Tooltip.Content>
									</Tooltip.Root>
								{:else}
									<Tooltip.Root>
										<Tooltip.Trigger>
											<div
												class="flex size-6 items-center justify-center rounded-full bg-[#196ee6] text-[10px] font-medium text-white ring-1 ring-[#243347]"
											>
												{getInitials(issue.assigned_to_name)}
											</div>
										</Tooltip.Trigger>
										<Tooltip.Content>
											<p>{issue.assigned_to_name}</p>
										</Tooltip.Content>
									</Tooltip.Root>
								{/if}
							</div>
						{:else}
							<Tooltip.Root>
								<Tooltip.Trigger>
									<div
										class="flex size-6 items-center justify-center rounded-full border border-dashed border-[#93a9c8] bg-[#243347]"
									>
										<UserPlus class="h-3.5 w-3.5 text-[#93a9c8]" />
									</div>
								</Tooltip.Trigger>
								<Tooltip.Content>
									<p>{$t('table.unassigned')}</p>
								</Tooltip.Content>
							</Tooltip.Root>
						{/if}
					</td>
					<td class="px-2 py-2.5">
						{#if issue.modified_date}
							<div class="flex items-center gap-1.5 text-xs text-[#93a9c8]">
								<Clock class="h-3.5 w-3.5" />
								<span>{formatDate(issue.modified_date)}</span>
							</div>
						{:else}
							<span class="text-xs text-[#93a9c8]/50">—</span>
						{/if}
					</td>
				</tr>
			{/each}
		{/if}
	</tbody>
</table>
