//! Implementation of the `solve` subcommand.

use std::process::{Command, Stdio};

use crate::commands::run::RunMode;
use crate::commands::submit::submit_answer_ext;
use crate::prelude::*;
// use crate::utils::extract::extract_article_text; /* not strictly required here, but *
// useful later */
use crate::utils::input::resolve_input_path;

/// Run the day binary, inspect its output, prompt the user which part to
/// submit and then perform the submission (or a dry run).
///
/// This expects the day binary to print lines that start with:
///   * `Part 1:`
///   * `Part 2:`
pub fn solve_day(year: u32, day: u8, input: &str, mode: RunMode, dry_run: bool) -> Result<()> {
    println!("Solving {year} day {day}...");

    let pkg = format!("aoc{year}");
    let bin_name = format!("d{day:02}");
    let input_path = resolve_input_path(year, day, input)?;

    // Build the day binary.
    let mut build = Command::new("cargo");
    build
        .arg("build")
        .arg("-p")
        .arg(&pkg)
        .arg("--bin")
        .arg(&bin_name);

    if matches!(mode, RunMode::Release) {
        build.arg("--release");
    }

    let status = build.status()?;
    if !status.success() {
        return Err(anyhow!("cargo build failed for {pkg}"));
    }

    let target_bin = if matches!(mode, RunMode::Release) {
        Path::new("target").join("release").join(&bin_name)
    } else {
        Path::new("target").join("debug").join(&bin_name)
    };

    if !target_bin.exists() {
        return Err(anyhow!(
            "expected binary {} does not exist",
            target_bin.display()
        ));
    }

    // Run the binary and capture its stdout.
    let input_file = fs::File::open(&input_path)?;
    let output = Command::new(&target_bin)
        .stdin(Stdio::from(input_file))
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("day binary exited with failure"));
    }

    let text = String::from_utf8_lossy(&output.stdout);
    println!("--- Solver output ---");
    println!("{text}");
    println!("---------------------");

    // Very simple output parsing: look for lines prefixed with "Part 1:" and "Part
    // 2:".
    let mut part1 = None;
    let mut part2 = None;

    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("Part 1:") {
            part1 = Some(rest.trim().to_string());
        }
        if let Some(rest) = line.strip_prefix("Part 2:") {
            part2 = Some(rest.trim().to_string());
        }
    }

    println!("Detected outputs:");
    println!("  Part 1 = {:?}", part1);
    println!("  Part 2 = {:?}", part2);

    println!("Submit which part? Enter 1, 2, or press Enter to skip:");

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let choice = buf.trim();

    let (part, answer_opt) = match choice {
        "1" => (1, part1),
        "2" => (2, part2),
        "" => {
            println!("Skipping submission.");
            return Ok(());
        }
        _ => return Err(anyhow!("Invalid selection: {choice}")),
    };

    let answer = answer_opt.ok_or_else(|| anyhow!("Missing output for part {part}"))?;

    println!("Preparing to submit part {part}: {answer}");

    // Call into the shared submission logic.
    submit_answer_ext(year, day, part, answer, dry_run)
}
