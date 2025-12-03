//! Implementation of the `submit` subcommand.

use std::fs::OpenOptions;

use crate::prelude::*;
use crate::utils::{extract, http};

/// Submit an answer for a particular day and part.
///
/// If `answer_arg` is `None`, this function will prompt on standard input.
/// If `dry_run` is true, this prints what would be sent and returns without
/// contacting the Advent of Code server.
pub fn submit_answer(
    year: u32,
    day: u8,
    part: u8,
    answer_arg: Option<String>,
    dry_run: bool,
) -> Result<()> {
    if part != 1 && part != 2 {
        return Err(anyhow!("Part must be 1 or 2, got {part}"));
    }

    let answer = match answer_arg {
        Some(a) => a,
        None => {
            println!("Enter answer for part {part}, then press Enter:");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            buf.trim().to_string()
        }
    };

    if answer.is_empty() {
        return Err(anyhow!("Cannot submit an empty answer"));
    }

    submit_answer_ext(year, day, part, answer, dry_run)
}

/// Internal helper used by both `submit` and `solve`.
///
/// This always uses the provided `answer` string and does not prompt for input.
pub fn submit_answer_ext(
    year: u32,
    day: u8,
    part: u8,
    answer: String,
    dry_run: bool,
) -> Result<()> {
    use std::io::Write as _;

    if dry_run {
        println!("[dry-run] Would submit:");
        println!("  year   = {year}");
        println!("  day    = {day}");
        println!("  part   = {part}");
        println!("  answer = {answer}");
        return Ok(());
    }

    let session = http::read_session()?;
    let client = http::http_client()?;
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");

    let body = serde_urlencoded::to_string(&[
        ("level", part.to_string()),
        ("answer", answer.clone()),
    ])?;

    println!("Submitting answer for {year} day {day} part {part}...");

    let resp = client
        .post(&url)
        .header(reqwest::header::COOKIE, format!("session={session}"))
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(body)
        .send()?;

    let status = resp.status();
    let text = resp.text()?;

    if !status.is_success() {
        println!("HTTP {} while submitting answer", status.as_u16());
        println!("{text}");
        return Ok(());
    }

    if let Some(msg) = extract::extract_article_text(&text) {
        println!("{msg}");
    } else {
        println!("{text}");
    }

    // Append an entry to a per-day answers log for your own notes.
    let answers_dir = Path::new("answers").join(year.to_string());
    fs::create_dir_all(&answers_dir)?;
    let file = answers_dir.join(format!("d{day:02}.txt"));

    let mut f = OpenOptions::new().create(true).append(true).open(&file)?;
    writeln!(f, "part {part}: {answer}")?;

    println!("Recorded submission in {}", file.display());
    Ok(())
}
