use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Entrypoint script
    pub entrypoint: PathBuf,

    /// Output file (defaults to stdout if not set)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
