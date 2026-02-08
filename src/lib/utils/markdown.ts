import { transformImageUrls } from '$lib/utils/image-auth';

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

export function renderMarkdown(text: string): string {
	if (!text) return '';

	// IMPORTANT: Escape HTML FIRST to prevent XSS
	const result = escapeHtml(text);

	// Apply markdown transformations on escaped text
	const html = result
		// Images (must come before links)
		.replace(/!\[(.*?)\]\((.+?)\)/g, (_, alt, url) => {
			const safeUrl = sanitizeUrl(url);
			return `<img src="${safeUrl}" alt="${alt}" class="max-w-full rounded border my-2" />`;
		})
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
		.replace(/\n/g, '<br>');

	return transformImageUrls(html);
}
