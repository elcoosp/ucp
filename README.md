# UCP v4.0 — AI Unification Engine

Universal Component Protocol synthesizer that extracts UI components from
React, Leptos, and GPUI codebases, unifies them into a canonical abstract
model (CAM), and detects cross-framework conflicts.

## Features

- **AST-based extraction** from Rust (Leptos/GPUI `#[component]`) and TSX/JSX/TS
- **Canonical Abstract Model** with typed props, events, state machines
- **Conflict detection** across frameworks (type mismatches, missing props)
- **LLM enrichment** via Ollama for semantic descriptions and SMDL state machines
- **Merge & curation** workflow with HTML review UI for conflict resolution
- **React.FC and class component** support

## Install

```bash
cargo install --path .
```

## Quick Start

```bash
# Extract components from a source directory
ucp bootstrap --source-dir ./src

# With LLM enrichment
ucp bootstrap --source-dir ./src --ollama-url http://localhost:11434 --llm-model llama3

# Watch for changes (requires watchexec)
ucp bootstrap --source-dir ./src --watch
```

## Commands

```bash
# Validate a spec file
ucp validate ucp-spec.json

# Merge multiple specs
ucp merge --input a.json --input b.json -o merged.json

# List components (text)
ucp components ucp-spec.json

# JSON output (composable in pipelines)
ucp components --format json ucp-spec.json

# Substring filter
ucp components --filter "Button" ucp-spec.json

# Regex filter
ucp components --filter "^Button$|^Input$" ucp-spec.json
```

## Architecture

```
ucp-core/          # Data types, SMDL parser, error types
ucp-synthesizer/   # Extraction, unification, pipeline, merge, curation
ucp-cli/           # CLI binary (clap)
```

## License

MIT
