<script lang="ts">
	import * as Table from '$lib/components/ui/table';
	import type { Issue, Project } from '$lib/types';
	import { Badge } from '$lib/components/ui/badge';
	import { t } from 'svelte-i18n';

	let {
		issues = [],
		projects = [],
		onIssueSelect
	}: {
		issues: Issue[];
		projects: Project[];
		onIssueSelect?: (issueId: number) => void;
	} = $props();

	function getProjectName(id: number) {
		return projects.find((p: Project) => p.id === id)?.name || id;
	}

	function handleRowClick(issueId: number) {
		if (onIssueSelect) {
			onIssueSelect(issueId);
		}
	}
</script>

<div class="rounded-md border">
	<Table.Root>
		<Table.Header>
			<Table.Row>
				<Table.Head>{$t('table.subject')}</Table.Head>
				<Table.Head>{$t('table.status')}</Table.Head>
				<Table.Head>{$t('table.project')}</Table.Head>
				<Table.Head>{$t('table.assignedTo')}</Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#if issues.length === 0}
				<Table.Row>
					<Table.Cell colspan={4} class="h-24 text-center">
						{$t('dashboard.noIssues')}
					</Table.Cell>
				</Table.Row>
			{:else}
				{#each issues as issue (issue.id)}
					<Table.Row
						class="hover:bg-muted/50 focus:ring-ring cursor-pointer transition-colors focus:ring-2 focus:outline-none"
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
						<Table.Cell class="font-medium">{issue.subject}</Table.Cell>
						<Table.Cell>
							<Badge
								variant="outline"
								style="border-color: {issue.status_color}; color: {issue.status_color}"
							>
								{issue.status_name || issue.status}
							</Badge>
						</Table.Cell>
						<Table.Cell>{getProjectName(issue.project)}</Table.Cell>
						<Table.Cell>{issue.assigned_to_name || $t('table.unassigned')}</Table.Cell>
					</Table.Row>
				{/each}
			{/if}
		</Table.Body>
	</Table.Root>
</div>
