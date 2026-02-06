import DOMPurify from 'dompurify';

const hook = (node: Element) => {
	if (node.tagName === 'A' && node.getAttribute('target') === '_blank') {
		node.setAttribute('rel', 'noopener noreferrer');
	}
};

export function sanitizeHtml(html: string): string {
	if (!html) return '';

	// Add hook to enforce rel="noopener noreferrer"
	DOMPurify.addHook('afterSanitizeAttributes', hook);

	try {
		// Sanitize with target attribute allowed
		// We use a try-finally block to ensure the hook is always removed
		const clean = DOMPurify.sanitize(html, {
			ADD_ATTR: ['target']
		});
		return clean;
	} finally {
		// Remove the hook to avoid side effects on other DOMPurify usages
		DOMPurify.removeHook('afterSanitizeAttributes');
	}
}
