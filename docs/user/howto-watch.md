# How-to: Continuous Watch Mode

Automatically rebuild specs when source files change.

## Basic Watch

``` bash
ucp watch --source-dir ./src --output ./live-spec
```

Every time a file changes in `./src`, UCP will re‑extract, merge (if a
base spec is given), and export all formats.

## Watch with Base Spec

``` bash
ucp watch --source-dir ./new-codebase --base-spec canonical.json --output ./live
```

This merges the freshly extracted spec into the existing canonical spec
before exporting – useful for incremental updates.

## Custom Debounce

``` bash
ucp watch --source-dir ./src --output ./live --debounce-ms 1000
```
