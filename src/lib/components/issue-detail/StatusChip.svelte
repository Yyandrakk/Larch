<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { RefreshCw, Check, Loader2, AlertTriangle } from '@lucide/svelte';
	import { t } from 'svelte-i18n';

	type Status = 'saved' | 'saving' | 'collision';

	let {
		status,
		onRefresh
	}: {
		status: Status;
		onRefresh?: () => void;
	} = $props();

	const statusConfig = {
		saved: {
			variant: 'secondary' as const,
			icon: Check,
			labelKey: 'issueDetail.saved',
			fallback: 'Saved',
			className: 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400',
			iconClass: ''
		},
		saving: {
			variant: 'secondary' as const,
			icon: Loader2,
			labelKey: 'issueDetail.saving',
			fallback: 'Saving...',
			className: 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400',
			iconClass: 'animate-spin'
		},
		collision: {
			variant: 'destructive' as const,
			icon: AlertTriangle,
			labelKey: 'issueDetail.collision',
			fallback: 'Collision',
			className: 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400',
			iconClass: ''
		}
	};

	let config = $derived(statusConfig[status]);
</script>

<div class="flex items-center gap-2">
	<Badge variant={config.variant} class="flex items-center gap-1.5 px-2.5 py-1 {config.className}">
		{#if status === 'saved'}
			<Check class="h-3.5 w-3.5" />
		{:else if status === 'saving'}
			<Loader2 class="h-3.5 w-3.5 animate-spin" />
		{:else}
			<AlertTriangle class="h-3.5 w-3.5" />
		{/if}
		<span class="text-xs font-medium">
			{$t(config.labelKey) || config.fallback}
		</span>
	</Badge>

	{#if status === 'collision' && onRefresh}
		<Button variant="ghost" size="sm" onclick={onRefresh} class="h-7 gap-1 px-2 text-xs">
			<RefreshCw class="h-3 w-3" />
			{$t('issueDetail.refresh') || 'Refresh'}
		</Button>
	{/if}
</div>
