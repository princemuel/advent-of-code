//! The binary entry point for the `aocctl` command line tool.
use aocctl::Cli;

fn main() -> anyhow::Result<()> { Cli::execute() }
