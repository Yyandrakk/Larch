let apiUrl = $state<string>('');

export function setApiUrl(url: string) {
	apiUrl = url;
}

export function getApiUrl() {
	return apiUrl;
}
