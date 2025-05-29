# Bashpack

**Bashpack** is a CLI tool written in Rust that bundles multiple Bash scripts into a single file.

## Features

- [x] Recursively resolve and inline `source ./script.sh` and `. ./script.sh` commands
- [x] Prevent duplicate inlining of the same file (cycle-safe)
- [x] Output to file or stdout
- [x] Optional minification: removes comments and blank lines while preserving the shebang
- [x] Optional "no inline" mode to only apply formatting (e.g. minification)
- [x] Verbose mode to show resolved paths

## Example

```bash
# scripts/main.sh

#!/bin/bash
source ./lib.sh
echo 'Done'
```

```bash
# scripts/lib.sh

echo 'Hello from lib'
```

Run:

```bash
mkdir dist
cargo run -- scripts/main.sh --output dist/bundle.sh
```

Output:

```bash
# dist/bundle.sh

#!/bin/bash
# Inlined: /abs/path/to/scripts/lib.sh
echo 'Hello from lib'
echo 'Done'
```

## CLI Usage

```bash
bashpack <ENTRYPOINT> [OPTIONS]
```

### Positional Arguments

- `<ENTRYPOINT>`: The root .sh file to bundle

### OPTIONS

| Flag          | Description                                               |
| ------------- | --------------------------------------------------------- |
| -o, --output  | Path to output file. Defaults to stdout.                  |
| --minify      | Strip comments and blank lines (preserves shebang).       |
| --no-inline   | Output only the entrypoint as-is (no source resolving).   |
| --dry-run     | Print the list of scripts that would be inlined and exit. |
| -v, --verbose | Print sourced files and other debug info.                 |

## Project Structure

```text
src/
├── main.rs        # Entrypoint
├── cli.rs         # CLI argument parsing with clap
├── bundler.rs     # Orchestrates bundling/minification/output
└── resolver.rs    # Resolves and inlines sourced Bash files recursively
```

## Dependencies

| Crate  | Use Case                              |
| ------ | ------------------------------------- |
| clap   | Command-line argument parsing         |
| anyhow | Ergonomic error handling with context |
| regex  | Regex matching for source statements  |

## Future Plans

### Functional Improvements

[ ] Handle `source` using variables (e.g. `source "$SCRIPT_DIR/foo.sh"`)
[ ] Handle `source` in if/case/functions conditionally
[ ] Add support for config file
[x] Add `--dry-run` to show what would be bundled
[ ] Add `--watch` to auto-bundle on changes (e.g. via `notify`)
[ ] Preserve file boundaries with inline `# ----- filename -----` comments
[ ] Add `--strip-sourced-comments` (keep entrypoint comments only)

### Packaging

- Publish to crates.io
- Homebrew formula or install script
- GitHub release with binary builds

## Developer Notes

- `--dry-run` is implemented using `resolve_script_recursive()` directly with a custom `visited` set.
- The `bundler::dry_run()` function handles dry-run logic separately from `bundle_script()`.
