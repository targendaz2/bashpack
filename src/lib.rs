use anyhow::{Context, Result, bail};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

/// Opens a file at the given path and returns a buffered reader.
fn open_file(path: &Path) -> Result<BufReader<File>> {
    if !path.exists() {
        bail!("File does not exist: {path:?}");
    }

    let file = File::open(path).context(format!("Failed to open input file: {path:?}"))?;

    let reader = BufReader::new(file);
    Ok(reader)
}

/// Gets the parent directory of the given path.
fn get_parent_path(path: &Path) -> Result<PathBuf> {
    let path = path.parent().context("File has no parent directory")?;
    Ok(path.to_path_buf())
}

pub fn run(input_file: &Path, output_file: &Path) -> Result<()> {
    // Attempt to open and read the input file, fail if we can't
    let reader = open_file(input_file)?;

    // Get the absolute path of the input file's parent directory
    let input_file_parent_path = get_parent_path(input_file)?;

    // Initialize the output file content
    let mut output_content = String::new();

    // Read the file line by line looking for source lines
    let re = Regex::new(r#"^\s*source\s+("|')?(?<filename>.+)("|')?\s*$"#)
        .context("Failed to compile regex")?;

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
        let sourced_reader = open_file(&sourced_file_path)?;

        // Write the sourced file content to the output content
        for sourced_line in sourced_reader.lines() {
            let sourced_line = sourced_line.with_context(|| {
                format!("Failed to read line from sourced file: {sourced_file_path:?}")
            })?;
            output_content.push_str(&sourced_line);
            output_content.push('\n');
        }
    }

    // Write the output content to the output file
    let output_file_path = output_file.to_path_buf();
    let output_file_handle = File::create(&output_file_path).context(format!(
        "Failed to create output file: {output_file_path:?}"
    ))?;
    let mut output_writer = BufWriter::new(output_file_handle);
    output_writer
        .write_all(output_content.as_bytes())
        .context(format!(
            "Failed to write to output file: {output_file_path:?}"
        ))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod open_file {
        use super::*;

        use tempfile::NamedTempFile;

        #[test]
        fn returns_a_reader_if_file_exists() {
            let mut tmpfile = NamedTempFile::new().unwrap();
            write!(tmpfile, "echo 'test'").unwrap();
            let reader = open_file(tmpfile.path()).unwrap();
            assert!(reader.lines().next().is_some());
        }

        #[test]
        fn returns_an_error_if_file_doesnt_exist() {
            let non_existent_path = Path::new("non_existent_file.txt");
            let result = open_file(non_existent_path);
            assert!(result.is_err());
        }
    }
}
