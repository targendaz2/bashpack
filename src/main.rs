use anyhow::{Context, Result};
use std::path::PathBuf;

use clap::Parser;

use bashpack::run;

#[derive(Parser)]
struct Cli {
    /// Input script to process
    input_file: PathBuf,

    /// Output file to write the processed script
    output_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(&args.input_file, &args.output_file).with_context(|| "Failed to run bashpack")?;

    Ok(())
}
