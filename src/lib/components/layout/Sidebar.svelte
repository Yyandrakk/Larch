<script lang="ts">
	import { t } from 'svelte-i18n';
	import { LayoutDashboard, FolderOpen } from '@lucide/svelte';

	type Screen = 'projects' | 'dashboard';
	let {
		currentScreen,
		onNavigate
	}: {
		currentScreen: Screen;
		onNavigate: (screen: Screen) => void;
	} = $props();
</script>

<aside class="border-border bg-background flex w-64 flex-shrink-0 flex-col border-r">
	<div class="flex flex-col gap-4 p-4">
		<div class="mb-4 flex items-center gap-3">
			<div
				class="from-brand-primary flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br to-blue-600 shadow-lg"
			>
				<LayoutDashboard class="h-6 w-6 text-white" />
			</div>
			<div class="flex flex-col">
				<h1 class="text-base leading-normal font-bold tracking-tight text-white">
					{$t('app.title')}
				</h1>
				<p class="text-muted-foreground text-xs font-normal">{$t('nav.workspace')}</p>
			</div>
		</div>

		<nav class="flex flex-col gap-1">
			<button
				onclick={() => onNavigate('dashboard')}
				class="flex items-center gap-3 rounded-lg px-3 py-2 transition-colors {currentScreen ===
				'dashboard'
					? 'bg-sidebar-accent text-white'
					: 'text-muted-foreground hover:bg-sidebar-accent/50 hover:text-white'}"
			>
				<LayoutDashboard
					class="h-5 w-5 {currentScreen === 'dashboard' ? 'text-brand-primary' : ''}"
				/>
				<span class="text-sm font-medium">{$t('nav.dashboard')}</span>
			</button>
			<button
				onclick={() => onNavigate('projects')}
				class="flex items-center gap-3 rounded-lg px-3 py-2 transition-colors {currentScreen ===
				'projects'
					? 'bg-sidebar-accent text-white'
					: 'text-muted-foreground hover:bg-sidebar-accent/50 hover:text-white'}"
			>
				<FolderOpen class="h-5 w-5 {currentScreen === 'projects' ? 'text-brand-primary' : ''}" />
				<span class="text-sm font-medium">{$t('nav.projects')}</span>
			</button>
		</nav>
	</div>
</aside>
