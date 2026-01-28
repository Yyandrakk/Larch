import DOMPurify from 'dompurify';

/**
 * Sanitizes HTML content to prevent XSS attacks.
 * Uses DOMPurify to strip dangerous tags and attributes.
 * This is crucial when rendering user-generated content with {@html}.
 */
export function sanitizeHtml(html: string): string {
	if (!html) return '';

	// DOMPurify cleans the HTML and prevents XSS
	// By default, it removes scripts, iframes, and dangerous attributes
	return DOMPurify.sanitize(html, {
		USE_PROFILES: { html: true }, // Restrict to HTML profile
		ADD_ATTR: ['target'] // Allow target attribute (e.g. for links)
	});
}
