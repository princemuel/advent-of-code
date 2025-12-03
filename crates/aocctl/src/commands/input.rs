//! Implementation of the `input` subcommand (download puzzle input).

use crate::prelude::*;
use crate::utils::http;

/// Download the puzzle input for the given year and day and save it under
/// `inputs/{year}/d{day:02}.txt`.
pub fn download_input(year: u32, day: u8) -> Result<()> {
    let session = http::read_session()?;
    let client = http::http_client()?;
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    println!("Downloading input for {year} day {day}...");

    let resp = client
        .get(&url)
        .header(reqwest::header::COOKIE, format!("session={session}"))
        .send()?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "HTTP {} while downloading input",
            resp.status().as_u16()
        ));
    }

    let body = resp.text()?;
    let trimmed = body.trim_end_matches('\n');

    let dir = Path::new("inputs").join(year.to_string());
    fs::create_dir_all(&dir)?;

    let path = dir.join(format!("d{day:02}.txt"));
    fs::write(&path, trimmed)?;

    let line_count = trimmed.lines().count();
    println!("✓ Saved to {}", path.display());
    println!("  Lines: {line_count}");

    for (i, line) in trimmed.lines().take(3).enumerate() {
        println!("  {}: {}", i + 1, line);
    }
    if line_count > 3 {
        println!("  ...");
    }

    Ok(())
}
