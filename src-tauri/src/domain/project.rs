use serde::Serialize;
use taiga_client::models::{IssueTypeDto, PriorityDto, ProjectDto, SeverityDto};

#[derive(Debug, Clone, Serialize)]
pub struct IssueStatus {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Member {
    pub id: i64,
    pub user_id: Option<i64>,
    pub full_name: String,
    pub role_name: String,
    pub photo: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Priority {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub order: i64,
}

impl From<&PriorityDto> for Priority {
    fn from(dto: &PriorityDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name.clone(),
            color: dto.color.clone(),
            order: dto.order,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Severity {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub order: i64,
}

impl From<&SeverityDto> for Severity {
    fn from(dto: &SeverityDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name.clone(),
            color: dto.color.clone(),
            order: dto.order,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IssueType {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub order: i64,
}

impl From<&IssueTypeDto> for IssueType {
    fn from(dto: &IssueTypeDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name.clone(),
            color: dto.color.clone(),
            order: dto.order,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TagColor {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectMetadata {
    pub id: i64,
    pub statuses: Vec<IssueStatus>,
    pub members: Vec<Member>,
    pub priorities: Vec<Priority>,
    pub severities: Vec<Severity>,
    pub issue_types: Vec<IssueType>,
    pub tags_colors: Vec<TagColor>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub owner: i64,
    pub created_date: Option<String>,
    pub modified_date: Option<String>,
}

impl From<ProjectDto> for Project {
    fn from(dto: ProjectDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            slug: dto.slug,
            description: dto.description,
            owner: dto.owner.id,
            created_date: dto.created_date,
            modified_date: dto.modified_date,
        }
    }
}

impl From<taiga_client::models::ProjectListEntryDto> for Project {
    fn from(dto: taiga_client::models::ProjectListEntryDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            slug: dto.slug,
            description: dto.description,
            owner: dto.owner.id,
            created_date: dto.created_date,
            modified_date: dto.modified_date,
        }
    }
}
