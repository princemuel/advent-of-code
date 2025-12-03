//! Implementation of the `status` subcommand.

use crate::prelude::*;
use crate::utils::http;

/// Fetch the puzzle page and count how many stars have been earned for the
/// given year and day.
///
/// This parses the HTML and counts `class="star"` occurrences, which is simple
/// but works well enough for this purpose.
pub fn check_status(year: u32, day: u8) -> Result<()> {
    let client = http::http_client()?;
    let url = format!("https://adventofcode.com/{year}/day/{day}");

    println!("Fetching status from {url}");

    let body = client.get(&url).send()?.text()?;
    let stars = body.matches("class=\"star\"").count();

    println!("Stars for {year} day {day}: {stars}");
    match stars {
        0 => println!("No parts completed yet"),
        1 => println!("Part 1 complete"),
        _ => println!("Both parts complete"),
    }

    Ok(())
}
