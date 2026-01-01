<script lang="ts">
	import type { Component } from 'svelte';
	import { Eye, EyeOff } from '@lucide/svelte';

	let {
		value = $bindable(''),
		placeholder = '',
		type = 'text',
		icon: Icon,
		disabled = false,
		required = false,
		id
	} = $props<{
		value?: string;
		placeholder?: string;
		type?: string;
		icon?: Component;
		disabled?: boolean;
		required?: boolean;
		id?: string;
	}>();

	let showPassword = $state(false);

	function togglePassword() {
		showPassword = !showPassword;
	}

	let inputType = $derived(type === 'password' && showPassword ? 'text' : type);
</script>

<div class="relative flex items-center">
	{#if Icon}
		<span class="absolute left-3 text-gray-400 dark:text-[var(--login-muted)]">
			<Icon class="h-5 w-5" />
		</span>
	{/if}

	<input
		{id}
		type={inputType}
		bind:value
		class="w-full rounded-lg border border-gray-300 bg-gray-50 py-2.5 text-sm text-gray-900 placeholder-gray-400 focus:border-[var(--login-primary)] focus:ring-1 focus:ring-[var(--login-primary)] dark:border-[#344865] dark:bg-[#1a2432] dark:text-white dark:placeholder-[#6a7e99] dark:focus:border-[var(--login-primary)]"
		class:pl-10={!!Icon}
		class:pr-3={type !== 'password'}
		class:pr-10={type === 'password'}
		{placeholder}
		{disabled}
		{required}
	/>

	{#if type === 'password'}
		<button
			type="button"
			class="absolute right-3 flex items-center text-gray-400 hover:text-gray-600 dark:text-[var(--login-muted)] dark:hover:text-gray-300"
			onclick={togglePassword}
			tabindex="-1"
		>
			{#if showPassword}
				<EyeOff class="h-5 w-5" />
			{:else}
				<Eye class="h-5 w-5" />
			{/if}
		</button>
	{/if}
</div>
