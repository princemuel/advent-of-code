//! Minimal HTTP helpers for talking to the Advent of Code website.

use reqwest::blocking::Client;

use crate::prelude::*;

/// Create a blocking HTTP client with a custom user agent.
///
/// The user agent string identifies this tool but does not need to be fancy.
pub fn http_client() -> Result<Client> {
    let client = Client::builder()
        .user_agent("aocctl (github.com/your-name/your-repo)")
        .build()?;
    Ok(client)
}

/// Read the Advent of Code session cookie from the environment or from a file.
///
/// This first tries the `SESSION` environment variable, then falls back to
/// several common file locations such as `.session` and `.config/aoc/session`.
pub fn read_session() -> Result<String> {
    // Environment variable wins.
    if let Ok(value) = std::env::var("SESSION") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    // Try a few common file locations.
    let candidates = [
        ".session",
        ".config/aoc/session",
        "config/.session",
        "aoc.session",
    ];

    for path in candidates.iter().map(Path::new) {
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            let trimmed = contents.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.to_string());
            }
        }
    }

    Err(anyhow!(
        "No AoC session token found. Set SESSION or create a `.session` file in the project \
         root."
    ))
}
