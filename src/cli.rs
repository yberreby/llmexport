use clap::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Collect and format repository files for LLM consumption"
)]
pub struct Cli {
    #[arg(value_name = "PATHS")]
    pub paths: Vec<PathBuf>,

    #[arg(short, long, default_value_t = 25)]
    pub commits: usize,

    #[arg(short, long, value_delimiter = ',')]
    pub ignore: Vec<String>,

    #[arg(short, long)]
    pub stdout: bool,
}
