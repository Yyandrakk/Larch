use serde::Serialize;
use taiga_client::models::IssueDto;

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    pub id: i64,
    pub subject: String,
    pub project: i64,
    pub status: i64,
    pub owner: Option<i64>,
    pub assigned_to: Option<i64>,
}

impl From<IssueDto> for Issue {
    fn from(dto: IssueDto) -> Self {
        Self {
            id: dto.id,
            subject: dto.subject,
            project: dto.project,
            status: dto.status,
            owner: dto.owner,
            assigned_to: dto.assigned_to,
        }
    }
}
