//! Helpers to detect years and days based on the workspace layout.

use crate::prelude::*;

/// Resolve the year to operate on given an optional `--year` flag.
///
/// If the flag is absent, this function looks for `crates/aocYYYY`
/// directories and returns the largest year found.
pub fn resolve_year(global: Option<u32>) -> Result<u32> {
    if let Some(y) = global {
        return Ok(y);
    }

    detect_latest_year()?.ok_or_else(|| {
        anyhow!("No crates/aocYYYY projects found; run `aocctl init <year>` first")
    })
}

/// Find the latest `aoc` year based on the `crates` directory contents.
///
/// This looks for directories with names of the form `aocYYYY` and returns
/// the maximum year, or `None` if none exist.
pub fn detect_latest_year() -> Result<Option<u32>> {
    let crates_dir = Path::new("crates");
    if !crates_dir.exists() {
        return Ok(None);
    }

    let max_year = fs::read_dir(crates_dir)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            name.strip_prefix("aoc").map(str::to_string)
        })
        .filter_map(|suffix| suffix.parse::<u32>().ok())
        .max();

    Ok(max_year)
}

/// Find the latest day number for a given year.
///
/// This scans `crates/aoc{year}/src/bin` for files named `dNN.rs` and returns
/// the maximum day value present, or `None` if no such files exist.
pub fn detect_latest_day(year: u32) -> Result<Option<u8>> {
    let pkg = format!("aoc{year}");
    let bin_dir = Path::new("crates").join(&pkg).join("src/bin");

    if !bin_dir.exists() {
        return Ok(None);
    }

    let max_day = fs::read_dir(bin_dir)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            name.strip_prefix('d').map(str::to_string)
        })
        .filter_map(|name| name.strip_suffix(".rs").map(str::to_string))
        .filter_map(|num| num.parse::<u8>().ok())
        .max();

    Ok(max_day)
}
