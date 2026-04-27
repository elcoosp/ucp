# `ucp curate`

Interactive terminal UI for resolving merge conflicts.

## Usage

``` bash
ucp curate --merged <PATH> [--output <PATH>]
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--merged` | (required) | Path to merged spec with conflicts |
| `--output` | `./curated.json` | Where to save the curated spec |

## TUI Controls

| Key | Action |
|-----|--------|
| `←` `→` | Navigate conflicts |
| `a` | Accept suggestion |
| `r` | Reject |
| `s` | Skip |
| `q` | Quit and save |

## Example

``` bash
ucp curate --merged merged.json --output final.json
```
