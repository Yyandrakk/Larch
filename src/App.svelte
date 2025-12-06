<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { isLoading } from 'svelte-i18n';
	import '$lib/i18n'; // Import to initialize
	import LoginScreen from '$lib/screens/LoginScreen.svelte';
	import ProjectConfigurationScreen from '$lib/screens/ProjectConfigurationScreen.svelte';
	import DashboardScreen from '$lib/screens/DashboardScreen.svelte';
	import { Toaster } from 'svelte-sonner';

	type Screen = 'login' | 'config' | 'dashboard';
	let currentScreen = $state<Screen>('login');
	let isCheckingAuth = $state(true);

	onMount(async () => {
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
		class="bg-background text-foreground min-h-screen"
		class:flex={currentScreen === 'login'}
		class:items-center={currentScreen === 'login'}
		class:justify-center={currentScreen === 'login'}
		class:bg-gradient-to-br={currentScreen === 'login'}
		class:from-indigo-500={currentScreen === 'login'}
		class:via-purple-500={currentScreen === 'login'}
		class:to-pink-500={currentScreen === 'login'}
	>
		{#if currentScreen === 'login'}
			<LoginScreen onLoginSuccess={handleLoginSuccess} />
		{:else if currentScreen === 'config'}
			<ProjectConfigurationScreen onContinue={handleConfigContinue} />
		{:else if currentScreen === 'dashboard'}
			<DashboardScreen />
		{/if}
	</main>
{/if}
