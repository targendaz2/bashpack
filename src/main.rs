mod bundler;
mod cli;
mod resolver;

use anyhow::Result;

fn main() -> Result<()> {
    let args = cli::parse_args();
    if args.dry_run {
        bundler::dry_run(&args)?;
    } else {
        bundler::bundle_script(&args)?;
    }
    Ok(())
}
