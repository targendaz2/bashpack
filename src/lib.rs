mod fs;

use anyhow::{Context, Result};
use regex::Regex;
use std::{io::BufRead, path::Path};

use crate::fs::{get_parent_path, read_file, read_file_to_string, write_file};

/// Processes a single line of the input file, looking for a source directive and replacing it with the content of the sourced file.
fn process_line(line: &str, working_directory: &Path, re: &Regex) -> Result<String> {
    let Some(result) = re.captures(line) else {
        return Ok(format!("{line}\n"));
    };

    let sourced_file_path = working_directory.join(&result["filename"]);
    let output_content = read_file_to_string(&sourced_file_path).context(format!(
        "Failed to read sourced file: {sourced_file_path:?}"
    ))?;
    Ok(output_content)
}

/// Processes the lines of the input file, looking for source lines and replacing them with their content.
fn process_lines<R: BufRead>(reader: R, input_file: &Path, re: &Regex) -> Result<String> {
    let mut output_content = String::new();
    let input_file_parent_path = get_parent_path(input_file)?;

    for (index, line) in reader.lines().enumerate() {
        let index = index + 1;
        let line = line.context(format!(
            "Failed to read line {index} from file: {input_file:?}"
        ))?;

        let processed_line = process_line(&line, &input_file_parent_path, re).context(format!(
            "Failed to process line {index} from file: {input_file:?}"
        ))?;
        output_content.push_str(&processed_line);
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
