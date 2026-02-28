const PROJECT_COLOR_PALETTE = [
	{ bg: 'bg-emerald-500', tagBg: 'bg-emerald-500/10', tagText: 'text-emerald-400' },
	{ bg: 'bg-purple-500', tagBg: 'bg-purple-500/10', tagText: 'text-purple-400' },
	{ bg: 'bg-blue-500', tagBg: 'bg-blue-500/10', tagText: 'text-blue-400' },
	{ bg: 'bg-orange-500', tagBg: 'bg-orange-500/10', tagText: 'text-orange-400' },
	{ bg: 'bg-pink-500', tagBg: 'bg-pink-500/10', tagText: 'text-pink-400' },
	{ bg: 'bg-cyan-500', tagBg: 'bg-cyan-500/10', tagText: 'text-cyan-400' }
];

function getColorIndex(projectName: string): number {
	const hash = projectName.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
	return hash % PROJECT_COLOR_PALETTE.length;
}

export function getProjectColor(projectName: string): string {
	return PROJECT_COLOR_PALETTE[getColorIndex(projectName)].bg;
}

export function getProjectTagStyles(projectName: string): string {
	const palette = PROJECT_COLOR_PALETTE[getColorIndex(projectName)];
	return `${palette.tagBg} ${palette.tagText}`;
}
