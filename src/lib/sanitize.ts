import DOMPurify from 'isomorphic-dompurify';

// Add a hook to enforce noopener noreferrer on target="_blank" links
// This prevents reverse tabnabbing attacks
DOMPurify.addHook('afterSanitizeAttributes', (node) => {
	if (node instanceof Element && node.tagName === 'A' && node.getAttribute('target') === '_blank') {
		node.setAttribute('rel', 'noopener noreferrer');
	}
});

/**
 * Sanitizes HTML content to prevent XSS attacks.
 * Allows safe HTML tags and attributes, and ensures external links are safe.
 */
export function sanitizeHtml(html: string | null | undefined): string {
	if (!html) return '';

	// DOMPurify sanitizes the HTML and returns a safe string
	// We allow 'target' attribute for links, but the hook above ensures they are safe
	return DOMPurify.sanitize(html, {
		ADD_ATTR: ['target']
	});
}
