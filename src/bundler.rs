use crate::{cli::Cli, resolver::resolve_script};
use anyhow::{Context, Result};
use std::{
    fs,
    io::{self, Write},
};

/// Reads the entrypoint Bash file and writes it to output or stdout.
pub fn bundle_script(args: &Cli) -> Result<()> {
    let bundled = if args.no_inline {
        fs::read_to_string(&args.entrypoint)
            .with_context(|| format!("Failed to read file: {}", args.entrypoint.display()))?
    } else {
        if args.verbose {
            eprintln!("Resolving and bundling {}", args.entrypoint.display());
        }
        let content = resolve_script(&args.entrypoint)?;
        if args.minify {
            let mut lines = content.lines();
            let mut result = Vec::new();

            // Preserve shebang if present
            if let Some(first_line) = lines.next() {
                if first_line.trim_start().starts_with("#!") {
                    result.push(first_line);
                } else if !first_line.trim().is_empty() && !first_line.trim().starts_with('#') {
                    result.push(first_line);
                }

                // Filter remaining lines
                for line in lines {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() && !trimmed.starts_with('#') {
                        result.push(trimmed);
                    }
                }
            }

            result.join("\n")
        } else {
            content
        }
    };

    match &args.output {
        Some(output_path) => {
            fs::write(output_path, bundled).with_context(|| {
                format!("Failed to write to output file: {}", output_path.display())
            })?;
            println!("Bundled script written to {}", output_path.display());
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
