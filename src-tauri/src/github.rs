use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::auth;

#[derive(Error, Debug)]
pub enum GitHubError {
    #[error("Authentication error: {0}")]
    Auth(#[from] auth::AuthError),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
}

impl Serialize for GitHubError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub full_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PullRequest {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub html_url: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
    pub repository: Repository,
    pub draft: bool,
    pub labels: Vec<Label>,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
struct SearchItem {
    id: u64,
    number: u64,
    title: String,
    html_url: String,
    user: User,
    created_at: String,
    updated_at: String,
    repository_url: String,
    draft: Option<bool>,
    labels: Vec<Label>,
}

#[tauri::command]
pub async fn fetch_pull_requests() -> Result<Vec<PullRequest>, GitHubError> {
    let token = auth::get_token()?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/search/issues")
        .query(&[
            ("q", "is:pr is:open review-requested:@me"),
            ("sort", "updated"),
            ("order", "desc"),
            ("per_page", "50"),
        ])
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "ghview")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(GitHubError::Api(format!(
            "GitHub API error ({}): {}",
            status, error_text
        )));
    }

    let search_response: SearchResponse = response.json().await?;

    let pull_requests: Vec<PullRequest> = search_response
        .items
        .into_iter()
        .map(|item| {
            // Extract repo name from repository_url
            // Format: https://api.github.com/repos/owner/repo
            let full_name = item
                .repository_url
                .strip_prefix("https://api.github.com/repos/")
                .unwrap_or(&item.repository_url)
                .to_string();

            PullRequest {
                id: item.id,
                number: item.number,
                title: item.title,
                html_url: item.html_url,
                user: item.user,
                created_at: item.created_at,
                updated_at: item.updated_at,
                repository: Repository { full_name },
                draft: item.draft.unwrap_or(false),
                labels: item.labels,
            }
        })
        .collect();

    Ok(pull_requests)
}
