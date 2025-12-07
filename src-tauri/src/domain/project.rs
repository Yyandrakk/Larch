use serde::Serialize;
use taiga_client::models::ProjectDto;

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
pub struct ProjectMetadata {
    pub id: i64,
    pub statuses: Vec<IssueStatus>,
    pub members: Vec<Member>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub owner: i64,
}

impl From<ProjectDto> for Project {
    fn from(dto: ProjectDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            slug: dto.slug,
            description: dto.description,
            owner: dto.owner.id,
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
        }
    }
}
