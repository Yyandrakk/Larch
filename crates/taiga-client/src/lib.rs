use reqwest::StatusCode;
use secrecy::{ExposeSecret, Secret};
use url::Url;

pub mod errors;
pub mod models;
pub mod prelude;

use errors::TaigaClientError;
use models::{AuthDetail, IssueDto, LoginRequest, Me, ProjectDto};

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
        let request_body = LoginRequest {
            r#type: "normal",
            username,
            password,
        };

        let response = self.client.post(url).json(&request_body).send().await?;

        if response.status().is_success() {
            let auth_detail = response.json::<AuthDetail>().await?;
            Ok(auth_detail)
        } else {
            let status = response.status();
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

        let response = self
            .client
            .get(url)
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        if response.status().is_success() {
            let me = response.json::<Me>().await?;
            Ok(me)
        } else {
            let status = response.status();
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
    ) -> Result<Vec<ProjectDto>, TaigaClientError> {
        let url = self.build_url("projects")?;

        let response = self
            .client
            .get(url)
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        if response.status().is_success() {
            let projects = response.json::<Vec<ProjectDto>>().await?;
            Ok(projects)
        } else {
            let status = response.status();
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
    ) -> Result<Vec<IssueDto>, TaigaClientError> {
        let url = self.build_url("issues")?;

        let response = self
            .client
            .get(url)
            .query(&[("project", project_id)])
            .bearer_auth(token.expose_secret())
            .send()
            .await?;

        if response.status().is_success() {
            let issues = response.json::<Vec<IssueDto>>().await?;
            Ok(issues)
        } else {
            let status = response.status();
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
}
