use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Reads the entrypoint Bash file and writes it to output or stdout.
pub fn bundle_script(entrypoint: &Path, output: Option<&Path>) -> Result<()> {
    let contents =
        fs::read_to_string(entrypoint).context(format!("Failed to read file: {entrypoint:?}"))?;

    match output {
        Some(output_path) => {
            fs::write(output_path, contents)
                .context(format!("Failed to write to output file: {output_path:?}"))?;
            println!("Bundled script written to {output_path:?}");
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle
                .write_all(contents.as_bytes())
                .context("Failed to write to stdout")?;
        }
    }

    Ok(())
}
