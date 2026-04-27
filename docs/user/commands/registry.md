# `ucp registry`

Local SQLite-backed spec store.

## Subcommands

| Command | Description |
|---------|-------------|
| `store` | Save a spec |
| `list` | List stored specs |
| `show <ID>` | Display a spec |
| `delete <ID>` | Remove a spec |

## Example

``` bash
ucp registry-store store --spec spec.json --name my-spec
ucp registry-store list
ucp registry-store show 1
ucp registry-store delete 1
```
