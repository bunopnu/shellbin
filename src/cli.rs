use clap::command;
use std::{path::PathBuf, str::FromStr};

pub use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Sets the shell script to assembly.
    #[clap(short, long)]
    pub script: Option<String>,

    /// Sets the input file containing the shell script.
    #[clap(short, long)]
    pub file: Option<PathBuf>,

    /// Sets the output file path.
    #[clap(short, long, default_value = "output.exe")]
    pub output: PathBuf,

    /// Sets the target platform.
    #[clap(short, long)]
    pub target: Targets,
}

/// Represents supported target platforms.
#[derive(Clone, Copy)]
pub enum Targets {
    Windows,
    LinuxAMD64,
    LinuxX86,
}

impl FromStr for Targets {
    type Err = crate::result::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "windows" => Ok(Targets::Windows),
            "linux-amd64" => Ok(Targets::LinuxAMD64),
            "linux-x86" => Ok(Targets::LinuxX86),
            _ => Err(crate::result::Error::UnknownPlatform),
        }
    }
}
