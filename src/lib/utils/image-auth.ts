import { getApiUrl } from '$lib/stores/config.svelte';

export function transformImageUrls(html: string): string {
	const apiUrl = getApiUrl();
	if (!apiUrl) return html;

	try {
		const apiObj = new URL(apiUrl);
		const apiHost = apiObj.host;

		return html.replace(/src="(https?:\/\/[^"]+)"/g, (match, url) => {
			try {
				const urlObj = new URL(url);
				if (urlObj.host === apiHost || urlObj.host.endsWith('.' + apiHost)) {
					return `src="${url.replace(/^https?:\/\//, 'taiga-auth://')}"`;
				}
			} catch {
				// Invalid URL
			}
			return match;
		});
	} catch {
		return html;
	}
}
