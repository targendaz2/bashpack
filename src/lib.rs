mod fs;

use anyhow::{Context, Result};
use regex::Regex;
use std::{io::BufRead, path::Path};

use crate::fs::{get_parent_path, read_file, write_file};

/// Processes the lines of the input file, looking for source lines and replacing them with their content.
fn process_lines<R: BufRead>(reader: R, input_file: &Path, re: &Regex) -> Result<String> {
    let mut output_content = String::new();
    let input_file_parent_path = get_parent_path(input_file)?;

    for (index, line) in reader.lines().enumerate() {
        let index = index + 1;
        let line = line.context(format!(
            "Failed to read line {index} from file: {input_file:?}"
        ))?;

        // Parse the line, keep it as is if it's not a source line
        let Some(result) = re.captures(&line) else {
            output_content.push_str(&line);
            output_content.push('\n');
            continue;
        };

        println!("Found source \"{}\" on line {index}", &result["filename"]);

        // Check if the source file exists, bail if not
        let sourced_file_path =
            Path::new(&input_file_parent_path.to_str().unwrap()).join(&result["filename"]);
        let sourced_reader = read_file(&sourced_file_path)?;

        // Write the sourced file content to the output content
        for sourced_line in sourced_reader.lines() {
            let sourced_line = sourced_line.with_context(|| {
                format!("Failed to read line from sourced file: {sourced_file_path:?}")
            })?;
            output_content.push_str(&sourced_line);
            output_content.push('\n');
        }
    }

    Ok(output_content)
}

pub fn run(input_file: &Path, output_file: &Path) -> Result<()> {
    // Attempt to open and read the input file, fail if we can't
    let reader = read_file(input_file)?;

    // Read the file line by line looking for source lines
    let re = Regex::new(r#"^\s*source\s+("|')?(?<filename>.+)("|')?\s*$"#)
        .context("Failed to compile regex")?;

    let output_content = process_lines(reader, input_file, &re)?;

    // Write the output content to the output file
    write_file(output_file, &output_content)?;

    Ok(())
}
