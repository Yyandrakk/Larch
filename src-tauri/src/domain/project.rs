use serde::Serialize;
use taiga_client::models::ProjectDto;

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
            owner: dto.owner,
        }
    }
}
