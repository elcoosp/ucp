# `ucp export-all`

Export a spec to all supported formats in one command.

## Usage

``` bash
ucp export-all --spec <PATH> [--output <PATH>] [--library <NAME>] [--version <VER>]
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--spec` | (required) | Source spec |
| `--output` | `./export-all` | Output directory |
| `--library` | `ucp-library` | Library name for exports |
| `--version` | `0.1.0` | Version string |

## Example

``` bash
ucp export-all --spec curated.json --output ./dist --library my-lib --version 1.0.0
```
