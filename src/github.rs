//! Functions to call the GitHub API
use crate::env;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubUser {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubPullRequest {
    pub number: i32,
    pub title: String,
    pub html_url: String,
    pub user: GitHubUser,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitHubPullRequestFile {
    pub sha: String,
    pub filename: String,
    pub additions: i32,
    pub deletions: i32,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitHubPullRequestCommit {
    pub sha: String,
    pub commit: GitHubCommitInfo,
}

#[derive(Serialize, Deserialize)]
pub struct GitHubCommitInfo {
    pub author: GitHubCommitAuthor,
}

#[derive(Serialize, Deserialize)]
pub struct GitHubCommitAuthor {
    pub name: String,
    pub email: String,
}

pub struct GitHub {
    client: Client,
}

impl GitHub {
    pub fn new(env: &env::Env) -> anyhow::Result<Self> {
        let github_token = env.github_token.clone();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", github_token))?,
        );

        let client = Client::builder()
            .user_agent("kwoc-stats-tracker")
            .default_headers(headers)
            .build()?;
        Ok(Self { client })
    }

    /// Get a project's pull requests
    pub async fn get_project_pull_requests(
        &self,
        repo_owner: &str,
        repo_name: &str,
        page: i32,
    ) -> anyhow::Result<Vec<GitHubPullRequest>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls?per_page=100&page={}&state=all",
            repo_owner, repo_name, page
        );
        let response = self.client.get(&url).send().await?;
        let pull_requests: Vec<GitHubPullRequest> = response.json().await?;
        Ok(pull_requests)
    }

    /// Get the list of commits in a pull request
    pub async fn get_pull_request_commits(
        &self,
        repo_owner: &str,
        repo_name: &str,
        pull_request_number: i32,
    ) -> anyhow::Result<Vec<GitHubPullRequestCommit>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{}/commits?per_page=100",
            repo_owner, repo_name, pull_request_number
        );
        let response = self.client.get(&url).send().await?;
        let commits: Vec<GitHubPullRequestCommit> = response.json().await?;
        Ok(commits)
    }

    /// Get the list of files changed in a pull request
    pub async fn get_pull_request_files(
        &self,
        repo_owner: &str,
        repo_name: &str,
        pull_request_number: i32,
    ) -> anyhow::Result<Vec<GitHubPullRequestFile>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{}/files?per_page=100",
            repo_owner, repo_name, pull_request_number
        );
        let response = self.client.get(&url).send().await?;
        let files: Vec<GitHubPullRequestFile> = response.json().await?;
        Ok(files)
    }
}
