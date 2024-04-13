use std::io::{Read, Write};

use anyhow::Context;
use assembler::{windows::WindowsAssembler, Assembler};
use cli::Parser;

/// This module contains functionality related to generating executable binary.
mod assembler;
/// This module contains the definition of CLI options.
mod cli;
/// This module contains custom error types for the application.
mod result;

fn main() -> anyhow::Result<()> {
    let opts = cli::Args::parse();

    let script = match opts.script {
        Some(script) => script,
        None => {
            // If script argument is not provided, attempt to read from input file
            let script_from_file = read_script_from_file(&opts.file)?;
            script_from_file.ok_or(result::Error::NotFound)?
        }
    };

    let binary = WindowsAssembler::assemble(script)?;

    // Write binary to the output file
    let mut file = std::fs::File::create(&opts.output)?;
    file.write_all(&binary)?;

    println!("Wrote {} bytes to: {}", binary.len(), opts.output.display());

    Ok(())
}

/// Reads the shell script from a file.
fn read_script_from_file(file_path: &Option<std::path::PathBuf>) -> anyhow::Result<Option<String>> {
    match file_path {
        Some(path) => {
            let mut file = std::fs::File::open(path).context("Failed to open input file")?;
            let mut script = String::new();
            file.read_to_string(&mut script)
                .context("Failed to read from input file")?;

            Ok(Some(script))
        }
        None => Ok(None),
    }
}
