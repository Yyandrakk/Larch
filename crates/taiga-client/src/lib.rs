use reqwest::{multipart, StatusCode};
use secrecy::{ExposeSecret, Secret};
use url::Url;

pub mod errors;
pub mod models;
pub mod prelude;

use errors::TaigaClientError;
use models::{
    AuthDetail, IssueDetailDto, IssueDto, IssueHistoryEntryDto, LoginRequest, Me, ProjectDto,
    ProjectListEntryDto,
};

const API_V1_PREFIX: &str = "api/v1/";

#[derive(Debug, Clone)]
pub struct TaigaClient {
    client: reqwest::Client,
    api_base_url: Url,
}

impl TaigaClient {
    pub fn new(api_base_url: Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_base_url,
        }
    }

    fn build_url(&self, path: &str) -> Result<Url, TaigaClientError> {
        let full_path = format!("{}{}", API_V1_PREFIX, path);
        self.api_base_url.join(&full_path).map_err(Into::into)
    }

    fn is_version_conflict(body: &str) -> bool {
        let body_lower = body.to_lowercase();
        body_lower.contains("version")
            && (body_lower.contains("doesn't match") || body_lower.contains("does not match"))
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthDetail, TaigaClientError> {
        let url = self.build_url("auth")?;
        log::info!("Sending login request to {}", url);

        let request_body = LoginRequest {
            r#type: "normal",
            username,
            password,
        };

        let response = self.client.post(url).json(&request_body).send().await?;
        log::info!("Login response status: {}", response.status());

        if response.status().is_success() {
            let auth_detail = response.json::<AuthDetail>().await?;
            Ok(auth_detail)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Login failed. Status: {}, Body: {}", status, body);
            let err = match status {
                StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TaigaClientError::Unauthorized(status)
                }
                _ => TaigaClientError::AuthFailed(status),
            };
            Err(err)
        }
    }

    pub async fn get_me(&self, token: &Secret<String>) -> Result<Me, TaigaClientError> {
        let url = self.build_url("users/me")?;
        log::info!("Fetching current user from {}", url);

        let response = self
            .client
            .get(url)
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        log::info!("Get Me response status: {}", response.status());

        if response.status().is_success() {
            let me = response.json::<Me>().await?;
            Ok(me)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Get Me failed. Status: {}, Body: {}", status, body);
            let err = match status {
                StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TaigaClientError::Unauthorized(status)
                }
                _ => TaigaClientError::AuthFailed(status),
            };
            Err(err)
        }
    }

    pub async fn get_project(
        &self,
        token: &Secret<String>,
        project_id: i64,
    ) -> Result<ProjectDto, TaigaClientError> {
        let url = self.build_url(&format!("projects/{}", project_id))?;
        log::info!("Fetching project {} from {}", project_id, url);

        let response = self
            .client
            .get(url)
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        log::info!("Get Project response status: {}", response.status());

        if response.status().is_success() {
            let body = response.text().await?;
            match serde_json::from_str::<ProjectDto>(&body) {
                Ok(project) => Ok(project),
                Err(e) => {
                    log::error!(
                        "Failed to parse ProjectDto: {}. Body (first 1000 chars): {}",
                        e,
                        &body[..body.len().min(1000)]
                    );
                    Err(TaigaClientError::Serde(e))
                }
            }
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Get Project failed. Status: {}, Body: {}", status, body);
            let err = match status {
                StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TaigaClientError::Unauthorized(status)
                }
                _ => TaigaClientError::AuthFailed(status),
            };
            Err(err)
        }
    }

    pub async fn get_projects(
        &self,
        token: &Secret<String>,
        member_id: Option<i64>,
    ) -> Result<Vec<ProjectListEntryDto>, TaigaClientError> {
        let url = self.build_url("projects")?;
        log::info!(
            "Fetching projects from {} (member_id: {:?})",
            url,
            member_id
        );

        let mut request = self.client.get(url).bearer_auth(token.expose_secret());

        if let Some(id) = member_id {
            request = request.query(&[("member", id)]);
        }

        let response = request.send().await?;
        log::info!("Get Projects response status: {}", response.status());

        if response.status().is_success() {
            let projects = response.json::<Vec<ProjectListEntryDto>>().await?;
            log::info!("Found {} projects", projects.len());
            Ok(projects)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Get Projects failed. Status: {}, Body: {}", status, body);
            let err = match status {
                StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TaigaClientError::Unauthorized(status)
                }
                _ => TaigaClientError::AuthFailed(status),
            };
            Err(err)
        }
    }

    pub async fn list_issues(
        &self,
        token: &Secret<String>,
        project_id: i64,
        filters: Option<Vec<(String, String)>>,
    ) -> Result<Vec<IssueDto>, TaigaClientError> {
        let url = self.build_url("issues")?;
        log::info!("Fetching issues from {} for project {}", url, project_id);

        let mut request = self
            .client
            .get(url)
            .query(&[("project", project_id.to_string())])
            .bearer_auth(token.expose_secret());

        if let Some(f) = filters {
            request = request.query(&f);
        }

        let response = request.send().await?;

        log::info!("List Issues response status: {}", response.status());

        if response.status().is_success() {
            let issues = response.json::<Vec<IssueDto>>().await?;
            Ok(issues)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("List Issues failed. Status: {}, Body: {}", status, body);
            let err = match status {
                StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TaigaClientError::Unauthorized(status)
                }
                _ => TaigaClientError::AuthFailed(status),
            };
            Err(err)
        }
    }

    /// Fetch detailed issue information by ID
    /// GET /api/v1/issues/{issue_id}
    pub async fn get_issue(
        &self,
        token: &Secret<String>,
        issue_id: i64,
    ) -> Result<IssueDetailDto, TaigaClientError> {
        let url = self.build_url(&format!("issues/{}", issue_id))?;
        log::info!("Fetching issue detail {} from {}", issue_id, url);

        let response = self
            .client
            .get(url)
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        log::info!("Get Issue response status: {}", response.status());

        if response.status().is_success() {
            // Get body as text first for debugging
            let body = response.text().await?;

            // Try to parse
            match serde_json::from_str::<IssueDetailDto>(&body) {
                Ok(issue) => Ok(issue),
                Err(e) => {
                    log::error!("Failed to parse issue detail: {}", e);
                    log::error!(
                        "Raw response body (first 2000 chars): {}",
                        &body[..body.len().min(2000)]
                    );
                    Err(TaigaClientError::Serde(e))
                }
            }
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Get Issue failed. Status: {}, Body: {}", status, body);
            let err = match status {
                StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TaigaClientError::Unauthorized(status)
                }
                _ => TaigaClientError::AuthFailed(status),
            };
            Err(err)
        }
    }

    /// Fetch issue history (comments and changes)
    /// GET /api/v1/history/issue/{issue_id}
    pub async fn get_issue_history(
        &self,
        token: &Secret<String>,
        issue_id: i64,
    ) -> Result<Vec<IssueHistoryEntryDto>, TaigaClientError> {
        let url = self.build_url(&format!("history/issue/{}", issue_id))?;
        log::info!("Fetching issue history {} from {}", issue_id, url);

        let response = self
            .client
            .get(url)
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        log::info!("Get Issue History response status: {}", response.status());

        if response.status().is_success() {
            let history = response.json::<Vec<IssueHistoryEntryDto>>().await?;
            Ok(history)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!(
                "Get Issue History failed. Status: {}, Body: {}",
                status,
                body
            );
            let err = match status {
                StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TaigaClientError::Unauthorized(status)
                }
                _ => TaigaClientError::AuthFailed(status),
            };
            Err(err)
        }
    }

    /// Patch an issue (e.g., change status)
    /// PATCH /api/v1/issues/{issue_id}
    /// Uses version field for optimistic locking - returns VersionConflict on 412
    pub async fn patch_issue(
        &self,
        token: &Secret<String>,
        issue_id: i64,
        request: models::PatchIssueRequest,
    ) -> Result<IssueDetailDto, TaigaClientError> {
        let url = self.build_url(&format!("issues/{}", issue_id))?;
        log::info!(
            "Patching issue {} at {} with version {}",
            issue_id,
            url,
            request.version
        );

        let response = self
            .client
            .patch(url)
            .bearer_auth(token.expose_secret())
            .json(&request)
            .send()
            .await?;

        log::info!("Patch Issue response status: {}", response.status());

        if response.status().is_success() {
            let body = response.text().await?;
            match serde_json::from_str::<IssueDetailDto>(&body) {
                Ok(issue) => Ok(issue),
                Err(e) => {
                    log::error!("Failed to parse patched issue: {}", e);
                    log::error!(
                        "Raw response body (first 2000 chars): {}",
                        &body[..body.len().min(2000)]
                    );
                    Err(TaigaClientError::Serde(e))
                }
            }
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Patch Issue failed. Status: {}, Body: {}", status, body);

            // Taiga returns 400 with body containing "version" error
            // instead of the standard 412 Precondition Failed
            let is_conflict =
                Self::is_version_conflict(&body) || status == StatusCode::PRECONDITION_FAILED;

            let err = if is_conflict {
                TaigaClientError::VersionConflict(status)
            } else {
                match status {
                    StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                    StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                        TaigaClientError::Unauthorized(status)
                    }
                    _ => TaigaClientError::AuthFailed(status),
                }
            };
            Err(err)
        }
    }

    pub async fn upload_issue_attachment(
        &self,
        token: &Secret<String>,
        project_id: i64,
        issue_id: i64,
        file_name: String,
        file_data: Vec<u8>,
    ) -> Result<models::AttachmentDto, TaigaClientError> {
        let url = self.build_url("issues/attachments")?;
        log::info!(
            "Uploading attachment {} for issue {} in project {}",
            file_name,
            issue_id,
            project_id
        );

        let file_part = multipart::Part::bytes(file_data)
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .unwrap();

        let form = multipart::Form::new()
            .text("object_id", issue_id.to_string())
            .text("project", project_id.to_string())
            .part("attached_file", file_part);

        let response = self
            .client
            .post(url)
            .bearer_auth(token.expose_secret())
            .multipart(form)
            .send()
            .await?;

        log::info!("Upload attachment response status: {}", response.status());

        if response.status().is_success() {
            let attachment = response.json::<models::AttachmentDto>().await?;
            Ok(attachment)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Upload attachment failed. Status: {}, Body: {}", status, body);
            let err = match status {
                StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TaigaClientError::Unauthorized(status)
                }
                _ => TaigaClientError::AuthFailed(status),
            };
            Err(err)
        }
    }

    pub async fn delete_issue_attachment(
        &self,
        token: &Secret<String>,
        attachment_id: i64,
    ) -> Result<(), TaigaClientError> {
        let url = self.build_url(&format!("issues/attachments/{}", attachment_id))?;
        log::info!("Deleting attachment {}", attachment_id);

        let response = self
            .client
            .delete(url)
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        log::info!("Delete attachment response status: {}", response.status());

        if response.status().is_success() || response.status() == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Delete attachment failed. Status: {}, Body: {}", status, body);
            let err = match status {
                StatusCode::NOT_FOUND => TaigaClientError::EndpointNotFound(status),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TaigaClientError::Unauthorized(status)
                }
                _ => TaigaClientError::AuthFailed(status),
            };
            Err(err)
        }
    }

    pub async fn get_priorities(
        &self,
        token: &Secret<String>,
        project_id: i64,
    ) -> Result<Vec<models::PriorityDto>, TaigaClientError> {
        let url = self.build_url("priorities")?;
        log::info!("Fetching priorities for project {}", project_id);

        let response = self
            .client
            .get(url)
            .query(&[("project", project_id)])
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            log::error!("Get priorities failed. Status: {}", status);
            Err(TaigaClientError::AuthFailed(status))
        }
    }

    pub async fn get_severities(
        &self,
        token: &Secret<String>,
        project_id: i64,
    ) -> Result<Vec<models::SeverityDto>, TaigaClientError> {
        let url = self.build_url("severities")?;
        log::info!("Fetching severities for project {}", project_id);

        let response = self
            .client
            .get(url)
            .query(&[("project", project_id)])
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            log::error!("Get severities failed. Status: {}", status);
            Err(TaigaClientError::AuthFailed(status))
        }
    }

    pub async fn get_issue_types(
        &self,
        token: &Secret<String>,
        project_id: i64,
    ) -> Result<Vec<models::IssueTypeDto>, TaigaClientError> {
        let url = self.build_url("issue-types")?;
        log::info!("Fetching issue types for project {}", project_id);

        let response = self
            .client
            .get(url)
            .query(&[("project", project_id)])
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            log::error!("Get issue types failed. Status: {}", status);
            Err(TaigaClientError::AuthFailed(status))
        }
    }

    pub async fn get_project_tags_colors(
        &self,
        token: &Secret<String>,
        project_id: i64,
    ) -> Result<serde_json::Value, TaigaClientError> {
        let url = self.build_url(&format!("projects/{}/tags_colors", project_id))?;
        log::info!("Fetching tags colors for project {}", project_id);

        let response = self
            .client
            .get(url)
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            log::error!("Get tags colors failed. Status: {}", status);
            Err(TaigaClientError::AuthFailed(status))
        }
    }

    pub async fn get_memberships(
        &self,
        token: &Secret<String>,
        project_id: i64,
    ) -> Result<Vec<models::MembershipDto>, TaigaClientError> {
        let url = self.build_url("memberships")?;
        log::info!("Fetching memberships for project {}", project_id);

        let response = self
            .client
            .get(url)
            .query(&[("project", project_id)])
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.text().await?;
            match serde_json::from_str::<Vec<models::MembershipDto>>(&body) {
                Ok(memberships) => Ok(memberships),
                Err(e) => {
                    log::error!(
                        "Failed to parse memberships: {}. Body (first 500 chars): {}",
                        e,
                        &body[..body.len().min(500)]
                    );
                    Err(TaigaClientError::Serde(e))
                }
            }
        } else {
            let status = response.status();
            log::error!("Get memberships failed. Status: {}", status);
            Err(TaigaClientError::AuthFailed(status))
        }
    }
}
