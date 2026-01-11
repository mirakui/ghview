use keyring::Entry;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const KEYRING_SERVICE: &str = "ghview";
const KEYRING_USER: &str = "github_oauth_token";

// GitHub OAuth App credentials for ghview
// Device flow doesn't require a client secret
const GITHUB_CLIENT_ID: &str = "Iv23li78KgNyGR5C061j";

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Keyring error: {0}")]
    Keyring(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("OAuth error: {0}")]
    OAuth(String),
    #[error("Not authenticated")]
    NotAuthenticated,
}

impl Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    pub authenticated: bool,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u32,
    interval: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AccessTokenResponse {
    access_token: Option<String>,
    token_type: Option<String>,
    scope: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeviceFlowInit {
    pub user_code: String,
    pub verification_uri: String,
    pub device_code: String,
    pub expires_in: u32,
    pub interval: u32,
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
}

fn get_keyring_entry() -> Result<Entry, AuthError> {
    Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| AuthError::Keyring(e.to_string()))
}

#[tauri::command]
pub async fn check_auth_status() -> Result<AuthStatus, AuthError> {
    let entry = get_keyring_entry()?;

    match entry.get_password() {
        Ok(token) => {
            // Verify token by fetching user info
            let client = reqwest::Client::new();
            let response = client
                .get("https://api.github.com/user")
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", "ghview")
                .header("Accept", "application/vnd.github+json")
                .send()
                .await?;

            if response.status().is_success() {
                let user: GitHubUser = response.json().await?;
                Ok(AuthStatus {
                    authenticated: true,
                    username: Some(user.login),
                })
            } else {
                // Token is invalid, remove it
                let _ = entry.delete_credential();
                Ok(AuthStatus {
                    authenticated: false,
                    username: None,
                })
            }
        }
        Err(_) => Ok(AuthStatus {
            authenticated: false,
            username: None,
        }),
    }
}

#[tauri::command]
pub async fn start_device_flow() -> Result<DeviceFlowInit, AuthError> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[("client_id", GITHUB_CLIENT_ID), ("scope", "repo")])
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AuthError::OAuth(format!(
            "Failed to start device flow: {}",
            error_text
        )));
    }

    let device_code_response: DeviceCodeResponse = response.json().await?;

    Ok(DeviceFlowInit {
        user_code: device_code_response.user_code,
        verification_uri: device_code_response.verification_uri,
        device_code: device_code_response.device_code,
        expires_in: device_code_response.expires_in,
        interval: device_code_response.interval,
    })
}

#[tauri::command]
pub async fn poll_device_flow(device_code: String) -> Result<AuthStatus, AuthError> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", GITHUB_CLIENT_ID),
            ("device_code", &device_code),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ])
        .send()
        .await?;

    let token_response: AccessTokenResponse = response.json().await?;

    if let Some(error) = token_response.error {
        if error == "authorization_pending" {
            return Ok(AuthStatus {
                authenticated: false,
                username: None,
            });
        }
        return Err(AuthError::OAuth(
            token_response
                .error_description
                .unwrap_or(error.to_string()),
        ));
    }

    if let Some(access_token) = token_response.access_token {
        // Store the token
        let entry = get_keyring_entry()?;
        entry
            .set_password(&access_token)
            .map_err(|e| AuthError::Keyring(e.to_string()))?;

        // Get username
        let user_response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "ghview")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if user_response.status().is_success() {
            let user: GitHubUser = user_response.json().await?;
            return Ok(AuthStatus {
                authenticated: true,
                username: Some(user.login),
            });
        }
    }

    Err(AuthError::OAuth(
        "Failed to obtain access token".to_string(),
    ))
}

#[tauri::command]
pub async fn logout() -> Result<(), AuthError> {
    let entry = get_keyring_entry()?;
    entry
        .delete_credential()
        .map_err(|e| AuthError::Keyring(e.to_string()))?;
    Ok(())
}

pub fn get_stored_token() -> Result<String, AuthError> {
    let entry = get_keyring_entry()?;
    entry
        .get_password()
        .map_err(|_| AuthError::NotAuthenticated)
}
