mod bundler;
mod cli;
mod resolver;

use anyhow::Result;

fn main() -> Result<()> {
    let args = cli::parse_args();
    bundler::bundle_script(&args)?;
    Ok(())
}
