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
}

export interface Project {
    id: number;
    name: string;
    slug: string;
    description: string;
    owner: number;
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
}

export interface FilterObject {
    project_ids?: number[];
    status_ids?: number[];
    assignee_ids?: number[];
    status_exclude?: boolean;
    assignee_exclude?: boolean;
}
