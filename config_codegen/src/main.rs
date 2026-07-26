mod excel_confgen;

use anyhow::Context;
use std::{env, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("config_codegen must be inside the workspace")?
        .to_owned();
    let mut args = env::args_os().skip(1);
    let input = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace.join("data/excel2json"));
    let output = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace.join("config/configs"));

    excel_confgen::generate_rust_modules(&input.to_string_lossy(), &output.to_string_lossy())?;
    println!("Generated Rust config modules in {}", output.display());
    Ok(())
}
