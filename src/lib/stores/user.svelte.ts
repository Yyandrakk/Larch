import type { User } from '$lib/types';

let currentUser: User | null = $state(null);

export function getCurrentUser(): User | null {
	return currentUser;
}

export function setCurrentUser(user: User | null): void {
	currentUser = user;
}

export function clearCurrentUser(): void {
	currentUser = null;
}

export function getUserDisplayName(): string {
	if (!currentUser) return 'User';
	return currentUser.full_name || currentUser.username || 'User';
}

export function getUserInitials(): string {
	const name = getUserDisplayName();
	const initials = name
		.split(' ')
		.filter((n) => n.length > 0)
		.map((n) => n[0])
		.join('')
		.toUpperCase()
		.slice(0, 2);

	if (initials.length > 0) return initials;
	const trimmed = name.trim();
	return trimmed.length > 0 ? trimmed.slice(0, 2).toUpperCase() : '?';
}

export function getUserPhoto(): string | null {
	return currentUser?.photo || currentUser?.big_photo || null;
}
