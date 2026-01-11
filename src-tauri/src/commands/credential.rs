use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CredentialError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Credential not found")]
    NotFound,
    #[error("Home directory not found")]
    HomeDirNotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub access_token: String,
}

/// Get the path to the credentials file (~/.config/ghview/credentials.json)
pub fn get_credentials_path() -> Result<PathBuf, CredentialError> {
    let home = std::env::var("HOME").map_err(|_| CredentialError::HomeDirNotFound)?;
    Ok(PathBuf::from(home)
        .join(".config")
        .join("ghview")
        .join("credentials.json"))
}

/// Save credentials to file
pub fn save_credentials(credentials: &Credentials) -> Result<(), CredentialError> {
    let path = get_credentials_path()?;

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(credentials)?;
    fs::write(&path, json)?;

    // Set file permissions to 600 (owner read/write only) on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, permissions)?;
    }

    Ok(())
}

/// Load credentials from file
pub fn load_credentials() -> Result<Credentials, CredentialError> {
    let path = get_credentials_path()?;

    if !path.exists() {
        return Err(CredentialError::NotFound);
    }

    let content = fs::read_to_string(&path)?;
    let credentials: Credentials = serde_json::from_str(&content)?;
    Ok(credentials)
}

/// Delete credentials file
pub fn delete_credentials() -> Result<(), CredentialError> {
    let path = get_credentials_path()?;

    if path.exists() {
        fs::remove_file(&path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::fs;

    struct TestEnv {
        original_home: Option<String>,
        temp_dir: PathBuf,
    }

    impl TestEnv {
        fn new() -> Self {
            let temp_dir = env::temp_dir().join(format!("ghview_test_{}", rand::random::<u64>()));
            fs::create_dir_all(&temp_dir).unwrap();
            let original_home = env::var("HOME").ok();
            env::set_var("HOME", &temp_dir);
            TestEnv {
                original_home,
                temp_dir,
            }
        }
    }

    impl Drop for TestEnv {
        fn drop(&mut self) {
            if let Some(ref home) = self.original_home {
                env::set_var("HOME", home);
            } else {
                env::remove_var("HOME");
            }
            let _ = fs::remove_dir_all(&self.temp_dir);
        }
    }

    #[test]
    #[serial]
    fn test_get_credentials_path() {
        let _env = TestEnv::new();
        let path = get_credentials_path().unwrap();
        assert!(path.ends_with(".config/ghview/credentials.json"));
    }

    #[test]
    #[serial]
    fn test_save_and_load_credentials() {
        let _env = TestEnv::new();
        let credentials = Credentials {
            access_token: "test_token_123".to_string(),
        };

        save_credentials(&credentials).unwrap();
        let loaded = load_credentials().unwrap();
        assert_eq!(loaded.access_token, "test_token_123");
    }

    #[test]
    #[serial]
    fn test_load_credentials_not_found() {
        let _env = TestEnv::new();
        let result = load_credentials();
        assert!(matches!(result, Err(CredentialError::NotFound)));
    }

    #[test]
    #[serial]
    fn test_delete_credentials() {
        let _env = TestEnv::new();
        let credentials = Credentials {
            access_token: "test_token".to_string(),
        };
        save_credentials(&credentials).unwrap();

        delete_credentials().unwrap();

        let result = load_credentials();
        assert!(matches!(result, Err(CredentialError::NotFound)));
    }

    #[test]
    #[serial]
    fn test_delete_credentials_not_found() {
        let _env = TestEnv::new();
        // Should not error if file doesn't exist
        let result = delete_credentials();
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_credentials_json_format() {
        let _env = TestEnv::new();
        let credentials = Credentials {
            access_token: "gho_test123".to_string(),
        };
        save_credentials(&credentials).unwrap();

        let path = get_credentials_path().unwrap();
        let content = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["access_token"], "gho_test123");
    }
}
