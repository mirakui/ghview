use crate::commands::auth::{get_stored_token, AuthError};
use crate::models::{
    CheckState, CheckStatus, Label, PullRequest, PullRequestState, PullRequestWithChecks,
    Repository, StatusCheck, User,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubError {
    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
}

impl serde::Serialize for GitHubError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SearchResponse {
    total_count: i32,
    incomplete_results: bool,
    items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
struct SearchItem {
    id: i64,
    number: i32,
    title: String,
    html_url: String,
    state: String,
    draft: Option<bool>,
    created_at: String,
    updated_at: String,
    merged_at: Option<String>,
    user: ApiUser,
    labels: Vec<ApiLabel>,
    requested_reviewers: Option<Vec<ApiUser>>,
    repository_url: String,
}

#[derive(Debug, Deserialize)]
struct ApiUser {
    id: i64,
    login: String,
    avatar_url: String,
    html_url: String,
}

#[derive(Debug, Deserialize)]
struct ApiLabel {
    id: i64,
    name: String,
    color: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiRepository {
    id: i64,
    name: String,
    full_name: String,
    html_url: String,
    owner: ApiUser,
}

#[derive(Debug, Deserialize)]
struct ApiCombinedStatus {
    state: String,
    total_count: i32,
    statuses: Vec<ApiStatus>,
}

#[derive(Debug, Deserialize)]
struct ApiStatus {
    state: String,
    context: String,
    description: Option<String>,
    target_url: Option<String>,
}

impl From<ApiUser> for User {
    fn from(api_user: ApiUser) -> Self {
        User {
            id: api_user.id,
            login: api_user.login,
            avatar_url: api_user.avatar_url,
            html_url: api_user.html_url,
        }
    }
}

impl From<ApiLabel> for Label {
    fn from(api_label: ApiLabel) -> Self {
        Label {
            id: api_label.id,
            name: api_label.name,
            color: api_label.color,
            description: api_label.description,
        }
    }
}

impl From<ApiRepository> for Repository {
    fn from(api_repo: ApiRepository) -> Self {
        Repository {
            id: api_repo.id,
            name: api_repo.name,
            full_name: api_repo.full_name,
            html_url: api_repo.html_url,
            owner: api_repo.owner.into(),
        }
    }
}

fn parse_state(state: &str) -> PullRequestState {
    match state.to_lowercase().as_str() {
        "open" => PullRequestState::Open,
        "closed" => PullRequestState::Closed,
        _ => PullRequestState::Open,
    }
}

fn parse_check_state(state: &str) -> CheckState {
    match state.to_lowercase().as_str() {
        "success" => CheckState::Success,
        "pending" => CheckState::Pending,
        "failure" => CheckState::Failure,
        "error" => CheckState::Error,
        _ => CheckState::Pending,
    }
}

#[tauri::command]
pub async fn fetch_review_requested_prs() -> Result<Vec<PullRequestWithChecks>, GitHubError> {
    let token = get_stored_token()?;
    let client = reqwest::Client::new();

    // Search for PRs where review is requested from the authenticated user
    let search_url = "https://api.github.com/search/issues";
    let query = "is:pr is:open review-requested:@me";

    let response = client
        .get(search_url)
        .query(&[("q", query), ("sort", "updated"), ("order", "desc")])
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "ghview")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(GitHubError::Api(format!(
            "Failed to fetch PRs: {}",
            error_text
        )));
    }

    let search_response: SearchResponse = response.json().await?;

    let mut prs_with_checks = Vec::new();

    for item in search_response.items {
        // Fetch repository details
        let repo_response = client
            .get(&item.repository_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "ghview")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        let repository: Repository = if repo_response.status().is_success() {
            let api_repo: ApiRepository = repo_response.json().await?;
            api_repo.into()
        } else {
            // Fallback: extract repo info from URL
            let parts: Vec<&str> = item.repository_url.split('/').collect();
            let repo_name = parts.last().unwrap_or(&"unknown").to_string();
            let owner_name = parts
                .get(parts.len().saturating_sub(2))
                .unwrap_or(&"unknown");
            Repository {
                id: 0,
                name: repo_name.clone(),
                full_name: format!("{}/{}", owner_name, repo_name),
                html_url: item
                    .repository_url
                    .replace("api.github.com/repos", "github.com"),
                owner: User {
                    id: 0,
                    login: owner_name.to_string(),
                    avatar_url: String::new(),
                    html_url: format!("https://github.com/{}", owner_name),
                },
            }
        };

        // Fetch PR details to get requested_reviewers
        let pr_url = format!(
            "https://api.github.com/repos/{}/pulls/{}",
            repository.full_name, item.number
        );
        let pr_response = client
            .get(&pr_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "ghview")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        let requested_reviewers: Vec<User> = if pr_response.status().is_success() {
            #[derive(Deserialize)]
            struct PrDetail {
                requested_reviewers: Vec<ApiUser>,
            }
            let pr_detail: PrDetail = pr_response.json().await?;
            pr_detail
                .requested_reviewers
                .into_iter()
                .map(|u| u.into())
                .collect()
        } else {
            item.requested_reviewers
                .unwrap_or_default()
                .into_iter()
                .map(|u| u.into())
                .collect()
        };

        // Fetch check status
        let status_url = format!(
            "https://api.github.com/repos/{}/commits/HEAD/status",
            repository.full_name
        );
        let _status_response = client
            .get(&status_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "ghview")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await;

        // Parse dates
        let created_at = chrono::DateTime::parse_from_rfc3339(&item.created_at)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());
        let updated_at = chrono::DateTime::parse_from_rfc3339(&item.updated_at)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());
        let merged_at = item.merged_at.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .ok()
        });

        let pr = PullRequest {
            id: item.id,
            number: item.number,
            title: item.title,
            html_url: item.html_url,
            state: parse_state(&item.state),
            draft: item.draft.unwrap_or(false),
            created_at,
            updated_at,
            merged_at,
            user: item.user.into(),
            labels: item.labels.into_iter().map(|l| l.into()).collect(),
            requested_reviewers,
            repository,
        };

        // Fetch combined status for the PR's head commit
        let check_status = fetch_pr_check_status(&client, &token, &pr).await.ok();

        prs_with_checks.push(PullRequestWithChecks {
            pull_request: pr,
            check_status,
        });
    }

    Ok(prs_with_checks)
}

async fn fetch_pr_check_status(
    client: &reqwest::Client,
    token: &str,
    pr: &PullRequest,
) -> Result<CheckStatus, GitHubError> {
    // Get the PR's head SHA
    let pr_url = format!(
        "https://api.github.com/repos/{}/pulls/{}",
        pr.repository.full_name, pr.number
    );

    let pr_response = client
        .get(&pr_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "ghview")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?;

    if !pr_response.status().is_success() {
        return Err(GitHubError::Api("Failed to fetch PR details".to_string()));
    }

    #[derive(Deserialize)]
    struct PrHead {
        head: PrHeadRef,
    }

    #[derive(Deserialize)]
    struct PrHeadRef {
        sha: String,
    }

    let pr_head: PrHead = pr_response.json().await?;

    // Fetch combined status
    let status_url = format!(
        "https://api.github.com/repos/{}/commits/{}/status",
        pr.repository.full_name, pr_head.head.sha
    );

    let status_response = client
        .get(&status_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "ghview")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?;

    if !status_response.status().is_success() {
        return Ok(CheckStatus {
            state: CheckState::Pending,
            total_count: 0,
            statuses: vec![],
        });
    }

    let api_status: ApiCombinedStatus = status_response.json().await?;

    Ok(CheckStatus {
        state: parse_check_state(&api_status.state),
        total_count: api_status.total_count,
        statuses: api_status
            .statuses
            .into_iter()
            .map(|s| StatusCheck {
                state: parse_check_state(&s.state),
                context: s.context,
                description: s.description,
                target_url: s.target_url,
            })
            .collect(),
    })
}
