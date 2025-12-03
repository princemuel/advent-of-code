//! Basic integration tests for the `aocctl` CLI surface.
//!
//! These tests focus on argument parsing and do not hit the network.

use aocctl::{Cli, Command};
use clap::Parser;

#[test]
fn parse_init_command() {
    let cli = Cli::parse_from(["aocctl", "init", "2025"]);
    match cli.command {
        Command::Init { year } => assert_eq!(year, 2025),
        other => panic!("Expected Init, got {other:?}"),
    }
}

#[test]
fn parse_run_with_year_flag() {
    let cli = Cli::parse_from(["aocctl", "--year", "2025", "run", "3"]);
    match cli.command {
        Command::Run { day, input, .. } => {
            assert_eq!(day, 3);
            assert_eq!(input, "puzzle");
        }
        other => panic!("Expected Run, got {other:?}"),
    }
}

#[test]
fn parse_submit_with_dry_run() {
    let cli = Cli::parse_from([
        "aocctl",
        "--year",
        "2025",
        "submit",
        "4",
        "--part",
        "1",
        "--dry-run",
        "12345",
    ]);

    match cli.command {
        Command::Submit {
            day,
            part,
            answer,
            dry_run,
        } => {
            assert_eq!(day, 4);
            assert_eq!(part, 1);
            assert!(dry_run);
            assert_eq!(answer.as_deref(), Some("12345"));
        }
        other => panic!("Expected Submit, got {other:?}"),
    }
}

#[test]
fn parse_solve_defaults() {
    let cli = Cli::parse_from(["aocctl", "--year", "2025", "solve", "5"]);
    match cli.command {
        Command::Solve {
            day,
            ref input,
            dry_run,
            ..
        } => {
            assert_eq!(day, 5);
            assert_eq!(input, "puzzle");
            assert!(!dry_run);
        }
        other => panic!("Expected Solve, got {other:?}"),
    }
}
