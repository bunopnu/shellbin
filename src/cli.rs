use clap::command;
use std::path::PathBuf;

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
}
