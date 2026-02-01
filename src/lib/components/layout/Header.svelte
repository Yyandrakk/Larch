<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { t } from 'svelte-i18n';
	import { ChevronDown, LogOut, Bell } from '@lucide/svelte';
	import { CMD_LOGOUT } from '$lib/commands.svelte';
	import { toast } from 'svelte-sonner';
	import {
		getUserDisplayName,
		getUserInitials,
		getUserPhoto,
		clearCurrentUser
	} from '$lib/stores/user.svelte';

	let { onLogout }: { onLogout: () => void } = $props();
	let dropdownOpen = $state(false);

	let displayName = $derived(getUserDisplayName());
	let initials = $derived(getUserInitials());
	let photoUrl = $derived(getUserPhoto());

	async function handleLogout() {
		try {
			await invoke(CMD_LOGOUT);
			clearCurrentUser();
			toast.success($t('header.loggedOut'));
			onLogout();
		} catch (error) {
			console.error('Logout failed:', error);
			toast.error($t('errors.unknown'));
		}
	}

	function closeDropdown() {
		dropdownOpen = false;
	}

	function toggleDropdown() {
		dropdownOpen = !dropdownOpen;
	}
</script>

<header
	class="z-50 flex h-14 items-center justify-between border-b border-[#243347] bg-[#111822]/95 px-6 backdrop-blur"
>
	<div class="flex flex-1 items-center gap-4"></div>

	<div class="flex items-center gap-3">
		<button
			class="relative flex size-8 items-center justify-center rounded text-[#93a9c8] transition-colors hover:bg-[#243347] hover:text-white"
		>
			<Bell class="h-5 w-5" />
			<span
				class="absolute top-1.5 right-1.5 size-2 rounded-full border-2 border-[#111822] bg-[#196ee6]"
			></span>
		</button>

		<div class="mx-1 h-6 w-px bg-[#243347]"></div>

		<div class="relative">
			<button
				onclick={toggleDropdown}
				class="flex items-center gap-2 rounded-full py-1 pr-2 pl-1 transition-colors hover:bg-[#243347]"
			>
				{#if photoUrl}
					<img
						src={photoUrl}
						alt={displayName}
						class="h-7 w-7 rounded-full border border-[#243347] object-cover"
					/>
				{:else}
					<div
						class="flex h-7 w-7 items-center justify-center rounded-full border border-[#243347] bg-[#196ee6] text-xs font-medium text-white"
					>
						{initials}
					</div>
				{/if}
				<span class="hidden text-sm font-medium text-white md:block">{displayName}</span>
				<ChevronDown class="h-4 w-4 text-[#93a9c8]" />
			</button>

			{#if dropdownOpen}
				<div
					class="absolute top-full right-0 z-50 mt-1 w-48 rounded-lg border border-[#243347] bg-[#1e293b] shadow-xl"
				>
					<div class="py-1">
						<button
							onclick={handleLogout}
							class="flex w-full items-center gap-2 px-4 py-2 text-sm text-red-400 transition-colors hover:bg-[#243347] hover:text-red-300"
						>
							<LogOut class="h-4 w-4" />
							{$t('header.logout')}
						</button>
					</div>
				</div>
			{/if}
		</div>
	</div>
</header>

{#if dropdownOpen}
	<div
		class="fixed inset-0 z-40"
		onclick={closeDropdown}
		onkeydown={(e) => {
			if (e.key === 'Escape') closeDropdown();
		}}
		role="button"
		tabindex="0"
	></div>
{/if}
