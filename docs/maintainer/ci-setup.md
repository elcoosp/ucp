# CI Pipeline Documentation

UCP uses GitHub Actions for continuous integration.

## Workflows

### `test.yml` — Main CI

Runs on every push and PR to `main`:

- Checks out code
- Installs Rust (stable)
- Runs `just check` (compiles all crates with all targets)
- Runs `just test` (all tests via `cargo nextest`)
- Runs `just lint` (clippy with `-D warnings`)
- Runs `just fmt-check` (rustfmt in check mode)

### `coverage.yml` — Coverage (optional)

Runs on PRs to `main`:

- Runs `just coverage` (generates `lcov.info` and HTML report)
- Uploads to a coverage service

## Required Tools

- `cargo nextest` — faster test runner
- `cargo insta` — snapshot testing
- `cargo clippy` — linting
- `rustfmt` — formatting
- `cargo llvm-cov` — coverage (optional)

## GitHub Actions Example

``` yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-nextest
      - run: cargo check --all-targets
      - run: cargo nextest run
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets -- -D warnings
```

## Local CI Simulation

``` bash
# Same sequence as CI
just check
just test
just lint
just fmt-check
```
