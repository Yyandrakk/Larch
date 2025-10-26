use secrecy::{ExposeSecret, Secret};
use url::Url;

pub mod errors;
pub mod models;
pub mod prelude;

use errors::TaigaClientError;
use models::{AuthDetail, IssueDto, LoginRequest, Me, ProjectDto};

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

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthDetail, TaigaClientError> {
        let url = self.api_base_url.join("auth")?;
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
            Err(TaigaClientError::AuthFailed(response.status()))
        }
    }

    pub async fn get_me(&self, token: &Secret<String>) -> Result<Me, TaigaClientError> {
        let url = self.api_base_url.join("users/me")?;

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
            Err(TaigaClientError::AuthFailed(response.status()))
        }
    }

    pub async fn get_projects(
        &self,
        token: &Secret<String>,
    ) -> Result<Vec<ProjectDto>, TaigaClientError> {
        let url = self.api_base_url.join("projects")?;

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
            Err(TaigaClientError::AuthFailed(response.status()))
        }
    }

    pub async fn list_issues(
        &self,
        token: &Secret<String>,
        project_id: i64,
    ) -> Result<Vec<IssueDto>, TaigaClientError> {
        let url = self.api_base_url.join("issues")?;

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
            Err(TaigaClientError::AuthFailed(response.status()))
        }
    }
}
