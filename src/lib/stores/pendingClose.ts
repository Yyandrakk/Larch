/**
 * Global state for tracking if there are pending changes that need saving before app close.
 * This module provides a simple singleton pattern with getters/setters for cross-component coordination.
 *
 * NOTE: Currently only supports ONE global pending commit callback at a time.
 * If multiple components need to prevent close, they would need a more complex registry.
 */

// The pending draft commit function
let pendingCommitFn: (() => Promise<boolean>) | null = null;

export function setPendingCommit(fn: (() => Promise<boolean>) | null) {
	pendingCommitFn = fn;
}

export function hasPendingCommit(): boolean {
	return pendingCommitFn !== null;
}

/**
 * Executes the registered pending commit callback.
 * If successful, the callback is cleared.
 *
 * @returns true if commit succeeded (or no pending), false if failed (conflict)
 */
export async function tryCommitPending(): Promise<boolean> {
	if (!pendingCommitFn) {
		return true;
	}
	const success = await pendingCommitFn();
	if (success) {
		pendingCommitFn = null;
	}
	return success;
}
