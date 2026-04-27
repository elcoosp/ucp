# `ucp merge`

Merge multiple UCP specs into one.

## Usage

``` bash
ucp merge --input <PATH> [--input <PATH> ...] [--output <PATH>] [--html-dir <PATH>]
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--input` | (required, repeatable) | Paths to spec files to merge |
| `--output` | `./ucp-output/merged.json` | Output path for merged spec |
| `--html-dir` | `./ucp-output` | Directory for conflict review HTML |

## Example

``` bash
ucp merge --input a.json --input b.json --input c.json -o merged.json
```
