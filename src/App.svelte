<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { isLoading } from 'svelte-i18n';
	import '$lib/i18n';
	import LoginScreen from '$lib/screens/LoginScreen.svelte';
	import ProjectConfigurationScreen from '$lib/screens/ProjectConfigurationScreen.svelte';
	import DashboardScreen from '$lib/screens/DashboardScreen.svelte';
	import { AppShell } from '$lib/components/layout';
	import { Toaster } from 'svelte-sonner';
	import { CMD_FORCE_CLOSE_APP } from '$lib/commands.svelte';
	import { hasPendingCommit, tryCommitPending } from '$lib/stores/pendingClose';

	type Screen = 'login' | 'projects' | 'dashboard';
	let currentScreen = $state<Screen>('login');
	let isCheckingAuth = $state(true);

	onMount(() => {
		const unlistenPromise = listen('app-close-requested', async () => {
			try {
				if (hasPendingCommit()) {
					const success = await tryCommitPending();
					if (success) {
						await invoke(CMD_FORCE_CLOSE_APP);
					}
				} else {
					await invoke(CMD_FORCE_CLOSE_APP);
				}
			} catch (e) {
				console.error('Failed to close app:', e);
			}
		});

		(async () => {
			try {
				const hasToken = await invoke<boolean>('has_api_token');
				if (hasToken) {
					try {
						await invoke('get_me');
						const selectedIds = await invoke<number[]>('get_selected_projects');
						if (selectedIds.length > 0) {
							currentScreen = 'dashboard';
						} else {
							currentScreen = 'projects';
						}
					} catch (e) {
						console.error('Token validation failed:', e);
						currentScreen = 'login';
					}
				} else {
					currentScreen = 'login';
				}
			} catch (e) {
				console.error('Auth check failed:', e);
				currentScreen = 'login';
			} finally {
				isCheckingAuth = false;
			}
		})();

		return () => {
			unlistenPromise.then((unlisten) => unlisten());
		};
	});

	function handleLoginSuccess() {
		currentScreen = 'projects';
	}

	function handleConfigContinue() {
		currentScreen = 'dashboard';
	}

	function handleNavigate(screen: 'projects' | 'dashboard') {
		currentScreen = screen;
	}

	function handleLogout() {
		currentScreen = 'login';
	}
</script>

<Toaster />

{#if $isLoading || isCheckingAuth}
	<main class="flex min-h-screen items-center justify-center bg-[#111821] p-4 select-none">
		<p class="text-white">Loading...</p>
	</main>
{:else if currentScreen === 'login'}
	<main
		class="bg-background-dark relative flex min-h-screen items-center justify-center text-white"
	>
		<div class="pointer-events-none fixed inset-0 z-0">
			<div
				class="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-blue-900/20 via-[#111821] to-[#111821]"
			></div>
			<div
				class="absolute top-0 left-0 h-px w-full bg-gradient-to-r from-transparent via-white/10 to-transparent"
			></div>
		</div>
		<LoginScreen onLoginSuccess={handleLoginSuccess} />
	</main>
{:else}
	<AppShell
		currentScreen={currentScreen as 'projects' | 'dashboard'}
		onNavigate={handleNavigate}
		onLogout={handleLogout}
	>
		{#if currentScreen === 'projects'}
			<ProjectConfigurationScreen onContinue={handleConfigContinue} />
		{:else if currentScreen === 'dashboard'}
			<DashboardScreen />
		{/if}
	</AppShell>
{/if}
