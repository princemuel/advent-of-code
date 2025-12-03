//! Implementation of the `run` and `current` behavior.

use std::process::{Command, Stdio};

use clap::ValueEnum;

use crate::prelude::*;
use crate::utils::input::resolve_input_path;

/// Build profile used by `run` and `solve` commands.
///
/// Debug builds are fast to compile, release builds are optimized.
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum RunMode {
    /// Compile without optimizations for faster iteration.
    Debug,
    /// Compile with optimizations for realistic performance.
    Release,
}

/// Build and run a particular day binary for the given year.
///
/// The binary is expected at `crates/aoc{year}/src/bin/d{day:02}.rs` and is
/// built via `cargo build`.
pub fn run_day(year: u32, day: u8, input: &str, mode: RunMode) -> Result<()> {
    let pkg = format!("aoc{year}");
    let bin_name = format!("d{day:02}");

    let input_path = resolve_input_path(year, day, input)?;

    println!(
        "Running {}::{} ({:?}) with {}",
        pkg,
        bin_name,
        mode,
        input_path.display()
    );

    // Build the day binary in the requested mode.
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
        return Err(anyhow!("cargo build failed for package {pkg}"));
    }

    // Determine the expected output binary path.
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

    // Open the input file and pipe it to the binary stdin.
    let input_file = fs::File::open(&input_path)?;
    let status = Command::new(&target_bin)
        .stdin(Stdio::from(input_file))
        .status()?;

    if !status.success() {
        return Err(anyhow!("day binary exited with status {status}"));
    }

    Ok(())
}
