<div align="center">
  <img src="docs/logo.png" alt="UCP Logo" width="300"/>
  <p>
    <strong>AI‑native component specification, extraction, and governance CLI for the shadcn/ui ecosystem.</strong><br/>
    UCP scans frontend codebases, extracts UI component definitions into a framework‑agnostic <b>Canonical Abstract Model (CAM)</b>, merges specs from multiple sources, detects conflicts, and exports to A2UI, AG‑UI, W3C, DESIGN.md, LLMs.txt, and the shadcn registry. Written in Rust, with a scriptable CLI and an interactive TUI for curation.
  </p>
  <p>
    <img src="https://img.shields.io/badge/Rust-1.80%2B-000000?style=flat-square&logo=rust" alt="Rust"/>
    <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License MIT"/>
    <img src="https://img.shields.io/badge/Crates-4-6F4E37?style=flat-square" alt="Crates"/>
    <img src="https://img.shields.io/badge/Frameworks-React%20%7C%20Leptos%20%7C%20Dioxus%20%7C%20GPUI-61DAFB?style=flat-square" alt="Frameworks"/>
    <img src="https://img.shields.io/badge/Export-A2UI%20%7C%20AG--UI%20%7C%20W3C%20%7C%20DESIGN.md-4B32C3?style=flat-square" alt="Export formats"/>
    <img src="https://img.shields.io/badge/Interface-CLI%20%2B%20TUI-00BFFF?style=flat-square" alt="Interface"/>
    <img src="https://img.shields.io/badge/Backend-Ollama%20(optional)-228B22?style=flat-square" alt="Backend"/>
    <img src="https://img.shields.io/badge/Build-Passing-brightgreen?style=flat-square&logo=githubactions" alt="Build"/>
  </p>
</div>

---

# UCP

> [!NOTE]
> UCP is under active development. The workspace builds and the CLI is usable, but the public API and spec format are still stabilising. See [ROADMAP.md](ROADMAP.md) for what is wired up today versus what is planned.

UCP solves a concrete problem: every framework port of a component library (React, Leptos, Dioxus, GPUI) describes the same components in a different way. UCP **discovers** those ports, **extracts** their APIs, **resolves semantic equivalences** across paradigms, and **synthesises** a single, canonical, machine‑readable specification. That spec can then be curated, merged, validated, and exported to the formats your downstream tools actually consume.

---

## Quick Start

```bash
# Install
brew install elcoosp/tap/ucp
# or:
cargo install ucp-cli

# Bootstrap a spec from a source directory
ucp bootstrap --source-dir ./my-dioxus-app/src --output-dir ./spec

# Merge specs from two codebases
ucp merge --input dioxus-spec.json --input react-spec.json -o merged.json

# Curate conflicts interactively (TUI)
ucp curate --merged merged.json --output curated.json

# Export to every supported format
ucp export-all --spec curated.json --output ./dist
```

A full walkthrough from zero to a validated spec is available in the [Quickstart guide](docs/user/quickstart.md).

---

## What UCP Does

| Stage | Command | What happens |
|-------|---------|--------------|
| **Bootstrap** | `ucp bootstrap` | Scans source files, extracts component definitions, unifies them into the CAM, and writes `ucp-spec.json`. |
| **Merge** | `ucp merge` | Combines multiple `SynthesisOutput` files, deduplicates via semantic fingerprinting, and flags conflicts. |
| **Curate** | `ucp curate` | Opens an interactive TUI (built with Ratatui) to resolve conflicts and produce a curated spec. |
| **Validate** | `ucp validate` | Checks the spec against UCP’s schema and invariants. |
| **Export** | `ucp export` / `ucp export-all` | Emits A2UI, AG‑UI, W3C, DESIGN.md, LLMs.txt, or a shadcn registry from the curated spec. |
| **Watch** | `ucp watch` | Watches source files and automatically re‑runs bootstrap → merge → export when they change. |

UCP optionally enriches extracted specs with a local LLM (Ollama) when deeper semantic understanding is needed, but the core pipeline works entirely offline.

---

## Workspace Crates

UCP is a Rust workspace of four crates, each with a single responsibility:

| Crate | Purpose |
|-------|---------|
| `ucp-core` | Canonical Abstract Model (CAM) types, SMDL parser, error types. |
| `ucp-synthesizer` | Extraction, unification, merging, LLM enrichment, code generation, and all export formats. |
| `ucp-maintainer` | Interactive curation TUI, diff, token merge, drift detection, local registry (SQLite), and watch mode. |
| `ucp-cli` | The `ucp` binary with subcommand dispatch and argument parsing. |

---

## Documentation

- [User Guide](docs/user/quickstart.md) — installation, tutorials, and the full command reference.
- [Conceptual Overview: CAM](docs/user/concepts/cam.md) — how UCP represents components in a framework‑agnostic way.
- [Architecture](docs/maintainer/architecture.md) — C4 context, crate responsibilities, and data flow.
- [ROADMAP.md](ROADMAP.md) — current status and planned work.
- [CONTRIBUTING.md](CONTRIBUTING.md) — how to build, test, and submit changes.
- [SECURITY.md](SECURITY.md) — vulnerability reporting.

---

## Design Principles

- **Framework‑agnostic first.** The CAM is the source of truth, not any one framework’s component model.
- **Offline by default.** Bootstrap, merge, curate, validate, and export all work without a network. Ollama enrichment is opt‑in.
- **Machine‑readable outputs.** Every export is deterministic and diff‑friendly, so specs can live in version control and drive CI.
- **Human‑in‑the‑loop curation.** Automatic merging gets you 90% of the way; the TUI makes the remaining conflicts cheap to resolve.

---

## License

MIT — see [LICENSE](LICENSE).
