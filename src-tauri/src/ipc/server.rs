use crate::ipc::protocol::{IpcRequest, IpcResponse, ScreenshotParams, ScreenshotResult};
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;

#[cfg(target_os = "macos")]
const SOCKET_PATH: &str = "/tmp/ghview.sock";

#[cfg(target_os = "linux")]
const SOCKET_PATH: &str = "/tmp/ghview.sock";

#[cfg(target_os = "windows")]
const SOCKET_PATH: &str = r"\\.\pipe\ghview";

#[allow(dead_code)]
pub fn get_socket_path() -> &'static str {
    SOCKET_PATH
}

pub async fn start_ipc_server(app: AppHandle) -> Result<()> {
    let socket_path = Path::new(SOCKET_PATH);
    if socket_path.exists() {
        std::fs::remove_file(socket_path).context("Failed to remove existing socket")?;
    }

    let listener = UnixListener::bind(SOCKET_PATH).context("Failed to bind Unix socket")?;
    let app = Arc::new(app);

    log::info!("IPC server listening on {}", SOCKET_PATH);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let app = Arc::clone(&app);
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, app).await {
                        log::error!("IPC connection error: {}", e);
                    }
                });
            }
            Err(e) => {
                log::error!("IPC accept error: {}", e);
            }
        }
    }
}

async fn handle_connection(stream: tokio::net::UnixStream, app: Arc<AppHandle>) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    reader.read_line(&mut line).await?;

    let response = match serde_json::from_str::<IpcRequest>(&line) {
        Ok(request) => handle_request(&request, &app).await,
        Err(e) => IpcResponse::error(format!("Invalid request: {}", e)),
    };

    let response_json = serde_json::to_string(&response)?;
    writer.write_all(response_json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;

    Ok(())
}

async fn handle_request(request: &IpcRequest, app: &AppHandle) -> IpcResponse {
    match request.method.as_str() {
        "screenshot" => handle_screenshot(request, app).await,
        "ping" => IpcResponse::success(serde_json::json!({"pong": true})),
        _ => IpcResponse::error(format!("Unknown method: {}", request.method)),
    }
}

async fn handle_screenshot(request: &IpcRequest, app: &AppHandle) -> IpcResponse {
    let params: ScreenshotParams = match serde_json::from_value(request.params.clone()) {
        Ok(p) => p,
        Err(e) => return IpcResponse::error(format!("Invalid screenshot params: {}", e)),
    };

    match capture_screenshot(app, &params.output_dir).await {
        Ok(path) => {
            let result = ScreenshotResult { path };
            IpcResponse::success(serde_json::to_value(result).unwrap())
        }
        Err(e) => IpcResponse::error(format!("Screenshot failed: {}", e)),
    }
}

async fn capture_screenshot(app: &AppHandle, output_dir: &str) -> Result<String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Verify the window exists
    let _window = app
        .get_webview_window("main")
        .context("Main window not found")?;

    let output_path = Path::new(output_dir);
    if !output_path.exists() {
        std::fs::create_dir_all(output_path).context("Failed to create output directory")?;
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let filename = format!("ghview-screenshot-{}.png", timestamp);
    let file_path = output_path.join(&filename);

    // Use xcap to capture the window
    capture_window_screenshot(&file_path)?;

    Ok(file_path.to_string_lossy().to_string())
}

fn capture_window_screenshot(output_path: &Path) -> Result<()> {
    use xcap::image::imageops::FilterType;
    use xcap::image::DynamicImage;
    use xcap::Window;

    // Find the ghview window by app_name (more reliable than title)
    let windows = Window::all().context("Failed to enumerate windows")?;
    let ghview_window = windows
        .into_iter()
        .find(|w| {
            w.app_name()
                .map(|name| name.to_lowercase() == "ghview")
                .unwrap_or(false)
        })
        .context("ghview window not found (is ghview running?)")?;

    // Get logical window size (before scaling)
    let logical_width = ghview_window.width().context("Failed to get window width")?;
    let logical_height = ghview_window.height().context("Failed to get window height")?;

    // Capture the window (this returns the actual pixel size on Retina displays)
    let image = ghview_window
        .capture_image()
        .context("Failed to capture window image")?;

    // Resize to logical size if the captured image is larger (Retina display)
    let resized = if image.width() > logical_width || image.height() > logical_height {
        DynamicImage::ImageRgba8(image)
            .resize_exact(logical_width, logical_height, FilterType::Lanczos3)
    } else {
        DynamicImage::ImageRgba8(image)
    };

    // Save as PNG
    resized
        .save(output_path)
        .context("Failed to save screenshot")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_path() {
        let path = get_socket_path();
        assert!(!path.is_empty());
    }
}
