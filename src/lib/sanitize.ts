import DOMPurify from 'dompurify';

/**
 * Hook to enforce rel="noopener noreferrer" on all links with target="_blank".
 * This prevents reverse tabnabbing attacks where the opened page can access window.opener.
 */
const hook = (node: Element) => {
	if (node.tagName === 'A' && node.hasAttribute('target')) {
		// Enforce rel="noopener noreferrer" for ALL targets (_blank, custom windows, etc.)
		// since any target that opens a new browsing context exposes window.opener
		// by default in some browsers, allowing reverse tabnabbing.
		const target = node.getAttribute('target') || '';
		const targetLower = target.toLowerCase();
		if (targetLower !== '_self' && targetLower !== '_parent' && targetLower !== '_top' && target !== '') {
			// Use setAttribute and append to existing rel if present to avoid overwriting (e.g. nofollow)
			const existingRel = node.getAttribute('rel') || '';
			const rels = new Set(existingRel.split(/\s+/).filter(Boolean));
			rels.add('noopener');
			rels.add('noreferrer');
			node.setAttribute('rel', Array.from(rels).join(' '));
		}
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
