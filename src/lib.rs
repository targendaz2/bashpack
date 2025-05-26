use anyhow::{Context, Result, bail};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn run(input_file: &Path, _output_file: &Path) -> Result<()> {
    // Check if the input file exists, bail if not
    if !input_file.exists() {
        bail!("Input file does not exist");
    }

    // Attempt to open and read the input file, fail if we can't
    let file = File::open(input_file)
        .with_context(|| format!("Failed to open input file: {input_file:?}"))?;
    let reader = BufReader::new(file);

    // Get the absolute path of the input file's parent directory
    let input_file_parent_path = input_file
        .parent()
        .context("Input file has no parent directory")?
        .to_path_buf();

    // Initialize the output file content
    let mut output_content = String::new();

    // Read the file line by line looking for source lines
    let re = Regex::new(r#"^\s*source\s+("|')?(?<filename>.+)("|')?\s*$"#)
        .with_context(|| "Failed to compile regex")?;

    for (index, line) in reader.lines().enumerate() {
        let index = index + 1;
        let line =
            line.with_context(|| format!("Failed to read line {index} from file: {input_file:?}"))?;

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
        if !sourced_file_path.exists() {
            bail!("Source file {sourced_file_path:?} on line {index} does not exist");
        }

        // Attempt to read the sourced file, fail if we can't
        let sourced_file = File::open(&sourced_file_path)
            .with_context(|| format!("Failed to open sourced file: {sourced_file_path:?}"))?;
        let sourced_reader = BufReader::new(sourced_file);

        // Write the sourced file content to the output content
        for sourced_line in sourced_reader.lines() {
            let sourced_line = sourced_line.with_context(|| {
                format!("Failed to read line from sourced file: {sourced_file_path:?}")
            })?;
            output_content.push_str(&sourced_line);
            output_content.push('\n');
        }
    }

    println!("{output_content}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doesnt_throw_if_input_file_exists() {
        let input_file = Path::new("Cargo.toml");
        let output_file = Path::new("output.sh");
        let result = run(input_file, output_file);
        assert!(result.is_ok(), "Expected no error when input file exists");
    }

    #[test]
    fn throws_if_input_file_doesnt_exist() {
        let input_file = Path::new("non_existent_file.sh");
        let output_file = Path::new("output.sh");
        let result = run(input_file, output_file);
        assert!(
            result.is_err(),
            "Expected an error when input file does not exist"
        );
    }
}
