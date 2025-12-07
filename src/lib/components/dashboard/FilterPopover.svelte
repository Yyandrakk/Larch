<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import * as Command from '$lib/components/ui/command';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Check, PlusCircle } from '@lucide/svelte';
	import { cn } from '$lib/utils';
	import type { Project, FilterObject, ProjectMetadata, IssueStatus, Member } from '$lib/types';
	import { t } from 'svelte-i18n';
	import { Switch } from '$lib/components/ui/switch';
	import { Label } from '$lib/components/ui/label';

	let {
		projects = [],
		metadata = {},
		filters = {},
		onApply
	} = $props<{
		projects: Project[];
		metadata: Record<number, ProjectMetadata>;
		filters: FilterObject;
		onApply: (filters: FilterObject) => void;
	}>();

	let open = $state(false);

	// Local state for the form
	let selectedProjectIds = $state(filters.project_ids || []);
	let selectedStatusIds = $state(filters.status_ids || []);
	let selectedAssigneeIds = $state(filters.assignee_ids || []);
	let statusExclude = $state(filters.status_exclude || false);
	let assigneeExclude = $state(filters.assignee_exclude || false);

	// Derived options based on selected projects (or all projects if none selected)
	let availableStatuses = $derived.by(() => {
		const pids =
			selectedProjectIds.length > 0 ? selectedProjectIds : projects.map((p: Project) => p.id);
		const statuses = new Map<number, IssueStatus>();
		pids.forEach((pid: number) => {
			metadata[pid]?.statuses.forEach((s: IssueStatus) => statuses.set(s.id, s));
		});
		return Array.from(statuses.values());
	});

	let availableMembers = $derived.by(() => {
		const pids =
			selectedProjectIds.length > 0 ? selectedProjectIds : projects.map((p: Project) => p.id);
		const members = new Map<number, Member>();
		pids.forEach((pid: number) => {
			metadata[pid]?.members.forEach((m: Member) => {
				if (m.user_id) members.set(m.user_id, m);
			});
		});
		return Array.from(members.values());
	});

	function toggleProject(id: number) {
		if (selectedProjectIds.includes(id)) {
			selectedProjectIds = selectedProjectIds.filter((i: number) => i !== id);
		} else {
			selectedProjectIds = [...selectedProjectIds, id];
		}
	}

	function toggleStatus(id: number) {
		if (selectedStatusIds.includes(id)) {
			selectedStatusIds = selectedStatusIds.filter((i: number) => i !== id);
		} else {
			selectedStatusIds = [...selectedStatusIds, id];
		}
	}

	function toggleAssignee(id: number) {
		if (selectedAssigneeIds.includes(id)) {
			selectedAssigneeIds = selectedAssigneeIds.filter((i: number) => i !== id);
		} else {
			selectedAssigneeIds = [...selectedAssigneeIds, id];
		}
	}

	function apply() {
		onApply({
			project_ids: selectedProjectIds.length > 0 ? selectedProjectIds : undefined,
			status_ids: selectedStatusIds.length > 0 ? selectedStatusIds : undefined,
			assignee_ids: selectedAssigneeIds.length > 0 ? selectedAssigneeIds : undefined,
			status_exclude: statusExclude,
			assignee_exclude: assigneeExclude
		});
		open = false;
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger
		class="border-input hover:bg-accent hover:text-accent-foreground flex h-8 items-center justify-center rounded-md border border-dashed bg-transparent px-3 text-xs shadow-sm"
	>
		<PlusCircle class="mr-2 h-4 w-4" />
		{$t('filters.filter')}
	</Popover.Trigger>
	<Popover.Content class="w-[300px] p-0" align="start">
		<div class="space-y-4 p-4">
			<!-- Projects -->
			<div class="space-y-2">
				<h4 class="leading-none font-medium">{$t('filters.projects')}</h4>
				<div class="flex flex-wrap gap-2">
					{#each projects as project}
						<Badge
							variant={selectedProjectIds.includes(project.id) ? 'default' : 'outline'}
							class="cursor-pointer"
							onclick={() => toggleProject(project.id)}
						>
							{project.name}
						</Badge>
					{/each}
				</div>
			</div>

			<Separator />

			<!-- Status -->
			<div class="space-y-2">
				<div class="flex items-center justify-between">
					<h4 class="leading-none font-medium">{$t('filters.status')}</h4>
					<div class="flex items-center space-x-2">
						<Label for="status-mode" class="text-xs"
							>{statusExclude ? $t('filters.exclude') : $t('filters.include')}</Label
						>
						<Switch id="status-mode" bind:checked={statusExclude} />
					</div>
				</div>
				<div class="flex max-h-[150px] flex-wrap gap-2 overflow-y-auto">
					{#each availableStatuses as status}
						<Badge
							variant="outline"
							class={cn(
								'cursor-pointer',
								selectedStatusIds.includes(status.id)
									? 'bg-accent text-accent-foreground border-primary'
									: ''
							)}
							style={selectedStatusIds.includes(status.id)
								? `border-color: ${status.color}`
								: `border-left: 3px solid ${status.color}`}
							onclick={() => toggleStatus(status.id)}
						>
							{status.name}
						</Badge>
					{/each}
				</div>
			</div>

			<Separator />

			<!-- Assignee -->
			<div class="space-y-2">
				<div class="flex items-center justify-between">
					<h4 class="leading-none font-medium">{$t('filters.assignee')}</h4>
					<div class="flex items-center space-x-2">
						<Label for="assignee-mode" class="text-xs"
							>{assigneeExclude ? $t('filters.exclude') : $t('filters.include')}</Label
						>
						<Switch id="assignee-mode" bind:checked={assigneeExclude} />
					</div>
				</div>
				<div class="flex max-h-[150px] flex-wrap gap-2 overflow-y-auto">
					{#each availableMembers as member}
						<Badge
							variant={selectedAssigneeIds.includes(member.user_id!) ? 'default' : 'outline'}
							class="cursor-pointer"
							onclick={() => toggleAssignee(member.user_id!)}
						>
							{member.full_name}
						</Badge>
					{/each}
				</div>
			</div>

			<Separator />

			<Button class="w-full" onclick={apply}>Apply Filters</Button>
		</div>
	</Popover.Content>
</Popover.Root>
