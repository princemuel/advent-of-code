//! Implementation of the `open` subcommand.

use std::process::{Command, Stdio};

use crate::prelude::*;

/// Open the Advent of Code puzzle page in the default browser.
///
/// This tries `xdg-open` (Linux) and `open` (macOS). On failure, it prints
/// the URL so that the user can open it manually.
pub fn open_day(year: u32, day: u8) -> Result<()> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    println!("Opening {url}");

    if try_open("xdg-open", &url)? || try_open("open", &url)? {
        return Ok(());
    }

    println!("Could not open a browser. Visit this URL manually:\n{url}");
    Ok(())
}

fn try_open(cmd: &str, url: &str) -> Result<bool> {
    let status_result = Command::new(cmd)
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    let ok = matches!(status_result, Ok(s) if s.success());
    Ok(ok)
}
