<script lang="ts">
	import { tick } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { t } from 'svelte-i18n';
	import { Bold, Italic, Code, Link, List, Eye, Edit3, Send, Loader2 } from '@lucide/svelte';

	let {
		value = $bindable(''),
		placeholder = '',
		disabled = false,
		submitting = false,
		onSubmit,
		onUpload
	}: {
		value?: string;
		placeholder?: string;
		disabled?: boolean;
		submitting?: boolean;
		onSubmit?: (text: string) => void;
		onUpload?: (file: File) => Promise<string | undefined>;
	} = $props();

	let showPreview = $state(false);
	let textareaRef = $state<HTMLTextAreaElement | null>(null);
	let uploading = $state(false);

	// ============================================================================
	// Helper: Apply selection after DOM update
	// ============================================================================

	async function setSelectionAndFocus(start: number, end: number) {
		await tick(); // Wait for Svelte to update the DOM
		textareaRef?.setSelectionRange(start, end);
		textareaRef?.focus();
	}

	// ============================================================================
	// Markdown Insertion Helpers
	// ============================================================================

	async function insertMarkdown(prefix: string, suffix: string = '') {
		if (!textareaRef) return;

		const start = textareaRef.selectionStart;
		const end = textareaRef.selectionEnd;
		const selectedText = value.substring(start, end);

		const before = value.substring(0, start);
		const after = value.substring(end);

		if (selectedText) {
			// Wrap selected text
			value = before + prefix + selectedText + suffix + after;
			await setSelectionAndFocus(
				start + prefix.length,
				start + prefix.length + selectedText.length
			);
		} else {
			// Insert at cursor with placeholder text
			const placeholderText = 'text';
			value = before + prefix + placeholderText + suffix + after;
			await setSelectionAndFocus(
				start + prefix.length,
				start + prefix.length + placeholderText.length
			);
		}
	}

	async function insertLink() {
		if (!textareaRef) return;

		const start = textareaRef.selectionStart;
		const end = textareaRef.selectionEnd;
		const selectedText = value.substring(start, end);

		const before = value.substring(0, start);
		const after = value.substring(end);

		if (selectedText) {
			// Wrap selected text as link text, select "url" for replacement
			value = before + '[' + selectedText + '](url)' + after;
			const urlStart = start + selectedText.length + 3;
			await setSelectionAndFocus(urlStart, urlStart + 3);
		} else {
			// Insert link template
			await insertMarkdown('[', '](url)');
		}
	}

	async function insertListItem() {
		if (!textareaRef) return;

		const start = textareaRef.selectionStart;

		// Find the start of the current line
		let lineStart = start;
		while (lineStart > 0 && value[lineStart - 1] !== '\n') {
			lineStart--;
		}

		const before = value.substring(0, lineStart);
		const after = value.substring(lineStart);

		value = before + '- ' + after;
		await setSelectionAndFocus(lineStart + 2, lineStart + 2);
	}

	// ============================================================================
	// Event Handlers
	// ============================================================================

	async function handlePaste(e: ClipboardEvent) {
		if (!onUpload || disabled || submitting || uploading) return;

		const items = e.clipboardData?.items;
		if (!items) return;

		for (const item of items) {
			if (item.type.startsWith('image/')) {
				const file = item.getAsFile();
				if (!file) continue;

				e.preventDefault();
				uploading = true;

				try {
					const markdown = await onUpload(file);
					if (markdown) {
						// Insert markdown at cursor
						const toInsert = markdown.startsWith('!') ? markdown : `![${file.name}](${markdown})`;
						await insertTextAtCursor(toInsert);
					}
				} catch (error) {
					console.error('Failed to upload image:', error);
				} finally {
					uploading = false;
				}
				return; // Handle only the first image
			}
		}
	}

	async function insertTextAtCursor(text: string) {
		if (!textareaRef) return;
		const start = textareaRef.selectionStart;
		const end = textareaRef.selectionEnd;
		const before = value.substring(0, start);
		const after = value.substring(end);

		value = before + text + after;
		await setSelectionAndFocus(start + text.length, start + text.length);
	}

	function handleKeydown(e: KeyboardEvent) {
		// Shift+Enter to submit
		if (e.key === 'Enter' && e.shiftKey) {
			e.preventDefault();
			handleSubmit();
		}
	}

	function handleSubmit() {
		if (!value.trim() || disabled || submitting) return;
		onSubmit?.(value);
	}

	// ============================================================================
	// Markdown Rendering (Preview Mode)
	// ============================================================================

	function escapeHtml(text: string): string {
		return text
			.replace(/&/g, '&amp;')
			.replace(/</g, '&lt;')
			.replace(/>/g, '&gt;')
			.replace(/"/g, '&quot;')
			.replace(/'/g, '&#039;');
	}

	function sanitizeUrl(url: string): string {
		const trimmed = url.trim().toLowerCase();
		if (
			trimmed.startsWith('http://') ||
			trimmed.startsWith('https://') ||
			trimmed.startsWith('mailto:') ||
			trimmed.startsWith('tel:') ||
			trimmed.startsWith('#') ||
			trimmed.startsWith('/')
		) {
			return url;
		}
		return '#';
	}

	function renderMarkdown(text: string): string {
		if (!text) return '';

		// IMPORTANT: Escape HTML FIRST to prevent XSS
		let result = escapeHtml(text);

		// Apply markdown transformations on escaped text
		return (
			result
				// Bold (must come before italic)
				.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
				// Italic
				.replace(/\*(.+?)\*/g, '<em>$1</em>')
				// Code
				.replace(/`(.+?)`/g, '<code class="bg-muted px-1 rounded">$1</code>')
				// Links
				.replace(/\[(.+?)\]\((.+?)\)/g, (_, linkText, url) => {
					const safeUrl = sanitizeUrl(url);
					return `<a href="${safeUrl}" class="text-primary underline" target="_blank" rel="noopener noreferrer">${linkText}</a>`;
				})
				// List items
				.replace(/^- (.+)$/gm, '• $1')
				// Line breaks
				.replace(/\n/g, '<br>')
		);
	}

	// ============================================================================
	// Derived State
	// ============================================================================

	let canSubmit = $derived(value.trim().length > 0 && !disabled && !submitting && !uploading);
	let isEditorDisabled = $derived(disabled || showPreview || uploading);
</script>

<div class="space-y-2">
	<!-- Toolbar -->
	<div class="flex items-center gap-1 border-b pb-2">
		<Button
			variant="ghost"
			size="icon"
			class="h-8 w-8"
			onclick={() => insertMarkdown('**', '**')}
			disabled={isEditorDisabled}
			title={$t('issueDetail.toolbarBold') || 'Bold'}
		>
			<Bold class="h-4 w-4" />
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="h-8 w-8"
			onclick={() => insertMarkdown('*', '*')}
			disabled={isEditorDisabled}
			title={$t('issueDetail.toolbarItalic') || 'Italic'}
		>
			<Italic class="h-4 w-4" />
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="h-8 w-8"
			onclick={() => insertMarkdown('`', '`')}
			disabled={isEditorDisabled}
			title={$t('issueDetail.toolbarCode') || 'Code'}
		>
			<Code class="h-4 w-4" />
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="h-8 w-8"
			onclick={insertLink}
			disabled={isEditorDisabled}
			title={$t('issueDetail.toolbarLink') || 'Link'}
		>
			<Link class="h-4 w-4" />
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="h-8 w-8"
			onclick={insertListItem}
			disabled={isEditorDisabled}
			title={$t('issueDetail.toolbarList') || 'List'}
		>
			<List class="h-4 w-4" />
		</Button>

		<div class="flex-1"></div>

		<Button
			variant="ghost"
			size="sm"
			class="h-8 gap-1 text-xs"
			onclick={() => (showPreview = !showPreview)}
		>
			{#if showPreview}
				<Edit3 class="h-3 w-3" />
				{$t('issueDetail.edit') || 'Edit'}
			{:else}
				<Eye class="h-3 w-3" />
				{$t('issueDetail.preview') || 'Preview'}
			{/if}
		</Button>
	</div>

	<!-- Editor / Preview -->
	{#if showPreview}
		<div
			class="prose prose-sm dark:prose-invert bg-muted/30 min-h-[100px] max-w-none rounded-lg border p-3"
		>
			{#if value.trim()}
				{@html renderMarkdown(value)}
			{:else}
				<span class="text-muted-foreground italic">
					{$t('issueDetail.noContent') || 'Nothing to preview'}
				</span>
			{/if}
		</div>
	{:else}
		<textarea
			bind:this={textareaRef}
			bind:value
			{placeholder}
			disabled={disabled || submitting || uploading}
			onkeydown={handleKeydown}
			onpaste={handlePaste}
			class="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[100px] w-full resize-none rounded-lg border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
			rows={4}
		></textarea>
	{/if}

	<!-- Submit Row -->
	<div class="flex items-center justify-between">
		<span class="text-muted-foreground text-xs">
			{#if uploading}
				{$t('common.uploading') || 'Uploading image...'}
			{:else}
				{$t('issueDetail.shiftEnterToSubmit') || 'Shift+Enter to submit'}
			{/if}
		</span>
		<Button size="sm" onclick={handleSubmit} disabled={!canSubmit}>
			{#if submitting}
				<Loader2 class="mr-2 h-4 w-4 animate-spin" />
			{:else}
				<Send class="mr-2 h-4 w-4" />
			{/if}
			{$t('issueDetail.addComment') || 'Add comment'}
		</Button>
	</div>
</div>
