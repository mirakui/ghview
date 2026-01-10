mod commands;
mod models;

use commands::{
    auth::{check_auth_status, logout, poll_device_flow, start_device_flow},
    github::fetch_review_requested_prs,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            check_auth_status,
            start_device_flow,
            poll_device_flow,
            logout,
            fetch_review_requested_prs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
