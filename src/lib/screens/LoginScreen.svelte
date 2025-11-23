<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Button } from '$lib/components/ui/button';
	import {
		Card,
		CardContent,
		CardDescription,
		CardFooter,
		CardHeader,
		CardTitle
	} from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Cog } from '@lucide/svelte';

	let taigaUrl = $state('https://api.taiga.io');
	let username = $state('');
	let password = $state('');
	let showAdvanced = $state(false);
	let loading = $state(false);
	let errorMsg = $state<string | null>(null);

	function toggleAdvanced() {
		showAdvanced = !showAdvanced;
	}

	async function handleLogin() {
		loading = true;
		errorMsg = null;
		try {
			const user = await invoke('login', {
				apiUrl: taigaUrl,
				username,
				password
			});
			console.log('Login successful for user:', user);
			// TODO: Navigate to the next screen (Project Selection)
		} catch (err) {
			console.error('Login failed:', err);
			if (err && typeof err === 'object' && 'TaigaClient' in err) {
				errorMsg = err.TaigaClient as string;
			} else {
				errorMsg = 'An unknown error occurred.';
			}
		} finally {
			loading = false;
		}
	}
</script>

<Card class="w-full max-w-md">
	<CardHeader>
		<div class="flex items-center justify-between">
			<div>
				<CardTitle class="text-2xl">Larch</CardTitle>
				<CardDescription>Login to your Taiga instance</CardDescription>
			</div>
			<Button variant="ghost" size="icon" onclick={toggleAdvanced}>
				<Cog class="h-6 w-6" />
			</Button>
		</div>
	</CardHeader>
	<CardContent class="grid gap-4">
		{#if showAdvanced}
			<div class="grid gap-2">
				<Label for="taiga-url">Taiga API URL</Label>
				<Input
					id="taiga-url"
					bind:value={taigaUrl}
					placeholder="https://api.taiga.io"
					disabled={loading}
				/>
				<p class="text-muted-foreground text-xs">
					Enter the base URL of your Taiga instance (without /api/v1).
				</p>
			</div>
		{/if}
		<div class="grid gap-2">
			<Label for="username">Username</Label>
			<Input id="username" bind:value={username} placeholder="user" disabled={loading} />
		</div>
		<div class="grid gap-2">
			<Label for="password">Password</Label>
			<Input id="password" type="password" bind:value={password} disabled={loading} />
		</div>
		{#if errorMsg}
			<p class="text-sm text-red-500">{errorMsg}</p>
		{/if}
	</CardContent>
	<CardFooter>
		<Button class="w-full" onclick={handleLogin} disabled={loading}>
			{#if loading}
				<svg
					class="mr-3 -ml-1 h-5 w-5 animate-spin text-white"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
					></circle>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
					></path>
				</svg>
			{/if}
			{loading ? 'Logging in...' : 'Login'}
		</Button>
	</CardFooter>
</Card>
