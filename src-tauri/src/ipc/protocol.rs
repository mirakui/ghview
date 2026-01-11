use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcRequest {
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl IpcResponse {
    pub fn success(result: serde_json::Value) -> Self {
        Self {
            result: Some(result),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            result: None,
            error: Some(message.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotParams {
    pub output_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotResult {
    pub path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_request_serialization() {
        let request = IpcRequest {
            method: "screenshot".to_string(),
            params: serde_json::json!({"output_dir": "/tmp"}),
        };
        let json = serde_json::to_string(&request).unwrap();
        let parsed: IpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.method, "screenshot");
    }

    #[test]
    fn test_ipc_response_success() {
        let response = IpcResponse::success(serde_json::json!({"path": "/tmp/screenshot.png"}));
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_ipc_response_error() {
        let response = IpcResponse::error("Failed to capture screenshot");
        assert!(response.result.is_none());
        assert_eq!(
            response.error,
            Some("Failed to capture screenshot".to_string())
        );
    }

    #[test]
    fn test_screenshot_params_deserialization() {
        let json = r#"{"output_dir": "/tmp/screenshots"}"#;
        let params: ScreenshotParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.output_dir, "/tmp/screenshots");
    }
}
