use serde::Serialize;
use taiga_client::models::IssueDto;

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    pub id: i64,
    pub subject: String,
    pub project: i64,
    pub status: i64,
    pub status_name: Option<String>,
    pub status_color: Option<String>,
    pub owner: Option<i64>,
    pub assigned_to: Option<i64>,
    pub assigned_to_name: Option<String>,
    pub assigned_to_photo: Option<String>,
}

impl From<IssueDto> for Issue {
    fn from(dto: IssueDto) -> Self {
        Self {
            id: dto.id,
            subject: dto.subject,
            project: dto.project,
            status: dto.status,
            status_name: dto.status_extra_info.as_ref().map(|s| s.name.clone()),
            status_color: dto.status_extra_info.as_ref().map(|s| s.color.clone()),
            owner: dto.owner,
            assigned_to: dto.assigned_to,
            assigned_to_name: dto
                .assigned_to_extra_info
                .as_ref()
                .map(|u| u.full_name_display.clone()),
            assigned_to_photo: dto
                .assigned_to_extra_info
                .as_ref()
                .and_then(|u| u.photo.clone()),
        }
    }
}
