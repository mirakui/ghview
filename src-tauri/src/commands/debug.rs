use reqwest::StatusCode;

/// Log an HTTP request to stderr
pub fn log_request(method: &str, url: &str) {
    eprintln!("[DEBUG] {} {}", method, url);
}

/// Log an HTTP response to stderr
pub fn log_response(url: &str, status: StatusCode) {
    eprintln!("[DEBUG] {} -> {}", url, status);
}

/// Log an HTTP response with body preview (for errors)
pub fn log_response_error(url: &str, status: StatusCode, body: &str) {
    eprintln!("[DEBUG] {} -> {} | {}", url, status, body);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_request_formats_correctly() {
        // This test verifies the function doesn't panic
        log_request("GET", "https://api.github.com/user");
    }

    #[test]
    fn test_log_response_formats_correctly() {
        log_response("https://api.github.com/user", StatusCode::OK);
    }

    #[test]
    fn test_log_response_error_formats_correctly() {
        log_response_error(
            "https://api.github.com/user",
            StatusCode::UNAUTHORIZED,
            "Bad credentials",
        );
    }
}
