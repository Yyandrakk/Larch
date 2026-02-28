<script lang="ts">
	import { t } from 'svelte-i18n';
	import { SvelteMap } from 'svelte/reactivity';
	import type { Member, ProjectMetadata } from '$lib/types';
	import * as Popover from '$lib/components/ui/popover';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Search, UserSearch, UserX, Check, Combine, Ban } from '@lucide/svelte';

	const UNASSIGNED_ID = -1;

	let {
		metadata = {},
		selectedIds = [],
		isExclude = false,
		currentUserId,
		open = false,
		onApply,
		customAnchor = null
	}: {
		metadata: Record<number, ProjectMetadata>;
		selectedIds: number[];
		isExclude: boolean;
		currentUserId?: number;
		open?: boolean;
		onApply: (ids: number[], exclude: boolean) => void;
		customAnchor?: HTMLElement | null;
	} = $props();

	// Local state for the form - synced from props reactively
	let localSelectedIds = $state<number[]>([]);
	let localExclude = $state(false);
	let searchQuery = $state('');

	let wasOpen = false;
	$effect(() => {
		if (open && !wasOpen) {
			localSelectedIds = [...selectedIds];
			localExclude = isExclude;
		}
		wasOpen = open;
	});

	interface MemberWithRole extends Member {
		isCurrentUser: boolean;
	}

	let allMembers = $derived.by(() => {
		const memberMap = new SvelteMap<number, MemberWithRole>();

		Object.values(metadata).forEach((meta) => {
			meta.members.forEach((m) => {
				if (m.user_id && !memberMap.has(m.user_id)) {
					memberMap.set(m.user_id, {
						...m,
						isCurrentUser: m.user_id === currentUserId
					});
				}
			});
		});

		const members = Array.from(memberMap.values());
		members.sort((a, b) => {
			if (a.isCurrentUser) return -1;
			if (b.isCurrentUser) return 1;
			return a.full_name.localeCompare(b.full_name);
		});

		return members;
	});

	let filteredMembers = $derived(
		allMembers.filter((m) => m.full_name.toLowerCase().includes(searchQuery.toLowerCase()))
	);

	function toggleMember(id: number) {
		if (localSelectedIds.includes(id)) {
			localSelectedIds = localSelectedIds.filter((i) => i !== id);
		} else {
			localSelectedIds = [...localSelectedIds, id];
		}
	}

	function handleApply() {
		onApply(localSelectedIds, localExclude);
	}

	function handleClear() {
		localSelectedIds = [];
	}

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.substring(0, 2)
			.toUpperCase();
	}

	function getAvatarColor(name: string): string {
		const colors = [
			'bg-purple-500',
			'bg-indigo-500',
			'bg-blue-500',
			'bg-emerald-500',
			'bg-orange-500',
			'bg-pink-500'
		];
		const hash = name.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
		return colors[hash % colors.length];
	}

	let isUnassignedSelected = $derived(localSelectedIds.includes(UNASSIGNED_ID));
	let selectedCount = $derived(
		localSelectedIds.filter((id) => id !== UNASSIGNED_ID).length + (isUnassignedSelected ? 1 : 0)
	);

	let allVisibleIds = $derived([
		UNASSIGNED_ID,
		...filteredMembers.map((m) => m.user_id).filter((id): id is number => id != null)
	]);

	let areAllVisibleSelected = $derived(
		allVisibleIds.length > 0 && allVisibleIds.every((id) => localSelectedIds.includes(id))
	);
	let isAnyVisibleSelected = $derived(allVisibleIds.some((id) => localSelectedIds.includes(id)));

	let masterChecked = $derived(areAllVisibleSelected);
	let masterIndeterminate = $derived(!areAllVisibleSelected && isAnyVisibleSelected);

	function toggleAll() {
		if (masterChecked) {
			localSelectedIds = localSelectedIds.filter((id) => !allVisibleIds.includes(id));
		} else {
			const newIds = allVisibleIds.filter((id) => !localSelectedIds.includes(id));
			localSelectedIds = [...localSelectedIds, ...newIds];
		}
	}
</script>

<Popover.Content
	class="flex w-[340px] flex-col overflow-hidden rounded-xl border border-[#2d3540] bg-[#1e2329] p-0 shadow-2xl ring-1 shadow-black/80 ring-white/5"
	align="start"
	sideOffset={8}
	{customAnchor}
>
	<div class="border-b border-[#2d3540] bg-[#111821] p-3">
		<div class="mb-3 flex items-center justify-between">
			<div class="flex items-center gap-2 text-slate-400">
				<UserSearch class="h-[18px] w-[18px]" />
				<span class="text-xs font-bold tracking-wider uppercase"
					>{$t('filters.assigneeFilter')}</span
				>
			</div>
			<div class="flex h-7 items-center rounded-lg bg-[#2d3540] p-0.5">
				<button
					class="flex h-6 w-6 items-center justify-center rounded-md transition-all {!localExclude
						? 'bg-[#196ee6]/20 text-[#196ee6]'
						: 'text-slate-500 hover:bg-white/5 hover:text-slate-300'}"
					onclick={() => (localExclude = false)}
					title={$t('filters.include')}
				>
					<Combine class="h-3.5 w-3.5" />
				</button>
				<button
					class="flex h-6 w-6 items-center justify-center rounded-md transition-all {localExclude
						? 'bg-red-500/20 text-red-400'
						: 'text-slate-500 hover:bg-white/5 hover:text-slate-300'}"
					onclick={() => (localExclude = true)}
					title={$t('filters.exclude')}
				>
					<Ban class="h-3.5 w-3.5" />
				</button>
			</div>
		</div>
		<div class="group relative">
			<Search
				class="absolute top-2.5 left-3 h-4 w-4 text-slate-500 transition-colors {localExclude
					? 'group-focus-within:text-red-400'
					: 'group-focus-within:text-[#196ee6]'}"
			/>
			<input
				type="text"
				placeholder={$t('filters.searchPeople')}
				bind:value={searchQuery}
				class="w-full rounded-lg border border-[#2d3540] bg-[#1e2329] py-2 pr-3 pl-9 text-sm text-white transition-all placeholder:text-slate-600 focus:ring-1 focus:outline-none {localExclude
					? 'focus:border-red-500/50 focus:ring-red-500/20'
					: 'focus:border-[#196ee6]/50 focus:ring-[#196ee6]/20'}"
			/>
		</div>
	</div>

	<div class="flex items-center gap-3 border-b border-[#2d3540] bg-[#1e2329] px-3 py-2">
		<Checkbox
			checked={masterChecked}
			indeterminate={masterIndeterminate}
			onCheckedChange={toggleAll}
			class="border-slate-600 bg-[#111821] {localExclude ? 'text-red-500' : 'text-[#196ee6]'}"
		/>
		<span class="text-xs font-semibold text-slate-300">{$t('filters.selectAll')}</span>
	</div>

	<div class="custom-scrollbar max-h-[340px] space-y-0.5 overflow-y-auto bg-[#1e2329] p-1.5">
		<label
			class="group flex cursor-pointer items-center justify-between rounded-lg px-2 py-2 transition-all {isUnassignedSelected &&
			localExclude
				? 'border border-red-500/20 bg-red-500/10'
				: isUnassignedSelected
					? 'border border-[#196ee6]/20 bg-[#196ee6]/10'
					: 'hover:bg-[#2d3540]'}"
		>
			<div class="flex items-center gap-3">
				<Checkbox
					checked={isUnassignedSelected}
					onCheckedChange={() => toggleMember(UNASSIGNED_ID)}
					class="border-slate-600 bg-[#111821] {localExclude ? 'text-red-500' : 'text-[#196ee6]'}"
				/>
				<div class="flex items-center gap-3">
					<div
						class="flex size-6 items-center justify-center rounded-full border border-slate-600 bg-[#2d3540] text-slate-400"
					>
						<UserX class="h-3.5 w-3.5" />
					</div>
					<span class="text-sm text-slate-200 group-hover:text-white">
						{$t('filters.unassigned')}
					</span>
				</div>
			</div>
			{#if isUnassignedSelected}
				<Check class="h-[18px] w-[18px] {localExclude ? 'text-red-400' : 'text-[#196ee6]'}" />
			{/if}
		</label>

		{#if filteredMembers.length > 0}
			<div
				class="mt-1 mb-0.5 px-2 py-1.5 text-[10px] font-semibold tracking-wider text-slate-500 uppercase"
			>
				{$t('filters.suggestions')}
			</div>
		{/if}

		{#each filteredMembers as member (member.user_id)}
			{@const isSelected = localSelectedIds.includes(member.user_id!)}
			<label
				class="group flex cursor-pointer items-center justify-between rounded-lg px-2 py-2 transition-all {isSelected &&
				localExclude
					? 'border border-red-500/20 bg-red-500/10'
					: isSelected
						? 'border border-[#196ee6]/20 bg-[#196ee6]/10'
						: 'hover:bg-[#2d3540]'}"
			>
				<div class="flex items-center gap-3">
					<Checkbox
						checked={isSelected}
						onCheckedChange={() => toggleMember(member.user_id!)}
						class="border-slate-600 bg-[#111821] {isSelected
							? localExclude
								? 'border-red-500 text-red-500'
								: 'border-[#196ee6] text-[#196ee6]'
							: 'text-[#196ee6]'}"
					/>
					<div class="flex items-center gap-3">
						{#if member.photo}
							<div
								class="size-6 rounded-full bg-cover bg-center"
								style="background-image: url('{member.photo}');"
							></div>
						{:else}
							<div
								class="flex size-6 items-center justify-center rounded-full border border-current/50 text-[10px] font-bold text-white {getAvatarColor(
									member.full_name
								)}"
							>
								{getInitials(member.full_name)}
							</div>
						{/if}
						<div class="flex flex-col">
							<span
								class="text-sm font-medium {isSelected
									? 'text-white'
									: 'text-slate-200 group-hover:text-white'}"
							>
								{member.full_name}
							</span>
							{#if member.isCurrentUser}
								<span class="text-[10px] text-[#196ee6]/80">{$t('filters.you')}</span>
							{:else if member.role_name}
								<span class="text-[10px] text-slate-500">{member.role_name}</span>
							{/if}
						</div>
					</div>
				</div>
				{#if isSelected}
					<Check class="h-[18px] w-[18px] {localExclude ? 'text-red-400' : 'text-[#196ee6]'}" />
				{/if}
			</label>
		{/each}

		{#if filteredMembers.length === 0 && searchQuery}
			<div class="py-4 text-center text-sm text-slate-500">
				{$t('filters.noResults')}
			</div>
		{/if}
	</div>

	<div class="flex items-center justify-between border-t border-[#2d3540] bg-[#111821] p-3">
		<span class="text-xs text-slate-500">
			<strong class="text-white">{selectedCount}</strong>
			{localExclude ? $t('filters.excluded') : $t('filters.included')}
		</span>
		<div class="flex items-center gap-2">
			<button
				class="px-3 py-1.5 text-xs font-medium text-slate-400 transition-colors hover:text-white"
				onclick={handleClear}
			>
				{$t('filters.clear')}
			</button>
			<button
				class="rounded-lg px-3 py-1.5 text-xs font-semibold text-white shadow-lg transition-all {localExclude
					? 'bg-red-500 shadow-red-500/20 hover:bg-red-600'
					: 'bg-[#196ee6] shadow-[#196ee6]/20 hover:bg-blue-600'}"
				onclick={handleApply}
			>
				{$t('filters.apply')}
			</button>
		</div>
	</div>
</Popover.Content>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: #3f4a59;
		border-radius: 2px;
	}
</style>
