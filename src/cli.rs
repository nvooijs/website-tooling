use crate::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Directory to run in
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// Path to the toml configuration file
    #[arg(short, long)]
    pub config: PathBuf,
}

pub fn parse() -> Cli {
    Cli::parse()
}
