//! Implementations for individual `aocctl` subcommands.

pub mod init;
pub mod input;
pub mod new;
pub mod open;
pub mod run;
pub mod solve;
pub mod status;
pub mod submit;

// Re-export the run mode enum so the CLI can refer to it as
// `commands::RunMode`.
pub use new::TemplateKind;
pub use run::RunMode;
