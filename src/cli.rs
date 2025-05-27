use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Entrypoint script
    pub entrypoint: PathBuf,

    /// Output file (defaults to stdout if not set)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Strip comments and blank lines
    #[arg(long)]
    pub minify: bool,

    /// Disable inlining, just output the entrypoint as-is
    #[arg(long)]
    pub no_inline: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
