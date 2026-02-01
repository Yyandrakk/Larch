<script lang="ts">
	import { X } from '@lucide/svelte';
	import type { Component } from 'svelte';

	let {
		label,
		value,
		isExclude = false,
		icon: Icon,
		avatarUrl,
		onRemove,
		onClick
	}: {
		label: string;
		value: string;
		isExclude?: boolean;
		icon?: Component;
		avatarUrl?: string;
		onRemove: () => void;
		onClick?: () => void;
	} = $props();

	// Dynamic classes based on exclude mode
	let chipClasses = $derived(
		isExclude
			? 'bg-red-500/10 hover:bg-red-500/20 border-red-500/30 hover:border-red-500/50'
			: 'bg-[#2d3540]/50 hover:bg-[#2d3540] border-[#2d3540] hover:border-slate-600'
	);

	let labelClasses = $derived(isExclude ? 'text-red-400' : 'text-slate-400');

	let valueClasses = $derived(isExclude ? 'text-red-100' : 'text-white');

	let iconClasses = $derived(isExclude ? 'text-red-400' : 'text-slate-400');

	let closeButtonClasses = $derived(
		isExclude
			? 'hover:bg-red-500/20 text-red-300 hover:text-white'
			: 'hover:bg-white/10 text-slate-400 hover:text-white'
	);

	function handleClick() {
		onClick?.();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			onClick?.();
		}
	}

	function handleRemove(e: MouseEvent) {
		e.stopPropagation();
		onRemove();
	}
</script>

<div
	class="group flex h-9 cursor-pointer items-center rounded-lg border py-1.5 pr-2 pl-3 transition-all select-none {chipClasses}"
	role="button"
	tabindex="0"
	onclick={handleClick}
	onkeydown={handleKeydown}
>
	<div class="flex items-center gap-2">
		{#if Icon}
			<Icon class="h-[18px] w-[18px] {iconClasses}" />
		{/if}
		<span class="text-xs font-medium tracking-wider uppercase {labelClasses}">
			{label}:
		</span>
		<div class="flex items-center gap-1.5">
			{#if avatarUrl}
				<div
					class="size-4 rounded-full bg-cover bg-center"
					style="background-image: url('{avatarUrl}');"
				></div>
			{/if}
			<span class="text-sm font-medium {valueClasses}">
				{value}
			</span>
		</div>
	</div>
	<button
		class="ml-2 rounded-full p-0.5 {closeButtonClasses}"
		onclick={handleRemove}
		type="button"
		aria-label="Remove filter"
	>
		<X class="h-4 w-4" />
	</button>
</div>
