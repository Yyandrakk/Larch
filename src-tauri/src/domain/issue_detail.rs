use serde::Serialize;
use taiga_client::models::{AttachmentDto, IssueDetailDto, IssueHistoryEntryDto, IssueNeighborDto};

// ============================================================================
// Issue Detail Domain Models
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct IssueDetail {
    pub id: i64,
    pub ref_number: i64,
    pub subject: String,
    pub description: Option<String>,
    pub description_html: Option<String>,

    // Project info
    pub project_id: i64,
    pub project_name: String,
    pub project_slug: String,

    // Status with is_closed distinction
    pub status_id: i64,
    pub status_name: String,
    pub status_color: String,
    pub is_closed: bool,

    // Type, Severity, Priority - resolved to names when possible
    pub type_id: Option<i64>,
    pub type_name: Option<String>,
    pub type_color: Option<String>,
    pub severity_id: Option<i64>,
    pub severity_name: Option<String>,
    pub severity_color: Option<String>,
    pub priority_id: Option<i64>,
    pub priority_name: Option<String>,
    pub priority_color: Option<String>,

    // People
    pub owner_id: Option<i64>,
    pub owner_name: Option<String>,
    pub owner_username: Option<String>,
    pub owner_photo: Option<String>,
    pub assigned_to_id: Option<i64>,
    pub assigned_to_name: Option<String>,
    pub assigned_to_username: Option<String>,
    pub assigned_to_photo: Option<String>,

    // Collections
    pub tags: Vec<Tag>,
    pub attachments: Vec<Attachment>,
    pub watchers: Vec<i64>,
    pub total_watchers: i64,

    // Blocking
    pub is_blocked: bool,
    pub blocked_note: Option<String>,

    // Dates
    pub due_date: Option<String>,
    pub due_date_status: Option<String>,
    pub created_date: String,
    pub modified_date: String,
    pub finished_date: Option<String>,

    // Navigation & versioning
    pub version: i64,
    pub next_issue: Option<IssueNeighbor>,
    pub previous_issue: Option<IssueNeighbor>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssueNeighbor {
    pub id: i64,
    pub ref_number: i64,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Attachment {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub size: i64,
    pub size_display: String,
    pub is_image: bool,
    pub created_date: String,
}

// ============================================================================
// History / Comment Domain Models
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct Comment {
    pub id: String,
    pub author_id: i64,
    pub author_name: String,
    pub author_username: String,
    pub author_photo: Option<String>,
    pub content: String,
    pub content_html: String,
    pub created_at: String,
    pub is_deleted: bool,
    pub is_edited: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct HistoryEntry {
    pub id: String,
    pub user_id: i64,
    pub user_name: String,
    pub user_username: String,
    pub user_photo: Option<String>,
    pub created_at: String,
    pub entry_type: String, // "comment" | "change"
    pub comment: Option<String>,
    pub comment_html: Option<String>,
    pub is_deleted: bool,
    pub is_edited: bool,
}

// ============================================================================
// Conversion Implementations
// ============================================================================

impl From<&IssueNeighborDto> for IssueNeighbor {
    fn from(dto: &IssueNeighborDto) -> Self {
        Self {
            id: dto.id,
            ref_number: dto.ref_,
            subject: dto.subject.clone(),
        }
    }
}

impl From<&AttachmentDto> for Attachment {
    fn from(dto: &AttachmentDto) -> Self {
        let is_image = is_image_file(&dto.name);
        Self {
            id: dto.id,
            name: dto.name.clone(),
            url: dto.url.clone(),
            thumbnail_url: dto.thumbnail_card_url.clone(),
            size: dto.size,
            size_display: format_file_size(dto.size),
            is_image,
            created_date: dto.created_date.clone(),
        }
    }
}

impl From<&IssueHistoryEntryDto> for HistoryEntry {
    fn from(dto: &IssueHistoryEntryDto) -> Self {
        let entry_type = if dto.entry_type == 1 {
            "change".to_string()
        } else {
            "comment".to_string()
        };

        let has_comment = !dto.comment.is_empty();
        let is_deleted = dto.delete_comment_date.is_some();
        let is_edited = dto.edit_comment_date.is_some();

        Self {
            id: dto.id.clone(),
            user_id: dto.user.pk,
            user_name: dto.user.name.clone(),
            user_username: dto.user.username.clone(),
            user_photo: dto.user.photo.clone(),
            created_at: dto.created_at.clone(),
            entry_type,
            comment: if has_comment {
                Some(dto.comment.clone())
            } else {
                None
            },
            comment_html: if has_comment {
                Some(dto.comment_html.clone())
            } else {
                None
            },
            is_deleted,
            is_edited,
        }
    }
}

impl IssueDetail {
    /// Create IssueDetail from DTO (without name resolution for type/severity/priority)
    pub fn from_dto(dto: IssueDetailDto) -> Self {
        let status_name = dto
            .status_extra_info
            .as_ref()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| format!("Status {}", dto.status));
        let status_color = dto
            .status_extra_info
            .as_ref()
            .map(|s| s.color.clone())
            .unwrap_or_else(|| "#808080".to_string());
        let is_closed = dto
            .status_extra_info
            .as_ref()
            .map(|s| s.is_closed)
            .unwrap_or(dto.is_closed);

        // Parse tags from JSON Value - handles null colors
        let tags: Vec<Tag> = dto
            .tags
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| {
                        let tag_arr = t.as_array()?;
                        let name = tag_arr.first()?.as_str()?.to_string();
                        let color = tag_arr
                            .get(1)
                            .and_then(|c| c.as_str())
                            .map(|s| s.to_string());
                        Some(Tag { name, color })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let attachments: Vec<Attachment> = dto.attachments.iter().map(|a| a.into()).collect();

        let next_issue = dto
            .neighbors
            .as_ref()
            .and_then(|n| n.next.as_ref())
            .map(|n| n.into());
        let previous_issue = dto
            .neighbors
            .as_ref()
            .and_then(|n| n.previous.as_ref())
            .map(|n| n.into());

        Self {
            id: dto.id,
            ref_number: dto.ref_,
            subject: dto.subject,
            description: dto.description,
            description_html: dto.description_html,
            project_id: dto.project,
            project_name: dto.project_extra_info.name,
            project_slug: dto.project_extra_info.slug,
            status_id: dto.status,
            status_name,
            status_color,
            is_closed,
            type_id: dto.type_,
            type_name: None, // Will be resolved by command
            type_color: None,
            severity_id: dto.severity,
            severity_name: None, // Will be resolved by command
            severity_color: None,
            priority_id: dto.priority,
            priority_name: None, // Will be resolved by command
            priority_color: None,
            owner_id: dto.owner,
            owner_name: dto
                .owner_extra_info
                .as_ref()
                .map(|o| o.full_name_display.clone()),
            owner_username: dto.owner_extra_info.as_ref().map(|o| o.username.clone()),
            owner_photo: dto.owner_extra_info.as_ref().and_then(|o| o.photo.clone()),
            assigned_to_id: dto.assigned_to,
            assigned_to_name: dto
                .assigned_to_extra_info
                .as_ref()
                .map(|a| a.full_name_display.clone()),
            assigned_to_username: dto
                .assigned_to_extra_info
                .as_ref()
                .map(|a| a.username.clone()),
            assigned_to_photo: dto
                .assigned_to_extra_info
                .as_ref()
                .and_then(|a| a.photo.clone()),
            tags,
            attachments,
            watchers: dto.watchers,
            total_watchers: dto.total_watchers,
            is_blocked: dto.is_blocked,
            blocked_note: dto.blocked_note,
            due_date: dto.due_date,
            due_date_status: dto.due_date_status,
            created_date: dto.created_date,
            modified_date: dto.modified_date,
            finished_date: dto.finished_date,
            version: dto.version,
            next_issue,
            previous_issue,
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Format file size in human-readable format
fn format_file_size(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Check if a filename is an image based on extension
fn is_image_file(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".gif")
        || lower.ends_with(".webp")
        || lower.ends_with(".svg")
        || lower.ends_with(".bmp")
}
