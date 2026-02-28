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
    pub modified_date: Option<String>,
    pub priority: Option<i64>,
    pub severity: Option<i64>,
    pub issue_type: Option<i64>,
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
            modified_date: dto.modified_date,
            priority: dto.priority,
            severity: dto.severity,
            issue_type: dto.type_,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use taiga_client::models::{IssueDto, IssueStatusExtraInfo, UserExtraInfo};

    #[test]
    fn test_issue_conversion_from_dto() {
        let dto = IssueDto {
            id: 123,
            subject: "Test Issue".to_string(),
            project: 456,
            status: 1,
            status_extra_info: Some(IssueStatusExtraInfo {
                name: "Open".to_string(),
                color: "#ff0000".to_string(),
                is_closed: false,
            }),
            assigned_to: Some(789),
            assigned_to_extra_info: Some(UserExtraInfo {
                username: "jdoe".to_string(),
                full_name_display: "John Doe".to_string(),
                photo: Some("http://example.com/photo.jpg".to_string()),
            }),
            owner: Some(999),
            modified_date: Some("2023-01-02T12:00:00Z".to_string()),
            priority: Some(10),
            severity: Some(20),
            type_: Some(30),
        };

        let issue: Issue = dto.into();

        assert_eq!(issue.id, 123);
        assert_eq!(issue.subject, "Test Issue");
        assert_eq!(issue.status_name, Some("Open".to_string()));
        assert_eq!(issue.status_color, Some("#ff0000".to_string()));
        assert_eq!(issue.assigned_to, Some(789));
        assert_eq!(issue.assigned_to_name, Some("John Doe".to_string()));
        assert_eq!(
            issue.assigned_to_photo,
            Some("http://example.com/photo.jpg".to_string())
        );
        assert_eq!(
            issue.modified_date,
            Some("2023-01-02T12:00:00Z".to_string())
        );
        assert_eq!(issue.priority, Some(10));
        assert_eq!(issue.severity, Some(20));
        assert_eq!(issue.issue_type, Some(30));
    }

    #[test]
    fn test_issue_conversion_minimal_dto() {
        let dto = IssueDto {
            id: 123,
            subject: "Minimal".to_string(),
            project: 456,
            status: 1,
            status_extra_info: None,
            assigned_to: None,
            assigned_to_extra_info: None,
            owner: None,
            modified_date: None,
            priority: None,
            severity: None,
            type_: None,
        };

        let issue: Issue = dto.into();

        assert_eq!(issue.id, 123);
        assert_eq!(issue.subject, "Minimal");
        assert_eq!(issue.status_name, None);
        assert_eq!(issue.assigned_to, None);
        assert_eq!(issue.assigned_to_name, None);
        assert_eq!(issue.modified_date, None);
        assert_eq!(issue.priority, None);
        assert_eq!(issue.severity, None);
        assert_eq!(issue.issue_type, None);
    }
}
