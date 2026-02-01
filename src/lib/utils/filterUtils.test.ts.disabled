import { describe, it, expect } from 'bun:test';
import { deepEqual, normalizeFilter } from './filterUtils';
import type { FilterObject } from '$lib/types';

describe('filterUtils', () => {
	describe('normalizeFilter', () => {
		it('should sort ID arrays', () => {
			const filter: FilterObject = {
				project_ids: [3, 1, 2],
				status_ids: [10, 5],
				assignee_ids: [100, 50]
			};
			const normalized = normalizeFilter(filter);
			expect(normalized.project_ids).toEqual([1, 2, 3]);
			expect(normalized.status_ids).toEqual([5, 10]);
			expect(normalized.assignee_ids).toEqual([50, 100]);
		});

		it('should treat empty arrays as undefined', () => {
			const filter: FilterObject = {
				project_ids: [],
				status_ids: [1]
			};
			const normalized = normalizeFilter(filter);
			expect(normalized.project_ids).toBeUndefined();
			expect(normalized.status_ids).toEqual([1]);
		});

		it('should treat false exclude flags as undefined', () => {
			const filter: FilterObject = {
				project_exclude: false,
				status_exclude: true
			};
			const normalized = normalizeFilter(filter);
			expect(normalized.project_exclude).toBeUndefined();
			expect(normalized.status_exclude).toBe(true);
		});

		it('should return a new object with deterministic key order', () => {
			const filter1: FilterObject = {
				status_ids: [1],
				project_ids: [2]
			};
			const filter2: FilterObject = {
				project_ids: [2],
				status_ids: [1]
			};
			const norm1 = normalizeFilter(filter1);
			const norm2 = normalizeFilter(filter2);

			expect(JSON.stringify(norm1)).toBe(JSON.stringify(norm2));
		});
	});

	describe('deepEqual', () => {
		it('should consider empty array and undefined as equal', () => {
			const a: FilterObject = { project_ids: [] };
			const b: FilterObject = { project_ids: undefined };
			expect(deepEqual(a, b)).toBe(true);
		});

		it('should consider different array order as equal', () => {
			const a: FilterObject = { project_ids: [1, 2] };
			const b: FilterObject = { project_ids: [2, 1] };
			expect(deepEqual(a, b)).toBe(true);
		});

		it('should consider false exclude and undefined as equal', () => {
			const a: FilterObject = { project_exclude: false };
			const b: FilterObject = { project_exclude: undefined };
			expect(deepEqual(a, b)).toBe(true);
		});

		it('should consider different values as not equal', () => {
			const a: FilterObject = { project_ids: [1] };
			const b: FilterObject = { project_ids: [2] };
			expect(deepEqual(a, b)).toBe(false);
		});

		it('should consider different exclude flags as not equal', () => {
			const a: FilterObject = { project_exclude: true };
			const b: FilterObject = { project_exclude: false };
			expect(deepEqual(a, b)).toBe(false);
		});
	});
});
