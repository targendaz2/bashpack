use anyhow::{Context, Result};
use regex::Regex;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

/// Resolves and bundles a Bash script recursively by inlining `source` and `.` directives
pub fn resolve_script(entrypoint: &Path) -> Result<String> {
    let re = Regex::new(r#"^\s*(source|\.)\s+([^\s]+)"#).unwrap();
    let mut visited = HashSet::new();
    resolve_script_recursive(entrypoint, &re, &mut visited)
}

fn inline_script_content(
    content: &str,
    parent_directory: &Path,
    re: &Regex,
    visited: &mut HashSet<PathBuf>,
) -> Result<String> {
    let mut bundled = String::new();

    for line in content.lines() {
        if let Some(captures) = re.captures(line) {
            let relative_path = captures.get(2).unwrap().as_str();
            let sourced_path = parent_directory.join(relative_path);
            bundled.push_str(&format!("# Inlined: {sourced_path:?}\n"));
            let resolved = resolve_script_recursive(&sourced_path, re, visited)?;
            bundled.push_str(&resolved);
        } else {
            bundled.push_str(line);
            bundled.push('\n');
        }
    }

    Ok(bundled)
}

pub fn resolve_script_recursive(
    path: &Path,
    re: &Regex,
    visited: &mut HashSet<PathBuf>,
) -> Result<String> {
    let canonical_path =
        fs::canonicalize(path).with_context(|| format!("Failed to resolve path: {path:?}"))?;

    // Prevent infinite recursion
    if !visited.insert(canonical_path.clone()) {
        return Ok(format!(
            "# Skipping already inlined file: {canonical_path:?}\n"
        ));
    }

    let content = fs::read_to_string(&canonical_path)
        .with_context(|| format!("Failed to read script: {canonical_path:?}"))?;

    let parent_directory = canonical_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    let bundled = inline_script_content(&content, &parent_directory, re, visited)
        .with_context(|| format!("Failed to inline script: {canonical_path:?}"))?;

    Ok(bundled)
}
