# UCP Quickstart

Get from zero to a bootstrapped spec in 5 minutes.

## 1. Install UCP

``` bash
brew install elcoosp/tap/ucp
# or: cargo install ucp-cli
```

## 2. Bootstrap a Component Library

Point `ucp` at a source directory containing UI components:

``` bash
ucp bootstrap --source-dir ./my-dioxus-app/src --output-dir ./spec
```

This scans the directory, extracts component definitions, unifies them into
the Canonical Abstract Model (CAM), and writes `spec/ucp-spec.json`.

## 3. Inspect the Spec

``` bash
cat spec/ucp-spec.json | python3 -m json.tool | head -30
```

``` bash
ucp components spec/ucp-spec.json --verbose
```

## 4. Validate

``` bash
ucp validate spec/ucp-spec.json
```

## 5. Export to a Format

``` bash
ucp export --target design-md --spec spec/ucp-spec.json --output ./design
cat design/DESIGN.md
```

## Next Steps

- [Bootstrap Tutorial](tutorial-bootstrap.md)
- [Merge Tutorial](tutorial-merge.md)
- [Curation Tutorial](tutorial-curate.md)
- [Export Tutorial](tutorial-export.md)
- [Command Reference](commands/index.md)
