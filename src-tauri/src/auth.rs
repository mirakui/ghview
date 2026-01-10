use serde::{Deserialize, Serialize};
use thiserror::Error;

const GITHUB_CLIENT_ID: &str = "Iv23li78KgNyGR5C061j";
const KEYRING_SERVICE: &str = "ghview";
const KEYRING_USER: &str = "github_token";

#[derive(Error, Debug)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceFlowInit {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
}

fn get_stored_token() -> Result<String, AuthError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| AuthError::Keyring(e.to_string()))?;
    entry
        .get_password()
        .map_err(|_| AuthError::NotAuthenticated)
}

fn store_token(token: &str) -> Result<(), AuthError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| AuthError::Keyring(e.to_string()))?;
    entry
        .set_password(token)
        .map_err(|e| AuthError::Keyring(e.to_string()))
}

fn delete_token() -> Result<(), AuthError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| AuthError::Keyring(e.to_string()))?;
    // Ignore error if credential doesn't exist
    let _ = entry.delete_credential();
    Ok(())
}

#[tauri::command]
pub async fn check_auth_status() -> Result<AuthStatus, AuthError> {
    let token = match get_stored_token() {
        Ok(t) => t,
        Err(_) => {
            return Ok(AuthStatus {
                authenticated: false,
                username: None,
            })
        }
    };

    // Validate token by calling GitHub API
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
        // Token is invalid, delete it
        let _ = delete_token();
        Ok(AuthStatus {
            authenticated: false,
            username: None,
        })
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

    let device_code: DeviceCodeResponse = response.json().await?;

    Ok(DeviceFlowInit {
        device_code: device_code.device_code,
        user_code: device_code.user_code,
        verification_uri: device_code.verification_uri,
        expires_in: device_code.expires_in,
        interval: device_code.interval,
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
        let description = token_response.error_description.unwrap_or_default();
        return Err(AuthError::OAuth(format!("{}: {}", error, description)));
    }

    if let Some(access_token) = token_response.access_token {
        store_token(&access_token)?;

        // Get username
        let user_response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "ghview")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        let user: GitHubUser = user_response.json().await?;

        Ok(AuthStatus {
            authenticated: true,
            username: Some(user.login),
        })
    } else {
        Err(AuthError::OAuth("No access token received".to_string()))
    }
}

#[tauri::command]
pub async fn logout() -> Result<(), AuthError> {
    delete_token()
}

pub fn get_token() -> Result<String, AuthError> {
    get_stored_token()
}
