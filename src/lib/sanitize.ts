import DOMPurify from 'dompurify';

/**
 * Hook to enforce rel="noopener noreferrer" on all links with target="_blank".
 * This prevents reverse tabnabbing attacks where the opened page can access window.opener.
 */
const hook = (node: Element) => {
	if (node.tagName === 'A' && node.getAttribute('target') === '_blank') {
		node.setAttribute('rel', 'noopener noreferrer');
	}
};

// Add hook globally once to ensure consistent security across the app.
// This prevents the race condition and side effects of adding/removing hooks per call.
DOMPurify.addHook('afterSanitizeAttributes', hook);

export function sanitizeHtml(html: string): string {
	if (!html) return '';

	// Sanitize with target attribute allowed (but controlled by the hook above)
	return DOMPurify.sanitize(html, {
		ADD_ATTR: ['target']
	});
}
