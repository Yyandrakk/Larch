<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Card from '$lib/components/ui/card';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import Label from '$lib/components/ui/label/label.svelte';
	import { toast } from 'svelte-sonner';

	type Project = {
		id: number;
		name: string;
		description: string;
		slug: string;
	};

	let { onContinue } = $props<{ onContinue: () => void }>();

	let projects = $state<Project[]>([]);
	let selectedProjectIds = $state<number[]>([]);
	let isLoading = $state(true);
	let isSaving = $state(false);

	onMount(async () => {
		try {
			const [allProjects, savedIds] = await Promise.all([
				invoke<Project[]>('get_projects'),
				invoke<number[]>('get_selected_projects')
			]);
			projects = allProjects;
			selectedProjectIds = savedIds;
		} catch (error) {
			console.error('Failed to load projects:', error);
			toast.error('Failed to load projects');
		} finally {
			isLoading = false;
		}
	});

	async function handleSave() {
		isSaving = true;
		try {
			// Ensure IDs are i64 compatible (numbers in JS are doubles, but should be fine)
			// Explicitly cast to array of numbers to be safe
			const idsToSave = selectedProjectIds.map((id) => Number(id));
			await invoke('save_selected_projects', { projectIds: idsToSave });
			toast.success('Project selection saved');
			onContinue();
		} catch (error) {
			console.error('Failed to save selection:', error);
			toast.error('Failed to save selection');
		} finally {
			isSaving = false;
		}
	}

	function toggleProject(projectId: number) {
		if (selectedProjectIds.includes(projectId)) {
			selectedProjectIds = selectedProjectIds.filter((id) => id !== projectId);
		} else {
			selectedProjectIds = [...selectedProjectIds, projectId];
		}
	}
</script>

<div class="container mx-auto max-w-2xl py-10">
	<Card.Root>
		<Card.Header>
			<Card.Title>Select Projects</Card.Title>
			<Card.Description>
				Choose the projects you want to monitor in your dashboard.
			</Card.Description>
		</Card.Header>
		<Card.Content>
			{#if isLoading}
				<div class="flex justify-center p-4">
					<span class="loading">Loading projects...</span>
				</div>
			{:else if projects.length === 0}
				<div class="text-muted-foreground p-4 text-center">No projects found.</div>
			{:else}
				<div class="max-h-[60vh] space-y-4 overflow-y-auto pr-2">
					{#each projects as project (project.id)}
						<div
							class="hover:bg-accent/50 flex items-start space-x-3 rounded-md border p-3 transition-colors"
						>
							<Checkbox
								id="project-{project.id}"
								checked={selectedProjectIds.includes(project.id)}
								onCheckedChange={() => toggleProject(project.id)}
							/>
							<div class="grid gap-1.5 leading-none">
								<Label
									for="project-{project.id}"
									class="cursor-pointer text-sm leading-none font-medium peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
								>
									{project.name}
								</Label>
								<p class="text-muted-foreground text-xs">
									{project.slug}
								</p>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</Card.Content>
		<Card.Footer class="flex justify-end space-x-2">
			<Button onclick={handleSave} disabled={isLoading || isSaving}>
				{isSaving ? 'Saving...' : 'Continue to Dashboard'}
			</Button>
		</Card.Footer>
	</Card.Root>
</div>
