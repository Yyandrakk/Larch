# ADR-0004: Issue Detail Sidebar Redesign (M3)

## Context

The original Issue Detail Sheet displayed metadata in a vertical list within the main content area. As we added more actionable fields (priority, severity, type, labels, attachments), the UI became cramped and the user experience degraded. Users needed a cleaner way to view and edit issue properties without losing focus on the main content (description and comments).

## Decision

Redesign the Issue Detail Sheet into a **two-column layout**:

1. **Left Panel (70%)**: Description, comments, and primary content
2. **Right Panel (30%)**: Metadata sidebar with all actionable fields

### New Components Created

| Component                     | Purpose                                                        |
| ----------------------------- | -------------------------------------------------------------- |
| `StatusChip.svelte`           | Shows save status (Saved/Saving/Collision) with refresh action |
| `ActivityLog.svelte`          | Displays history entries with "Show more" pagination           |
| `PrioritySelector.svelte`     | Dropdown for priority selection                                |
| `SeveritySelector.svelte`     | Dropdown for severity selection                                |
| `TypeSelector.svelte`         | Dropdown for issue type selection                              |
| `LabelManager.svelte`         | Tag management with on-the-fly creation                        |
| `AttachmentManager.svelte`    | File upload, download, and delete                              |
| `IssueMetadataSidebar.svelte` | Container orchestrating all sidebar components                 |

### Backend Commands Added

- `change_issue_priority` - Update issue priority
- `change_issue_severity` - Update issue severity
- `change_issue_type` - Update issue type
- `update_issue_tags` - Update issue tags/labels
- `upload_issue_attachment` - Upload file attachment (returns `Attachment`)
- `delete_issue_attachment` - Delete attachment by ID

### API Response Handling

Taiga API returns `tags_colors` in the project response as an array of tuples `[[name, color|null], ...]` rather than objects. We parse this with `serde_json::Value` and convert manually to maintain consistency with how issue tags are handled.

## Consequences

### Positive

- Cleaner separation of content vs metadata
- All issue properties editable inline without modal dialogs
- Consistent UX pattern across priority/severity/type selectors
- Real-time save status feedback via StatusChip
- Labels support on-the-fly creation with project color palette

### Negative

- More complex component hierarchy
- Sidebar requires minimum viewport width (~1024px) for comfortable use
- Additional API calls for project metadata on each issue load

## Status

Accepted

## Related

- M2: Offline Drafts (description draft system reused)
- M4: Future - Bulk editing may leverage these selectors
