# `ucp bootstrap`

Extract UI component definitions from a source directory.

## Usage

``` bash
ucp bootstrap --source-dir <PATH> [--output-dir <PATH>] [--json]
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--source-dir` | (required) | Directory containing component source files |
| `--output-dir` | `./ucp-output` | Where to write the spec |
| `--json` | `false` | Print a machine-readable summary instead of the human log |

## Example

``` bash
ucp bootstrap --source-dir src --output-dir spec
```
