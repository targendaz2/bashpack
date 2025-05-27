# Bashpack

A bundler for Bash scripts. Think webpack or Rollup for Bash.

## Features

- [x] single-level source file bundling
- [ ] recursive source file bundling
- [ ] circular reference handling
- [ ] config file support
- [ ] use Bash AST for bundling
- [ ] resolve dynamic source paths

## Finding Sourced Files

Bash attempts to source files relative to the current working directory. As a result, scripts will often use something like `BASE_DIR="$(dirname "${BASH_SOURCE[0]}")"` to get the current script's directory and use that to source files relative to the script. This poses a problem for Bashpack as it would need to execute Bash to handle any dynamic source paths. For the initial release, dynamic paths _will not be supported_, which results in the following behavior:

### Absolute Static Paths

Resolved as is.

### Relative Static Paths

Resolved relative to the current working directory.

### Dynamic Paths

Throw an error.
