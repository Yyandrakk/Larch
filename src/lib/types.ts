export interface Issue {
	id: number;
	subject: string;
	project: number;
	status: number;
	status_name?: string;
	status_color?: string;
	owner?: number;
	assigned_to?: number;
	assigned_to_name?: string;
	assigned_to_photo?: string;
	modified_date?: string;
}

export interface User {
	id: number;
	username: string;
	full_name: string;
	email: string;
	photo: string | null;
	big_photo: string | null;
	gravatar_id: string;
	is_active: boolean;
}

export interface Project {
	id: number;
	name: string;
	slug: string;
	description: string;
	owner: number;
	created_date: string | null;
	modified_date: string | null;
}

export interface IssueStatus {
	id: number;
	name: string;
	color: string;
	is_closed: boolean;
}

export interface Member {
	id: number;
	user_id?: number;
	full_name: string;
	role_name: string;
	photo?: string;
}

export interface ProjectMetadata {
	id: number;
	statuses: IssueStatus[];
	members: Member[];
	priorities: Priority[];
	severities: Severity[];
	issue_types: IssueType[];
	tags_colors: TagColor[];
}

export interface Priority {
	id: number;
	name: string;
	color: string;
	order: number;
}

export interface Severity {
	id: number;
	name: string;
	color: string;
	order: number;
}

export interface IssueType {
	id: number;
	name: string;
	color: string;
	order: number;
}

export interface TagColor {
	name: string;
	color?: string;
}

export interface FilterObject {
	project_ids?: number[];
	status_ids?: number[];
	assignee_ids?: number[];
	project_exclude?: boolean;
	status_exclude?: boolean;
	assignee_exclude?: boolean;
}

export interface SavedView {
	id: string;
	name: string;
	filter_data: string;
	is_system: boolean;
	is_default: boolean;
	last_used: string | null;
	created_at: string;
}

export interface SavedViewInput {
	name: string;
	filter_data: string;
	is_system: boolean;
	is_default: boolean;
}

// ============================================================================
// Issue Detail Types
// ============================================================================

export interface IssueNeighbor {
	id: number;
	ref_number: number;
	subject: string;
}

export interface Tag {
	name: string;
	color?: string;
}

export interface Attachment {
	id: number;
	name: string;
	url: string;
	thumbnail_url?: string;
	size: number;
	size_display: string;
	is_image: boolean;
	created_date: string;
}

export interface IssueDetail {
	id: number;
	ref_number: number;
	subject: string;
	description?: string;
	description_html?: string;

	// Project info
	project_id: number;
	project_name: string;
	project_slug: string;

	// Status with is_closed distinction
	status_id: number;
	status_name: string;
	status_color: string;
	is_closed: boolean;

	// Type, Severity, Priority
	type_id?: number;
	type_name?: string;
	type_color?: string;
	severity_id?: number;
	severity_name?: string;
	severity_color?: string;
	priority_id?: number;
	priority_name?: string;
	priority_color?: string;

	// People
	owner_id?: number;
	owner_name?: string;
	owner_username?: string;
	owner_photo?: string;
	assigned_to_id?: number;
	assigned_to_name?: string;
	assigned_to_username?: string;
	assigned_to_photo?: string;

	// Collections
	tags: Tag[];
	attachments: Attachment[];
	watchers: number[];
	total_watchers: number;

	// Blocking
	is_blocked: boolean;
	blocked_note?: string;

	// Dates
	due_date?: string;
	due_date_status?: string;
	created_date: string;
	modified_date: string;
	finished_date?: string;

	// Navigation & versioning
	version: number;
	next_issue?: IssueNeighbor;
	previous_issue?: IssueNeighbor;
}

export interface FieldChange {
	field: string;
	old_value?: string;
	new_value?: string;
}

export interface HistoryEntry {
	id: string;
	user_id: number;
	user_name: string;
	user_username: string;
	user_photo?: string;
	created_at: string;
	entry_type: 'comment' | 'change';
	comment?: string;
	comment_html?: string;
	is_deleted: boolean;
	is_edited: boolean;
	changes: FieldChange[];
}
