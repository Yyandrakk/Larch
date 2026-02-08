use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LoginRequest<'a> {
    #[serde(rename = "type")]
    pub r#type: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthDetail {
    pub id: i64,
    pub username: String,
    pub auth_token: String,
    pub refresh: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RefreshRequest<'a> {
    pub refresh: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RefreshResponse {
    pub auth_token: String,
    pub refresh: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Me {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub photo: Option<String>,
    pub big_photo: Option<String>,
    pub gravatar_id: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserShort {
    pub id: i64,
    pub username: String,
    pub full_name_display: String,
    pub photo: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueStatusDto {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MemberDto {
    pub id: i64, // User ID
    pub full_name: String,
    pub role_name: String,
    pub photo: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagColorDto {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectDto {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub owner: UserShort,
    pub created_date: Option<String>,
    pub modified_date: Option<String>,
    #[serde(default)]
    pub issue_statuses: Option<Vec<IssueStatusDto>>,
    #[serde(default)]
    pub members: Option<Vec<MemberDto>>,
    #[serde(default)]
    pub priorities: Option<Vec<PriorityDto>>,
    #[serde(default)]
    pub severities: Option<Vec<SeverityDto>>,
    #[serde(default)]
    pub issue_types: Option<Vec<IssueTypeDto>>,
    #[serde(default)]
    pub tags_colors: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectListEntryDto {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub owner: UserShort,
    pub members: Option<Vec<i64>>,
    pub created_date: Option<String>,
    pub modified_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueStatusExtraInfo {
    pub name: String,
    pub color: String,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserExtraInfo {
    pub username: String,
    pub full_name_display: String,
    pub photo: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueDto {
    pub id: i64,
    pub subject: String,
    pub project: i64,
    pub status: i64,
    pub status_extra_info: Option<IssueStatusExtraInfo>,
    pub owner: Option<i64>,
    pub assigned_to: Option<i64>,
    pub assigned_to_extra_info: Option<UserExtraInfo>,
    pub modified_date: Option<String>,
}

// ============================================================================
// Issue Detail DTOs (GET /api/v1/issues/{id})
// ============================================================================

/// Project info embedded in issue detail response
#[derive(Debug, Clone, Deserialize)]
pub struct ProjectExtraInfo {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub logo_small_url: Option<String>,
}

/// Attachment detail object
#[derive(Debug, Clone, Deserialize)]
pub struct AttachmentDto {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub attached_file: String,
    pub thumbnail_card_url: Option<String>,
    pub preview_url: Option<String>,
    pub size: i64,
    pub created_date: String,
    pub is_deprecated: bool,
    pub description: Option<String>,
    pub order: i64,
}

/// Navigation to adjacent issues
#[derive(Debug, Clone, Deserialize)]
pub struct IssueNeighbors {
    pub next: Option<IssueNeighborDto>,
    pub previous: Option<IssueNeighborDto>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueNeighborDto {
    pub id: i64,
    #[serde(rename = "ref")]
    pub ref_: i64,
    pub subject: String,
}

/// Full issue detail from GET /api/v1/issues/{id}
#[derive(Debug, Clone, Deserialize)]
pub struct IssueDetailDto {
    pub id: i64,
    #[serde(rename = "ref")]
    pub ref_: i64,
    pub subject: String,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub project: i64,
    pub project_extra_info: ProjectExtraInfo,
    pub status: i64,
    pub status_extra_info: Option<IssueStatusExtraInfo>,
    #[serde(rename = "type")]
    pub type_: Option<i64>,
    pub priority: Option<i64>,
    pub severity: Option<i64>,
    pub owner: Option<i64>,
    pub owner_extra_info: Option<UserExtraInfo>,
    pub assigned_to: Option<i64>,
    pub assigned_to_extra_info: Option<UserExtraInfo>,
    #[serde(default)]
    pub tags: serde_json::Value, // [[name, color|null], ...] - color can be null
    #[serde(default)]
    pub attachments: Vec<AttachmentDto>,
    #[serde(default)]
    pub watchers: Vec<i64>,
    #[serde(default)]
    pub total_watchers: i64,
    #[serde(default)]
    pub is_closed: bool,
    #[serde(default)]
    pub is_blocked: bool,
    pub blocked_note: Option<String>,
    pub due_date: Option<String>,
    pub due_date_status: Option<String>,
    pub created_date: String,
    pub modified_date: String,
    pub finished_date: Option<String>,
    pub version: i64,
    pub neighbors: Option<IssueNeighbors>,
}

// ============================================================================
// Issue History DTOs (GET /api/v1/history/issue/{id})
// ============================================================================

/// User info in history entries
#[derive(Debug, Clone, Deserialize)]
pub struct HistoryUserDto {
    pub pk: i64,
    pub username: String,
    pub name: String,
    pub photo: Option<String>,
    pub gravatar_id: Option<String>,
    pub is_active: bool,
}

/// User info for deleted comments (minimal info)
#[derive(Debug, Clone, Deserialize)]
pub struct HistoryDeleteUserDto {
    pub pk: i64,
    pub name: String,
}

/// History entry from GET /api/v1/history/issue/{id}
#[derive(Debug, Clone, Deserialize)]
pub struct IssueHistoryEntryDto {
    pub id: String,
    pub user: HistoryUserDto,
    pub created_at: String,
    pub comment: Option<String>,
    pub comment_html: Option<String>,
    pub delete_comment_date: Option<String>,
    pub delete_comment_user: Option<HistoryDeleteUserDto>,
    pub edit_comment_date: Option<String>,
    #[serde(rename = "type")]
    pub entry_type: i32, // 1 = change, 2 = comment
    pub diff: Option<serde_json::Value>,
    pub values_diff: Option<serde_json::Value>,
    pub is_hidden: Option<bool>,
}

// ============================================================================
// Project Metadata DTOs (Type, Priority, Severity)
// ============================================================================

/// Issue type detail (from project)
#[derive(Debug, Clone, Deserialize)]
pub struct IssueTypeDto {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub order: i64,
    #[serde(default)]
    pub project: Option<i64>,
}

/// Priority detail (from project)
#[derive(Debug, Clone, Deserialize)]
pub struct PriorityDto {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub order: i64,
    #[serde(default)]
    pub project: Option<i64>,
}

/// Severity detail (from project)
#[derive(Debug, Clone, Deserialize)]
pub struct SeverityDto {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub order: i64,
    #[serde(default)]
    pub project: Option<i64>,
}

/// Membership detail (GET /api/v1/memberships?project={id})
#[derive(Debug, Clone, Deserialize)]
pub struct MembershipDto {
    pub id: i64,
    pub user: Option<i64>,
    pub full_name: Option<String>,
    pub role_name: String,
    pub photo: Option<String>,
    pub is_admin: bool,
}

// ============================================================================
// Issue Patch Request DTOs (PATCH /api/v1/issues/{id})
// ============================================================================

/// Request body for patching an issue (e.g., status change, add comment, edit description)
#[derive(Debug, Clone, Serialize)]
pub struct PatchIssueRequest {
    /// The version field for optimistic locking (required by Taiga)
    pub version: i64,
    /// The new status ID (optional - only include if changing status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// A comment to add to the issue (optional - only include if adding a comment)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The new description (optional - only include if editing description)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The new subject (optional - only include if editing subject)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The new assignee user ID (optional).
    /// - `None`: Field omitted (assignee remains unchanged).
    /// - `Some(None)`: Serialized as `null` (unassigns the issue).
    /// - `Some(Some(id))`: Serialized as `id` (assigns to user).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<Option<i64>>,
    /// The new priority ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// The new severity ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i64>,
    /// The new issue type ID (optional)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    /// Tags as [[name, color|null], ...] (optional)
    /// When set, replaces all existing tags with the provided list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
