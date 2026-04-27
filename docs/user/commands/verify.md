# `ucp verify`

Check for drift between a canonical spec and source code.

## Usage

``` bash
ucp verify --spec <PATH> --source-dir <PATH>
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--spec` | (required) | Canonical spec to check |
| `--source-dir` | (required) | Source directory to re‑extract from |

## Example

``` bash
ucp verify --spec canonical.json --source-dir src
```
