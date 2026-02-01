<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { t } from 'svelte-i18n';

	let {
		open = $bindable(false),
		onSave
	}: {
		open: boolean;
		onSave: (name: string) => void;
	} = $props();

	let name = $state('');

	function handleSave() {
		const trimmedName = name.trim();
		if (trimmedName) {
			onSave(trimmedName);
			open = false;
			name = '';
		}
	}

	function handleCancel() {
		open = false;
		name = '';
	}

	$effect(() => {
		if (open) {
			name = '';
		}
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>{$t('views.saveAs')}</Dialog.Title>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<Input
				id="view-name"
				bind:value={name}
				placeholder={$t('views.namePlaceholder')}
				autofocus
				onkeydown={(e) => e.key === 'Enter' && handleSave()}
			/>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={handleCancel}>
				{$t('common.cancel')}
			</Button>
			<Button onclick={handleSave} disabled={!name.trim()}>
				{$t('views.save')}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
