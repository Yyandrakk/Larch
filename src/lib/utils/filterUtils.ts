import type { FilterObject } from '$lib/types';

export function normalizeFilter(filter: FilterObject): FilterObject {
	const normalized: FilterObject = {};

	const processArray = (arr: number[] | undefined) => {
		if (!arr || arr.length === 0) return undefined;
		return [...arr].sort((a, b) => a - b);
	};

	const processBoolean = (val: boolean | undefined) => {
		return val === true ? true : undefined;
	};

	const project_ids = processArray(filter.project_ids);
	if (project_ids) normalized.project_ids = project_ids;

	const project_exclude = processBoolean(filter.project_exclude);
	if (project_exclude) normalized.project_exclude = project_exclude;

	const status_ids = processArray(filter.status_ids);
	if (status_ids) normalized.status_ids = status_ids;

	const status_exclude = processBoolean(filter.status_exclude);
	if (status_exclude) normalized.status_exclude = status_exclude;

	const assignee_ids = processArray(filter.assignee_ids);
	if (assignee_ids) normalized.assignee_ids = assignee_ids;

	const assignee_exclude = processBoolean(filter.assignee_exclude);
	if (assignee_exclude) normalized.assignee_exclude = assignee_exclude;

	const priority_ids = processArray(filter.priority_ids);
	if (priority_ids) normalized.priority_ids = priority_ids;

	const priority_exclude = processBoolean(filter.priority_exclude);
	if (priority_exclude) normalized.priority_exclude = priority_exclude;

	const severity_ids = processArray(filter.severity_ids);
	if (severity_ids) normalized.severity_ids = severity_ids;

	const severity_exclude = processBoolean(filter.severity_exclude);
	if (severity_exclude) normalized.severity_exclude = severity_exclude;

	const type_ids = processArray(filter.type_ids);
	if (type_ids) normalized.type_ids = type_ids;

	const type_exclude = processBoolean(filter.type_exclude);
	if (type_exclude) normalized.type_exclude = type_exclude;

	return normalized;
}

export function deepEqual(a: FilterObject, b: FilterObject): boolean {
	const normA = normalizeFilter(a);
	const normB = normalizeFilter(b);

	return JSON.stringify(normA) === JSON.stringify(normB);
}
