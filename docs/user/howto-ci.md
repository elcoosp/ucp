# How-to: Integrate UCP in CI/CD

Run UCP verification as part of your CI pipeline.

## GitHub Actions Example

``` yaml
name: UCP Verify
on: [push, pull_request]
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install UCP
        run: cargo install ucp-cli
      - name: Bootstrap spec
        run: ucp bootstrap --source-dir src --output-dir spec
      - name: Validate spec
        run: ucp validate spec/ucp-spec.json
      - name: Check drift (if canonical spec exists)
        run: ucp verify --spec spec/canonical.json --source-dir src || echo "Drift detected"
```

## Using with Spec Registry

Store a canonical spec in your repo and re‑verify on each PR:

``` bash
ucp registry-store store --spec spec/canonical.json --name main
ucp verify --spec spec/canonical.json --source-dir src
```
