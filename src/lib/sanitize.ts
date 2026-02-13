import DOMPurify from 'dompurify';

const hook = (node: Element) => {
	if (node.tagName === 'A' && node.getAttribute('target') === '_blank') {
		node.setAttribute('rel', 'noopener noreferrer');
	}
};

// Add hook globally once to enforce rel="noopener noreferrer"
// This prevents adding duplicate hooks on every call and avoids
// the side effect of wiping all hooks with removeHook().
DOMPurify.addHook('afterSanitizeAttributes', hook);

export function sanitizeHtml(html: string): string {
	if (!html) return '';

	// Sanitize with target attribute allowed.
	// The hook above ensures target="_blank" is safe.
	return DOMPurify.sanitize(html, {
		ADD_ATTR: ['target']
	});
}
