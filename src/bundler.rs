use crate::resolver::resolve_script;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Reads the entrypoint Bash file and writes it to output or stdout.
pub fn bundle_script(entrypoint: &Path, output: Option<&Path>) -> Result<()> {
    let bundled = resolve_script(entrypoint)?;

    match output {
        Some(output_path) => {
            fs::write(output_path, bundled)
                .with_context(|| format!("Failed to write to output file: {output_path:?}"))?;
            println!("Bundled script written to {output_path:?}");
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle
                .write_all(bundled.as_bytes())
                .context("Failed to write to stdout")?;
        }
    }

    Ok(())
}
