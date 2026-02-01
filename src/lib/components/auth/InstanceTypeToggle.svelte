<script lang="ts">
	import { Cloud, Server } from '@lucide/svelte';
	import { t } from 'svelte-i18n';

	let { value = $bindable<'cloud' | 'self'>('cloud'), name = 'instance_type' } = $props<{
		value?: 'cloud' | 'self';
		name?: string;
	}>();
</script>

<div class="flex flex-col gap-2">
	<label
		class="text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-[var(--login-muted)]"
	>
		{$t('login.instanceType')}
	</label>
	<div class="relative flex h-10 w-full items-center rounded-lg bg-gray-100 p-1 dark:bg-[#0f151e]">
		<label
			class="group relative flex flex-1 cursor-pointer items-center justify-center gap-2 rounded-[4px] py-1 text-sm font-medium transition-all hover:text-gray-900 dark:hover:text-white"
		>
			<input type="radio" {name} value="cloud" bind:group={value} class="peer sr-only" />
			<span
				class="absolute inset-0 rounded-[4px] bg-white opacity-0 shadow-sm transition-all peer-checked:opacity-100 dark:bg-[var(--login-border)]"
			></span>
			<Cloud
				class="relative z-10 h-[18px] w-[18px] text-gray-500 transition-colors peer-checked:text-[var(--login-primary)] dark:text-[var(--login-muted)] dark:peer-checked:text-white"
			/>
			<span
				class="relative z-10 text-gray-500 transition-colors peer-checked:text-gray-900 dark:text-[var(--login-muted)] dark:peer-checked:text-white"
			>
				{$t('login.cloudOption')}
			</span>
		</label>
		<label
			class="group relative flex flex-1 cursor-pointer items-center justify-center gap-2 rounded-[4px] py-1 text-sm font-medium transition-all hover:text-gray-900 dark:hover:text-white"
		>
			<input type="radio" {name} value="self" bind:group={value} class="peer sr-only" />
			<span
				class="absolute inset-0 rounded-[4px] bg-white opacity-0 shadow-sm transition-all peer-checked:opacity-100 dark:bg-[var(--login-border)]"
			></span>
			<Server
				class="relative z-10 h-[18px] w-[18px] text-gray-500 transition-colors peer-checked:text-[var(--login-primary)] dark:text-[var(--login-muted)] dark:peer-checked:text-white"
			/>
			<span
				class="relative z-10 text-gray-500 transition-colors peer-checked:text-gray-900 dark:text-[var(--login-muted)] dark:peer-checked:text-white"
			>
				{$t('login.selfHostedOption')}
			</span>
		</label>
	</div>
</div>
