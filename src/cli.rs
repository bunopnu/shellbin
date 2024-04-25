use clap::command;
use std::{path::PathBuf, str::FromStr};

pub use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Sets the shell script to assembly.
    #[arg(short, long)]
    pub script: Option<String>,

    /// Sets the input file containing the shell script.
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Sets the output file path.
    #[arg(short, long, default_value = "output.exe")]
    pub output: PathBuf,

    /// Sets the target platform.
    #[arg(short, long)]
    pub target: Targets,
}

/// Represents supported target platforms.
#[derive(Clone, Copy)]
pub enum Targets {
    // Windows targets:
    Windows,
    // Linux targets:
    LinuxAMD64,
    LinuxI386,
}

impl FromStr for Targets {
    type Err = crate::result::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "windows" => Ok(Self::Windows),
            "linux-amd64" => Ok(Self::LinuxAMD64),
            "linux-i386" => Ok(Self::LinuxI386),
            _ => Err(crate::result::Error::UnknownPlatform),
        }
    }
}
