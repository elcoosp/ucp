# `ucp watch`

Watch a source directory and automatically rebuild the spec on changes.

## Usage

``` bash
ucp watch --source-dir <PATH> [--output <PATH>] [--base-spec <PATH>] [--library <NAME>] [--version <VER>] [--debounce-ms <MS>]
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--source-dir` | (required) | Directory to watch |
| `--output` | `./watch-output` | Output directory |
| `--base-spec` | `None` | Existing spec to merge into incrementally |
| `--library` | `ucp-library` | Library name |
| `--version` | `0.1.0` | Version string |
| `--debounce-ms` | `500` | Debounce delay in milliseconds |

## Example

``` bash
ucp watch --source-dir src --base-spec canonical.json --output live
```
