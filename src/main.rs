use std::io::{Read, Write};

use assembler::Assembler;
use cli::Parser;

/// Represents the maximum length of a shell command.
pub const SCRIPT_MAXIMUM_LENGTH: usize = 7168;

/// This module contains functionality related to generating executable binary.
mod assembler;
/// This module contains the definition of CLI options.
mod cli;
/// This module contains custom error types for the application.
mod result;

fn main() -> anyhow::Result<()> {
    let opts = cli::Args::parse();

    // Read script to assemble
    let script = match opts.script {
        Some(script) => script,
        None => {
            // If script argument is not provided, attempt to read from input file
            let script_from_file = read_script_from_file(&opts.file)?;
            script_from_file.ok_or(result::Error::NotFound)?
        }
    };

    // Check script length
    if script.len() > SCRIPT_MAXIMUM_LENGTH {
        return Err(crate::result::Error::TooLong.into());
    }

    // Assemble script into a binary executable based on the specified target platform
    let binary = assemble_binary(opts.target, script)?;

    // Write binary to the output file
    write_binary_to_file(&opts.output, &binary)?;

    Ok(())
}

/// Runs the assembler.
fn assemble_binary(target: cli::Targets, script: String) -> anyhow::Result<Vec<u8>> {
    let binary = match target {
        cli::Targets::Windows => assembler::windows::WindowsAssembler::assemble(script),
        cli::Targets::LinuxAMD64 => assembler::linux::amd64::LinuxAssembler::assemble(script),
        cli::Targets::LinuxI386 => assembler::linux::i386::LinuxAssembler::assemble(script),
    }?;

    Ok(binary)
}

/// Writes binary to a file.
fn write_binary_to_file(output_path: &std::path::PathBuf, binary: &Vec<u8>) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(output_path)?;
    file.write_all(binary)?;

    println!(
        "Successfully wrote {} bytes to: {}",
        binary.len(),
        output_path.display()
    );

    Ok(())
}

/// Reads the shell script from a file.
fn read_script_from_file(file_path: &Option<std::path::PathBuf>) -> anyhow::Result<Option<String>> {
    match file_path {
        Some(path) => {
            let mut file = std::fs::File::open(path)?;

            let mut script = String::new();
            file.read_to_string(&mut script)?;

            Ok(Some(script))
        }
        None => Ok(None),
    }
}
