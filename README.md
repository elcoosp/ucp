# UCP — Universal Component Protocol

[![Rust](https://img.shields.io/badge/rust-1.80%2B-blue)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.11.0-orange)](https://crates.io/crates/ucp-cli)

**AI‑native component specification, extraction, and governance CLI.**

UCP scans frontend codebases, extracts UI component definitions into a
framework‑agnostic **Canonical Abstract Model (CAM)**,
merges specs from multiple sources, detects conflicts, and exports
to multiple formats (A2UI, AG‑UI, W3C, DESIGN.md, LLMs.txt, shadcn registry).

## Quick Start

``` bash
# Install
brew install elcoosp/tap/ucp
# or: cargo install ucp-cli

# Bootstrap from a source directory
ucp bootstrap --source-dir ./my-dioxus-app/src

# Merge specs from two codebases
ucp merge --input dioxus-spec.json --input react-spec.json -o merged.json

# Curate conflicts interactively
ucp curate --merged merged.json --output curated.json

# Export to all formats
ucp export-all --spec curated.json --output ./dist
```

## Documentation

- [User Guide](docs/user/quickstart.md)
- [CLI Command Reference](docs/user/commands/index.md)
- [Conceptual Overview](docs/user/concepts/cam.md)
- [Contributing](CONTRIBUTING.md)
- [Maintainer Guide](MAINTAINERS.md)
- [Architecture](docs/maintainer/architecture.md)

## Workspace Crates

| Crate | Purpose |
|-------|---------|
| `ucp-core` | Canonical Abstract Model (CAM) types, SMDL parser, error types |
| `ucp-synthesizer` | Extraction, unification, merging, LLM enrichment, code generation, exports |
| `ucp-maintainer` | Interactive curation, diff, token merge, drift detection, registry, watch mode |
| `ucp-cli` | CLI binary with all subcommands |

## License

MIT — see [LICENSE](LICENSE).
