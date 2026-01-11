use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

#[cfg(target_os = "macos")]
const SOCKET_PATH: &str = "/tmp/ghview.sock";

#[cfg(target_os = "linux")]
const SOCKET_PATH: &str = "/tmp/ghview.sock";

#[cfg(target_os = "windows")]
const SOCKET_PATH: &str = r"\\.\pipe\ghview";

#[derive(Clone)]
pub struct IpcClient;

#[derive(Serialize)]
struct IpcRequest {
    method: String,
    params: serde_json::Value,
}

#[derive(Deserialize)]
struct IpcResponse {
    result: Option<serde_json::Value>,
    error: Option<String>,
}

impl IpcClient {
    pub async fn connect() -> Result<Self> {
        let socket_path = Path::new(SOCKET_PATH);
        if !socket_path.exists() {
            anyhow::bail!(
                "ghview is not running. Please start ghview first. (Socket not found: {})",
                SOCKET_PATH
            );
        }
        Ok(Self)
    }

    async fn send_request(&self, method: &str, params: serde_json::Value) -> Result<String> {
        let mut stream = UnixStream::connect(SOCKET_PATH)
            .await
            .context("Failed to connect to ghview")?;

        let request = IpcRequest {
            method: method.to_string(),
            params,
        };

        let mut request_json = serde_json::to_string(&request)?;
        request_json.push('\n');

        stream.write_all(request_json.as_bytes()).await?;

        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        let response_line = lines
            .next_line()
            .await?
            .context("No response from ghview")?;

        let response: IpcResponse = serde_json::from_str(&response_line)?;

        if let Some(error) = response.error {
            anyhow::bail!(error);
        }

        Ok(response
            .result
            .map(|v| serde_json::to_string_pretty(&v).unwrap_or_default())
            .unwrap_or_default())
    }

    pub async fn screenshot(&self, output_dir: &str) -> Result<String> {
        self.send_request(
            "screenshot",
            serde_json::json!({
                "output_dir": output_dir
            }),
        )
        .await
    }
}
