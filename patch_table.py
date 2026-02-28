import re

with open('src/lib/components/dashboard/IssueTable.svelte', 'r') as f:
    content = f.read()

# 1. Update imports
content = content.replace("import type { Issue, Project } from '$lib/types';", "import type { Issue, Project, ProjectMetadata } from '$lib/types';")

# 2. Update props
content = content.replace(
"""	let {
		issues = [],
		projects = [],
		changedIssueIds = new SvelteSet<number>(),
		onIssueSelect
	}: {
		issues: Issue[];
		projects: Project[];
		changedIssueIds?: SvelteSet<number>;
		onIssueSelect?: (issueId: number) => void;
	} = $props();""",
"""	let {
		issues = [],
		projects = [],
		metadata = {},
		changedIssueIds = new SvelteSet<number>(),
		onIssueSelect
	}: {
		issues: Issue[];
		projects: Project[];
		metadata?: Record<number, ProjectMetadata>;
		changedIssueIds?: SvelteSet<number>;
		onIssueSelect?: (issueId: number) => void;
	} = $props();""")

# 3. Add helpers
helpers = """
	function resolvePriority(issue: Issue) {
		if (!issue.priority || !metadata[issue.project]) return null;
		return metadata[issue.project].priorities?.find((p) => p.id === issue.priority) || null;
	}

	function resolveSeverity(issue: Issue) {
		if (!issue.severity || !metadata[issue.project]) return null;
		return metadata[issue.project].severities?.find((s) => s.id === issue.severity) || null;
	}

	function resolveType(issue: Issue) {
		if (!issue.issue_type || !metadata[issue.project]) return null;
		return metadata[issue.project].issue_types?.find((t) => t.id === issue.issue_type) || null;
	}
"""
content = content.replace("""	function getProjectName(id: number): string {
		return projects.find((p: Project) => p.id === id)?.name || `Project ${id}`;
	}""", """	function getProjectName(id: number): string {
		return projects.find((p: Project) => p.id === id)?.name || `Project ${id}`;
	}""" + helpers)


# 4. Add headers
headers = """			<th
				class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('table.status')}
			</th>
			<th
				class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('table.prioritySeverity')}
			</th>
			<th
				class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('table.type')}
			</th>"""
content = content.replace("""			<th
				class="border-b border-[#243347] px-2 py-3 text-xs font-semibold tracking-wider text-[#93a9c8] uppercase"
			>
				{$t('table.status')}
			</th>""", headers)

# 5. Update colspan
content = content.replace('<td colspan="6"', '<td colspan="8"')

# 6. Add columns
columns = """					<td class="px-2 py-2.5">
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
						<div class="flex flex-col gap-1">
							{@const priority = resolvePriority(issue)}
							{@const severity = resolveSeverity(issue)}
							{#if priority}
								<div class="flex items-center gap-1.5 text-xs">
									<span class="size-1.5 rounded-full" style="background-color: {priority.color};"></span>
									<span class="text-[#93a9c8]">{priority.name}</span>
								</div>
							{:else}
								<span class="text-xs text-[#93a9c8]/50">—</span>
							{/if}
							{#if severity}
								<div class="flex items-center gap-1.5 text-xs">
									<span class="size-1.5 rounded-full" style="background-color: {severity.color};"></span>
									<span class="text-[#93a9c8]">{severity.name}</span>
								</div>
							{:else}
								<span class="text-xs text-[#93a9c8]/50">—</span>
							{/if}
						</div>
					</td>
					<td class="px-2 py-2.5">
						{@const type = resolveType(issue)}
						{#if type}
							<div
								class="inline-flex items-center gap-1.5 rounded-full border px-2 py-0.5"
								style="border-color: {type.color}20; background-color: {type.color}10;"
							>
								<span class="size-1.5 rounded-full" style="background-color: {type.color};"></span>
								<span class="text-xs font-medium" style="color: {type.color};">
									{type.name}
								</span>
							</div>
						{:else}
							<span class="text-xs text-[#93a9c8]/50">—</span>
						{/if}
					</td>"""

content = content.replace("""					<td class="px-2 py-2.5">
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
					</td>""", columns)

with open('src/lib/components/dashboard/IssueTable.svelte', 'w') as f:
    f.write(content)
print("Patcher executed successfully.")
