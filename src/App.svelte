<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { isLoading } from 'svelte-i18n';
	import '$lib/i18n'; // Import to initialize
	import LoginScreen from '$lib/screens/LoginScreen.svelte';
	import ProjectConfigurationScreen from '$lib/screens/ProjectConfigurationScreen.svelte';
	import DashboardScreen from '$lib/screens/DashboardScreen.svelte';
	import { Toaster } from 'svelte-sonner';
	import { CMD_FORCE_CLOSE_APP } from '$lib/commands.svelte';
	import { hasPendingCommit, tryCommitPending } from '$lib/stores/pendingClose';

	type Screen = 'login' | 'config' | 'dashboard';
	let currentScreen = $state<Screen>('login');
	let isCheckingAuth = $state(true);

	onMount(() => {
		// Listen for app close request from backend
		const unlistenPromise = listen('app-close-requested', async () => {
			try {
				if (hasPendingCommit()) {
					// Try to commit pending changes
					const success = await tryCommitPending();
					if (success) {
						// Commit succeeded, safe to close
						await invoke(CMD_FORCE_CLOSE_APP);
					}
					// If commit failed (conflict), don't close - the conflict modal will show
				} else {
					// No pending changes, safe to close
					await invoke(CMD_FORCE_CLOSE_APP);
				}
			} catch (e) {
				console.error('Failed to close app:', e);
			}
		});

		// Check auth status
		(async () => {
			try {
				const hasToken = await invoke<boolean>('has_api_token');
				if (hasToken) {
					// Verify token by fetching user details
					try {
						await invoke('get_me');
						// Token is valid, proceed
						const selectedIds = await invoke<number[]>('get_selected_projects');
						if (selectedIds.length > 0) {
							currentScreen = 'dashboard';
						} else {
							currentScreen = 'config';
						}
					} catch (e) {
						console.error('Token validation failed:', e);
						// Token invalid or network error. For now, assume invalid and go to login.
						// TODO: Handle network error differently (retry?)
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
		currentScreen = 'config';
	}

	function handleConfigContinue() {
		currentScreen = 'dashboard';
	}
</script>

<Toaster />

{#if $isLoading || isCheckingAuth}
	<main
		class="flex min-h-screen items-center justify-center bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 p-4 select-none"
	>
		<p class="text-white">Loading...</p>
	</main>
{:else}
	<main
		class="bg-background-light dark:bg-background-dark relative min-h-screen text-gray-900 dark:text-white"
		class:flex={currentScreen === 'login'}
		class:items-center={currentScreen === 'login'}
		class:justify-center={currentScreen === 'login'}
	>
		{#if currentScreen === 'login'}
			<!-- Background overlays from Stitch Reference -->
			<div class="pointer-events-none fixed inset-0 z-0">
				<div
					class="via-background-dark to-background-dark absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-blue-900/20"
				></div>
				<div
					class="absolute top-0 left-0 h-px w-full bg-gradient-to-r from-transparent via-white/10 to-transparent"
				></div>
			</div>
		{/if}

		{#if currentScreen === 'login'}
			<LoginScreen onLoginSuccess={handleLoginSuccess} />
		{:else if currentScreen === 'config'}
			<ProjectConfigurationScreen onContinue={handleConfigContinue} />
		{:else if currentScreen === 'dashboard'}
			<DashboardScreen />
		{/if}
	</main>
{/if}
