use reqwest::StatusCode;
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
            let project = response.json::<ProjectDto>().await?;
            Ok(project)
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

    /// Retrieves the history entries (comments and change events) for a specific issue.
    ///
    /// The request is authenticated with the provided bearer token.
    ///
    /// # Parameters
    ///
    /// - `token`: Bearer token used for authentication.
    /// - `issue_id`: Identifier of the issue whose history will be fetched.
    ///
    /// # Returns
    ///
    /// A vector of `IssueHistoryEntryDto` representing the issue's history entries.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use secrecy::Secret;
    /// use url::Url;
    /// // Construct client and token (placeholders)
    /// let client = taiga_client::TaigaClient::new(Url::parse("https://taiga.example/").unwrap());
    /// let token = Secret::new("MY_TOKEN".to_string());
    /// // In an async context:
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let history = client.get_issue_history(&token, 123).await?;
    /// assert!(history.is_empty() || history.len() > 0);
    /// # Ok(())
    /// # }
    /// ```
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

    /// Patch an existing issue using optimistic locking via the request's `version` field.
    ///
    /// Sends a PATCH to `/api/v1/issues/{issue_id}` and returns the updated issue on success.
    /// If the server reports a version mismatch (detected by HTTP 412 Precondition Failed or a
    /// response body mentioning a `version` mismatch), this returns `TaigaClientError::VersionConflict`.
    /// Other HTTP failure statuses are mapped to appropriate `TaigaClientError` variants.
    ///
    /// # Parameters
    ///
    /// * `request` - The patch request; its `version` field is used for optimistic locking.
    ///
    /// # Returns
    ///
    /// The updated `IssueDetailDto`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use secrecy::Secret;
    /// # use url::Url;
    /// # use taiga_client::{TaigaClient, models};
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let api_base = Url::parse("https://taiga.example.com/")?;
    /// let client = TaigaClient::new(api_base);
    /// let token = Secret::new("MY_TOKEN".to_string());
    /// let request = models::PatchIssueRequest { version: 1, /* other fields */ ..Default::default() };
    /// let updated = client.patch_issue(&token, 123, request).await?;
    /// println!("Updated issue id: {}", updated.id);
    /// # Ok(()) }
    /// ```
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

            // Check for version conflict - Taiga returns 400 with body containing "version" error
            // instead of the standard 412 Precondition Failed
            let is_version_conflict = body.to_lowercase().contains("version")
                && (body.contains("doesn't match") || body.contains("does not match"));

            let err = if is_version_conflict || status == StatusCode::PRECONDITION_FAILED {
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
}