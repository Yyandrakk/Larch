import { invoke } from '@tauri-apps/api/core';
import { CMD_REFRESH_TOKEN } from '$lib/commands.svelte';

let isRefreshing = false;
let refreshPromise: Promise<void> | null = null;
let sessionExpiredCallback: (() => void) | null = null;

export function setSessionExpiredHandler(handler: () => void): void {
	sessionExpiredCallback = handler;
}

function isUnauthorizedError(error: unknown): boolean {
	const errorStr = String(error).toLowerCase();
	return errorStr.includes('unauthorized') || errorStr.includes('401');
}

async function handleTokenRefresh(): Promise<void> {
	if (isRefreshing && refreshPromise) {
		return refreshPromise;
	}

	isRefreshing = true;
	refreshPromise = invoke<void>(CMD_REFRESH_TOKEN);

	try {
		await refreshPromise;
	} finally {
		isRefreshing = false;
		refreshPromise = null;
	}
}

function handleSessionExpired(): void {
	if (sessionExpiredCallback) {
		sessionExpiredCallback();
	}
}

export async function apiCall<T>(command: string, args?: Record<string, unknown>): Promise<T> {
	try {
		return await invoke<T>(command, args);
	} catch (error: unknown) {
		if (isUnauthorizedError(error)) {
			try {
				await handleTokenRefresh();
				return await invoke<T>(command, args);
			} catch (refreshError) {
				handleSessionExpired();
				throw refreshError;
			}
		}
		throw error;
	}
}
