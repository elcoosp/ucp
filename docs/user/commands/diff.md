# `ucp diff`

Compare two specs structurally.

## Usage

``` bash
ucp diff --spec-a <PATH> --spec-b <PATH> [--json]
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--spec-a` | (required) | First spec |
| `--spec-b` | (required) | Second spec |
| `--json` | `false` | Output machine‑readable JSON |

## Example

``` bash
ucp diff --spec-a v1.json --spec-b v2.json
ucp diff --spec-a v1.json --spec-b v2.json --json
```
