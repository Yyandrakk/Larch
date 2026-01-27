import DOMPurify from 'dompurify';

// Initialize hooks once to avoid duplicate listeners
DOMPurify.addHook('afterSanitizeAttributes', (node) => {
	if (node.tagName === 'A' && node.getAttribute('target') === '_blank') {
		node.setAttribute('rel', 'noopener noreferrer');
	}
});

/**
 * Sanitizes HTML content to prevent XSS attacks.
 * Uses DOMPurify to strip dangerous tags (script, iframe, object, etc.) and attributes.
 * Also enforces rel="noopener noreferrer" on links with target="_blank".
 *
 * @param html The potentially unsafe HTML string.
 * @returns The sanitized HTML string.
 */
export function sanitizeHtml(html: string): string {
	if (!html) return '';

	return DOMPurify.sanitize(html, {
		USE_PROFILES: { html: true }, // Allow standard HTML tags
		ADD_ATTR: ['target'] // Allow target attribute on links
	});
}
