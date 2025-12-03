//! Implementation of the `new` subcommand.

use crate::prelude::*;

/// The code template used when generating a new day's source file.
///
/// Each variant provides a different balance of simplicity, performance,
/// and memory usage. Templates are named in lowercase for CLI use (e.g.
/// `--template minimal`).
#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub enum TemplateKind {
    /// Minimal template:
    /// - smallest amount of boilerplate,
    /// - stdin-only,
    /// - collects into Vec<String>,
    /// - perfect for simple puzzles.
    Minimal,

    /// Buffered template:
    /// - supports both stdin and file paths,
    /// - uses `BufReader` for efficiency,
    /// - still collects input to Vec<String>,
    /// - best for puzzles with moderately large inputs.
    Buffered,

    /// Streaming template:
    /// - line-by-line processing with no full allocation,
    /// - returns iterators instead of Vec,
    /// - suited for huge inputs or state-machine puzzles.
    Streaming,

    /// Fast template:
    /// - zero-copy parsing using &str slices,
    /// - no heap allocations for lines,
    /// - best for performance-heavy days where input structure is simple.
    Fast,
}

/// Create a new day binary source file for the given year and day.
///
/// The generated file is placed at: `crates/aoc{year}/src/bin/d{day:02}.rs`.
pub fn new_day(year: u32, day: u8, template: TemplateKind) -> Result<()> {
    let pkg = format!("aoc{year}");
    let crate_dir = Path::new("crates").join(&pkg);
    let bin_dir = crate_dir.join("src/bin");

    fs::create_dir_all(&bin_dir)?;

    let path = bin_dir.join(format!("d{day:02}.rs"));
    if path.exists() {
        return Err(anyhow!("Day {day:02} already exists at {}", path.display()));
    }

    let template = match template {
        TemplateKind::Minimal => TEMPLATE_MINIMAL,
        TemplateKind::Buffered => TEMPLATE_BUFFERED,
        TemplateKind::Streaming => TEMPLATE_STREAMING,
        TemplateKind::Fast => TEMPLATE_FAST,
    };

    let contents = template.replace("{YEAR}", &year.to_string());
    fs::write(&path, contents)?;

    println!("✓ Created {}", path.display());
    Ok(())
}

pub fn list_templates() {
    println!();
    println!("Available templates:");
    println!("---------------------");

    let table = [
        (
            "minimal",
            "Smallest boilerplate; stdin only; Vec<String> parser.",
        ),
        (
            "buffered",
            "Supports file paths; uses BufReader; flexible and reliable.",
        ),
        (
            "streaming",
            "Processes input lazily; no full allocation; ideal for huge input.",
        ),
        (
            "fast",
            "Zero-copy parsing (&str slices); fastest runtime template.",
        ),
    ];

    for (name, desc) in table {
        println!("  {:<10} {}", name, desc);
    }

    println!();
    println!("Run:  aocctl new <day> --template <name>");
    println!("Example: aocctl new 7 --template fast");
    println!();
}
