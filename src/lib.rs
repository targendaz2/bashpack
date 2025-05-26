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
        .with_context(|| format!("Failed to open input file: {:?}", input_file))?;
    let reader = BufReader::new(file);

    // Read the file line by line looking for source lines
    let re = Regex::new(r#"^\s*source\s+("|')?(?<filename>.+)("|')?\s*$"#)
        .with_context(|| "Failed to compile regex")?;

    for (index, line) in reader.lines().enumerate() {
        let index = index + 1;
        let line = line
            .with_context(|| format!("Failed to read line {index} from file: {:?}", input_file))?;

        let Some(result) = re.captures(&line) else {
            continue;
        };

        println!("Found source \"{}\" on line {index}", &result["filename"]);
    }

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
