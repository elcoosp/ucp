# Contributing to UCP

Welcome! This document outlines how to set up the project, make changes,
and submit contributions.

## Development Setup

### Prerequisites

- Rust stable 1.80+ (install via [rustup](https://rustup.rs))
- [just](https://github.com/casey/just) command runner (`brew install just`)

### Clone and Build

``` bash
git clone https://github.com/elcoosp/ucp.git
cd ucp
just check          # compile all crates
just test           # run all tests (uses cargo nextest)
```

### Workspace Structure

```
ucp/
├── ucp-core/           # Core types, SMDL parser
├── ucp-synthesizer/    # Extraction, merge, generation, exports
├── ucp-maintainer/     # Curation, diff, tokens, verify, registry, watch
├── ucp-cli/            # CLI binary
└── docs/               # Documentation (mdBook)
```

## Development Workflow

1. **Pick or create an issue** — describe what you're changing and why.
2. **Create a branch** — `git checkout -b feat/my-feature`
3. **Make changes** — write code and tests.
4. **Run tests** — `just test` (passes all unit, integration, doc, snapshot, proptest).
5. **Run lints** — `just lint` (clippy) and `just fmt-check` (rustfmt).
6. **Commit** — follow [Conventional Commits](https://www.conventionalcommits.org/).
7. **Push and open a PR**.

## Quality Gates

- All tests must pass (`just test`)
- No clippy warnings (`just lint`)
- Code formatted (`just fmt-check`)
- New public APIs must include doc comments with `# Examples`
- Significant changes should include an Architecture Decision Record in `docs/maintainer/adr/`

## Documentation

- **API docs** — `cargo doc --open`
- **Book** — `just doc` (builds mdBook from `docs/`)
- **Doc tests** — compiled as part of `just test`; ensure examples are runnable

## Release Process

See [MAINTAINERS.md](MAINTAINERS.md) for the full release checklist.
