# UCP Architecture Overview

This document describes the high‑level architecture of the UCP workspace.

## C4 Context Diagram (Level 1)

```
[User/CI] --> [UCP CLI]
[UCP CLI] --> [ucp-synthesizer]
[UCP CLI] --> [ucp-maintainer]
[ucp-synthesizer] --> [File System (source code, specs)]
[ucp-maintainer] --> [SQLite (registry)]
[ucp-maintainer] --> [Ollama (optional LLM)]
```

## Crate Responsibilities

| Crate | Role |
|-------|------|
| `ucp-core` | Canonical Abstract Model (CAM), SMDL parser, error types |
| `ucp-synthesizer` | Extraction, unification, merge, code generation, export |
| `ucp-maintainer` | Curation, diff, tokens, verify, registry, watch |
| `ucp-cli` | CLI binary, subcommand dispatch |

## Data Flow

1. **Bootstrap**: Source files → Extractors → Raw extractions → Unification → `SynthesisOutput`
2. **Merge**: Multiple `SynthesisOutput` → Deduplication (semantic fingerprint) → Conflict detection → Merged spec
3. **Curation**: Merged spec → Interactive TUI → Resolved conflicts → Curated spec
4. **Export**: Curated spec → Format-specific exporters → A2UI, AG‑UI, W3C, DESIGN.md, LLMs.txt, Registry
5. **Watch**: File watcher → Bootstrap → Merge (if base spec) → Export

## Key Design Decisions

See [Architecture Decision Records](adr/) for detailed rationale on:
- Using the CAM for framework‑agnostic representation
- Semantic fingerprinting for deduplication
- Ratatui for the curation TUI
- SQLite for the local registry
- `notify` crate for watch mode
