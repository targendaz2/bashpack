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

fn write_to_file(path: &Path, content: &str) -> Result<()> {
    let path = path.to_path_buf();
    let path_handle =
        File::create(&path).context(format!("Failed to create output file: {path:?}"))?;

    let mut writer = BufWriter::new(path_handle);
    writer
        .write_all(content.as_bytes())
        .context(format!("Failed to write to output file: {path:?}"))?;

    Ok(())
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

    Ok(output_content)
}

pub fn run(input_file: &Path, output_file: &Path) -> Result<()> {
    // Attempt to open and read the input file, fail if we can't
    let reader = open_file(input_file)?;

    // Read the file line by line looking for source lines
    let re = Regex::new(r#"^\s*source\s+("|')?(?<filename>.+)("|')?\s*$"#)
        .context("Failed to compile regex")?;

    let output_content = process_lines(reader, input_file, &re)?;

    // Write the output content to the output file
    write_to_file(output_file, &output_content)?;

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
