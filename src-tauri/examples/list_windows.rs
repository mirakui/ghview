// Run with: cargo run --example list_windows
// Or: rustc list_windows.rs -o list_windows && ./list_windows

use xcap::Window;

fn main() {
    let windows = Window::all().expect("Failed to enumerate windows");

    println!("Found {} windows:", windows.len());
    println!("{:-<80}", "");

    for (i, window) in windows.iter().enumerate() {
        let title = window.title().unwrap_or_else(|_| "<error>".to_string());
        let app_name = window.app_name().unwrap_or_else(|_| "<error>".to_string());
        let id = window.id().unwrap_or(0);

        let is_ghview = title.to_lowercase().contains("ghview")
            || app_name.to_lowercase().contains("ghview");
        let marker = if is_ghview { " <-- MATCH" } else { "" };

        println!("[{}] ID: {}", i, id);
        println!("    Title: {:?}", title);
        println!("    App:   {:?}{}", app_name, marker);
        println!();
    }
}
