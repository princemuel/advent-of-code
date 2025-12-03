//! Helpers for resolving input file paths.

use crate::prelude::*;

/// Resolve an input path for the given year and day.
///
/// If `input` is the literal word `"puzzle"`, this function looks for:
///   * `inputs/{year}/d{day:02}.txt`
///   * `inputs/{year}/input.txt`
///
/// If `input` is anything else, it is treated as a filesystem path and must
/// exist.
pub fn resolve_input_path(year: u32, day: u8, input: impl AsRef<str>) -> Result<PathBuf> {
    let input = input.as_ref();
    if input == "puzzle" {
        let specific = Path::new("inputs")
            .join(year.to_string())
            .join(format!("d{day:02}.txt"));
        let generic = Path::new("inputs").join(year.to_string()).join("input.txt");

        if specific.exists() {
            return Ok(specific);
        }
        if generic.exists() {
            return Ok(generic);
        }

        return Err(anyhow!(
            "No puzzle input found for year {year} day {day}. Expected {} or {}",
            specific.display(),
            generic.display()
        ));
    }

    let path = PathBuf::from(input);
    if !path.exists() {
        return Err(anyhow!("Input file does not exist: {}", path.display()));
    }
    Ok(path)
}
