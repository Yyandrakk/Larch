<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import { Plus, Folder, CircleDot, User } from '@lucide/svelte';
	import { t } from 'svelte-i18n';

	let {
		open = $bindable(false),
		buttonRef = $bindable<HTMLElement | null>(null),
		onSelectProject,
		onSelectStatus,
		onSelectAssignee,
		hasProjectFilter = false,
		hasStatusFilter = false,
		hasAssigneeFilter = false,
		hasPriorityFilter = false,
		hasSeverityFilter = false,
		hasTypeFilter = false,
		onSelectPriority,
		onSelectSeverity,
		onSelectType
	}: {
		open?: boolean;
		buttonRef?: HTMLElement | null;
		onSelectProject: () => void;
		onSelectStatus: () => void;
		onSelectAssignee: () => void;
		hasProjectFilter?: boolean;
		hasStatusFilter?: boolean;
		hasAssigneeFilter?: boolean;
		hasPriorityFilter?: boolean;
		hasSeverityFilter?: boolean;
		hasTypeFilter?: boolean;
		onSelectPriority?: () => void;
		onSelectSeverity?: () => void;
		onSelectType?: () => void;
	} = $props();

	function handleSelect(type: 'project' | 'status' | 'assignee') {
		open = false;
		if (type === 'project') onSelectProject();
		else if (type === 'status') onSelectStatus();
		else if (type === 'assignee') onSelectAssignee();
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger
		bind:ref={buttonRef}
		class="flex h-9 items-center gap-2 rounded-lg border border-dashed border-slate-600 px-3 text-sm font-medium text-slate-400 transition-all hover:border-[#196ee6] hover:bg-[#196ee6]/5 hover:text-[#196ee6]"
	>
		<Plus class="h-5 w-5" />
		<span>{$t('filters.addFilter')}</span>
	</Popover.Trigger>
	<Popover.Content
		class="w-[220px] overflow-hidden rounded-xl border border-[#2d3540] bg-[#1e2329] p-0 shadow-2xl ring-1 shadow-black/80 ring-white/5"
		align="start"
	>
		<div class="space-y-1 p-2">
			<div class="px-2 py-1.5 text-xs font-semibold tracking-wider text-slate-500 uppercase">
				{$t('filters.common')}
			</div>

			{#if !hasProjectFilter}
				<button
					class="group flex w-full items-center gap-3 rounded-lg px-3 py-2 text-slate-400 transition-colors hover:bg-[#2d3540] hover:text-white"
					onclick={() => handleSelect('project')}
				>
					<Folder class="h-[18px] w-[18px] transition-colors group-hover:text-[#196ee6]" />
					<span class="text-sm">{$t('filters.projects')}</span>
				</button>
			{/if}

			{#if !hasStatusFilter}
				<button
					class="group flex w-full items-center gap-3 rounded-lg px-3 py-2 text-slate-400 transition-colors hover:bg-[#2d3540] hover:text-white"
					onclick={() => handleSelect('status')}
				>
					<CircleDot class="h-[18px] w-[18px] transition-colors group-hover:text-emerald-400" />
					<span class="text-sm">{$t('filters.status')}</span>
				</button>
			{/if}

			{#if !hasAssigneeFilter}
				<button
					class="group flex w-full items-center gap-3 rounded-lg px-3 py-2 text-slate-400 transition-colors hover:bg-[#2d3540] hover:text-white"
					onclick={() => handleSelect('assignee')}
				>
					<User class="h-[18px] w-[18px] transition-colors group-hover:text-purple-400" />
					<span class="text-sm">{$t('filters.assignee')}</span>
				</button>
			{/if}

			{#if hasProjectFilter && hasStatusFilter && hasAssigneeFilter}
				<div class="px-3 py-2 text-sm text-slate-500 italic">
					{$t('filters.allFiltersActive')}
				</div>
			{/if}
		</div>
	</Popover.Content>
</Popover.Root>
