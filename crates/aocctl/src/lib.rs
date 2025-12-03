//! Library crate for the `aocctl` Advent of Code helper.
//!
//! This module exposes the [`Cli`] type which powers the binary entry point,
//! along with submodules that implement individual commands and utilities.
mod commands;
mod prelude;
mod templates;
mod utils;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};

/// Top level command line interface for the `aocctl` tool.
#[derive(Debug, Clone, Parser)]
#[command(name = "aocctl", version, about = "Advent of Code workspace helper",long_about = None)]
pub struct Cli {
    /// The year, for example 2025. Auto detected if omitted.
    #[arg(long, global = true)]
    year: Option<u32>,

    /// Concrete command the user wants to run.
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn execute() -> Result<()> {
        let cli = Self::parse();

        // Resolve year up front for all commands that need it.
        // `Init` uses its own explicit year.
        match cli.command {
            Command::Init { year } => commands::init::init_year(year),
            Command::New { day, template } => {
                let year = utils::detect::resolve_year(cli.year)?;
                commands::new::new_day(year, day, template)
            }
            Command::Open { day } => {
                let year = utils::detect::resolve_year(cli.year)?;
                commands::open::open_day(year, day)
            }
            Command::Input { day } => {
                let year = utils::detect::resolve_year(cli.year)?;
                commands::input::download_input(year, day)
            }
            Command::Run { day, input, mode } => {
                let year = utils::detect::resolve_year(cli.year)?;
                commands::run::run_day(year, day, &input, mode)
            }
            Command::Current { input, mode } => {
                let year = utils::detect::resolve_year(cli.year)?;
                let day = utils::detect::detect_latest_day(year)?
                    .ok_or_else(|| anyhow!("No days found for year {year}"))?;
                commands::run::run_day(year, day, &input, mode)
            }
            Command::Submit {
                day,
                part,
                answer,
                dry_run,
            } => {
                let year = utils::detect::resolve_year(cli.year)?;
                commands::submit::submit_answer(year, day, part, answer, dry_run)
            }
            Command::Solve {
                day,
                input,
                mode,
                dry_run,
            } => {
                let year = utils::detect::resolve_year(cli.year)?;
                commands::solve::solve_day(year, day, &input, mode, dry_run)
            }
            Command::Status { day } => {
                let year = utils::detect::resolve_year(cli.year)?;
                commands::status::check_status(year, day)
            }
            Command::ListTemplates => {
                commands::new::list_templates();
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Initialize a a new Advent of Code year.
    Init {
        /// Year to initialize, for example `2025`.
        year: u32,
    },

    /// Create a new day binary source file for the current year using one of
    /// the available templates.
    ///
    /// Templates differ in performance characteristics, memory usage,
    /// and intended workflow style.
    ///
    /// Use `aocctl list-templates` to see a readable table of all templates.
    ///
    /// Examples:
    ///
    /// ```bash
    /// aocctl new 3 --template minimal
    /// aocctl new 4 --template buffered
    /// aocctl new 5 --template streaming
    /// aocctl new 6 --template fast
    New {
        /// Day index in the range 1-25.
        day:      u8,
        /// Selects which template is used when generating the new day's source
        /// file.
        #[arg(long, value_enum, default_value_t = commands::TemplateKind::Minimal)]
        template: commands::TemplateKind,
    },

    /// Open the puzzle page in your browser
    Open {
        /// Day index in the range 1-25.
        day: u8,
    },

    /// Download puzzle input for a day
    Input {
        /// Day index in the range 1-25.
        day: u8,
    },

    /// Build and run a specific day.
    Run {
        /// Day index in the range 1-25.
        day: u8,

        /// Input file path or the literal word `puzzle`.
        #[arg(default_value = "puzzle")]
        input: String,

        /// Build mode: debug or release.
        #[arg(long, value_enum, default_value_t = commands::RunMode::Release)]
        mode: commands::RunMode,
    },

    /// Run the latest day for the current year
    Current {
        /// Input file path or the literal word `puzzle`.
        #[arg(default_value = "puzzle")]
        input: String,

        /// Build mode: debug or release.
        #[arg(long, value_enum, default_value_t = commands::RunMode::Release)]
        mode: commands::RunMode,
    },

    /// Submit an answer
    Submit {
        /// Day index in the range 1-25.
        day: u8,

        /// Part index, must be 1 or 2.
        #[arg(long)]
        part: u8,

        /// Answer to submit. If omitted, read from standard input.
        answer: Option<String>,

        /// Print what would be submitted without contacting the AoC server.
        #[arg(long)]
        dry_run: bool,
    },

    /// Show how many stars you have for a particular day.
    Status {
        /// Day index in the range 1-25.
        day: u8,
    },
    /// Run the solver and interactively submit an answer.
    ///
    /// This command:
    ///   * builds and runs the day binary,
    ///   * parses its output for `Part 1:` and `Part 2:` lines,
    ///   * prompts the user which part to submit,
    ///   * and then calls the submission logic.
    Solve {
        /// Day index in the range 1-25.
        day: u8,

        /// Input path or the literal word `puzzle`.
        #[arg(default_value = "puzzle")]
        input: String,

        /// Build mode: debug or release.
        #[arg(long, value_enum, default_value_t = commands::RunMode::Release)]
        mode:    commands::RunMode,
        /// Print what would be submitted without contacting the AoC server.
        #[arg(long)]
        dry_run: bool,
    },

    /// List all available templates and their descriptions.
    ListTemplates,
}
