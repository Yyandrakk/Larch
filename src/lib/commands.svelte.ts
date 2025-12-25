// This file centralizes the names of Tauri commands
// to prevent typos and improve maintainability.

export const CMD_LOGIN = 'login';
export const CMD_HAS_API_TOKEN = 'has_api_token';
export const CMD_LOGOUT = 'logout';
export const CMD_GET_ME = 'get_me';
export const CMD_GET_PROJECTS = 'get_projects';
export const CMD_LIST_ISSUES = 'list_issues';
export const CMD_GET_SELECTED_PROJECTS = 'get_selected_projects';
export const CMD_SAVE_SELECTED_PROJECTS = 'save_selected_projects';
export const CMD_GET_AGGREGATED_ISSUES = 'get_aggregated_issues';
export const CMD_GET_PROJECT_METADATA = 'get_project_metadata';

// Issue Detail Commands
export const CMD_GET_ISSUE_DETAIL = 'get_issue_detail';
export const CMD_GET_ISSUE_HISTORY = 'get_issue_history';
export const CMD_CHANGE_ISSUE_STATUS = 'change_issue_status';
export const CMD_ADD_ISSUE_COMMENT = 'add_issue_comment';
export const CMD_CHANGE_ISSUE_ASSIGNEE = 'change_issue_assignee';
export const CMD_COMMIT_ISSUE_DESCRIPTION = 'commit_issue_description';

// Draft Commands
export const CMD_SAVE_LOCAL_DRAFT = 'save_local_draft';
export const CMD_GET_LOCAL_DRAFT = 'get_local_draft';
export const CMD_DELETE_LOCAL_DRAFT = 'delete_local_draft';

// App Commands
export const CMD_FORCE_CLOSE_APP = 'force_close_app';
