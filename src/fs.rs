use anyhow::{Context, Result, bail};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

/// Opens a file at the given path and returns a buffered reader.
pub fn read_file(path: &Path) -> Result<BufReader<File>> {
    if !path.exists() {
        bail!("File does not exist: {path:?}");
    }

    let file = File::open(path).context(format!("Failed to open input file: {path:?}"))?;

    let reader = BufReader::new(file);
    Ok(reader)
}

/// Gets the parent directory of the given path.
pub fn get_parent_path(path: &Path) -> Result<PathBuf> {
    let path = path.parent().context("File has no parent directory")?;
    Ok(path.to_path_buf())
}

/// Writes the given content to a file at the specified path.
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    let path = path.to_path_buf();
    let path_handle =
        File::create(&path).context(format!("Failed to create output file: {path:?}"))?;

    let mut writer = BufWriter::new(path_handle);
    writer
        .write_all(content.as_bytes())
        .context(format!("Failed to write to output file: {path:?}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod open_file {
        use super::*;

        use std::io::BufRead;
        use tempfile::NamedTempFile;

        #[test]
        fn returns_a_reader_if_file_exists() {
            let mut tmpfile = NamedTempFile::new().unwrap();
            write!(tmpfile, "echo 'test'").unwrap();
            let reader = read_file(tmpfile.path()).unwrap();
            assert!(reader.lines().next().is_some());
        }

        #[test]
        fn returns_an_error_if_file_doesnt_exist() {
            let non_existent_path = Path::new("non_existent_file.txt");
            let result = read_file(non_existent_path);
            assert!(result.is_err());
        }
    }
}
