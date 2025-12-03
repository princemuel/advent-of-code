//! Convenience prelude for internal modules.
//!
//! Command and utility modules can `use crate::prelude::*;` to pull in common
//! types and imports and avoid long `use` lists.

pub use std::path::{Path, PathBuf};
pub use std::{fs, io};

pub use anyhow::{Result, anyhow};

pub use crate::templates::*;
