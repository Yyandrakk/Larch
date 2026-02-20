<script lang="ts">
	import Sidebar from './Sidebar.svelte';
	import Header from './Header.svelte';
	import type { Snippet } from 'svelte';
	import { TooltipProvider } from '$lib/components/ui/tooltip';

	type Screen = 'projects' | 'dashboard';
	let {
		currentScreen,
		onNavigate,
		onLogout,
		children
	}: {
		currentScreen: Screen;
		onNavigate: (screen: Screen) => void;
		onLogout: () => void;
		children: Snippet;
	} = $props();
</script>

<TooltipProvider>
	<div class="flex h-screen w-full overflow-hidden bg-[#111821] font-sans text-white">
		<Sidebar {currentScreen} {onNavigate} />

		<main class="relative flex min-w-0 flex-1 flex-col bg-[#111821]">
			<Header {onLogout} />

			<div class="flex flex-1 flex-col overflow-hidden">
				{@render children()}
			</div>
		</main>
	</div>
</TooltipProvider>
