/// Check if a line of text is a source line.
pub fn is_source_line(line: &str) -> bool {
    line.trim_start().starts_with("source")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod is_source_line {
        use super::*;

        #[test]
        fn returns_true_when_line_is_source_line() {
            let line = "source /path/to/file.sh";
            assert!(is_source_line(line));
        }

        #[test]
        fn returns_true_when_source_line_has_leading_spaces() {
            let line = "    source /path/to/file.sh";
            assert!(is_source_line(line));
        }

        #[test]
        fn returns_false_when_line_is_not_source_line() {
            let line = "echo 'Hello, World!'";
            assert!(!is_source_line(line));
        }
    }
}
