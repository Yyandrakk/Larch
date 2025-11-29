<script lang="ts">
	import { isLoading } from 'svelte-i18n';
	import '$lib/i18n'; // Import to initialize
	import LoginScreen from '$lib/screens/LoginScreen.svelte';
</script>

{#if $isLoading}
	<main
		class="flex min-h-screen items-center justify-center bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 p-4 select-none"
	>
		<p class="text-white">Loading...</p>
	</main>
{:else}
	<main
		class="flex min-h-screen items-center justify-center bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 p-4 select-none"
	>
		<LoginScreen />
	</main>
{/if}
