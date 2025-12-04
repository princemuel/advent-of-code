//! Implementation of the `init` subcommand.

use crate::prelude::*;

/// Initialize a new Advent of Code year crate and related directories.
///
/// This creates:
///   * `crates/aoc{year}` with a minimal `Cargo.toml` and `src/lib.rs`,
///   * `crates/aoc{year}/src/bin` for day binaries,
///   * `inputs/{year}` and `answers/{year}` directories.
pub fn init_year(year: u32) -> Result<()> {
    let pkg = format!("aoc{year}");
    let crate_dir = Path::new("crates").join(&pkg);

    println!(
        "Initializing Advent of Code year {year} in {}",
        crate_dir.display()
    );

    fs::create_dir_all(crate_dir.join("src/bin"))?;
    fs::create_dir_all(crate_dir.join("src"))?;
    fs::create_dir_all(Path::new("inputs").join(year.to_string()))?;
    fs::create_dir_all(Path::new("answers").join(year.to_string()))?;

    let cargo_toml = {
        let txt = include_str!("../../templates/cargo.txt");
        txt.replace("{pkg}", &pkg)
    };

    fs::write(crate_dir.join("Cargo.toml"), cargo_toml)?;

    let prelude_rs = {
        let txt = include_str!("../../templates/prelude.txt");
        txt.replace("{pkg}", &pkg)
    };

    let lib_rs = "pub mod prelude;";

    let src_dir = crate_dir.join("src");
    if !src_dir.join("prelude.rs").exists() {
        fs::write(src_dir.join("prelude.rs"), &prelude_rs)?;
    }

    if !src_dir.join("lib.rs").exists() {
        fs::write(src_dir.join("lib.rs"), lib_rs)?;
    }

    // Try to add the year crate to the workspace `Cargo.toml` if possible.
    let workspace = Path::new("Cargo.toml");
    if workspace.exists() {
        // TODO: This section works but fails when members are declared with a glob path
        // e.g crates/*   or crates/**
        let mut contents = fs::read_to_string(workspace)?;
        let member_line = format!(r#""crates/{pkg}""#);
        if !contents.contains(&member_line) {
            if let Some(idx) = contents.find("members = [") {
                let insert_pos = idx + "members = [".len();
                contents.insert_str(insert_pos, &format!("\n    {member_line},"));
                fs::write(workspace, contents)?;
                println!("Added crates/{pkg} to workspace members");
            } else {
                println!(
                    "Could not locate [workspace] members array in Cargo.toml; add \
                     crates/{pkg} manually"
                );
            }
        }
    }

    println!("✓ Year {year} initialized");
    Ok(())
}
