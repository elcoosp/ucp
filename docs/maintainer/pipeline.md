# Extraction Pipeline

How UCP turns source code into a `SynthesisOutput` spec.

## Overview

The pipeline is implemented in `ucp-synthesizer/src/pipeline/` and has these stages:

1. **Walk source directory** (`extraction.rs:walk_source_dir`)
2. **Extract per‑file** (`rust_ast.rs`, `tsx_ast.rs`, `dioxus_ast.rs`, `vue_ast.rs`, `svelte_ast.rs`)
3. **Unify raw types → CAM** (`unification.rs`)
4. **Detect conflicts** (`conflicts.rs`)
5. **Enrich with LLM (optional)** (`enrichment.rs`)
6. **Produce `SynthesisOutput`** (`output.rs`)

## Stage Details

### Walk Source Directory

Only files inside a `src/` or `components/` directory are processed.
Files with dangerous extensions (`.pem`, `.key`, `.env`) or in excluded
directories (`node_modules`, `target`, `.git`) are skipped. Hidden files
are also rejected.

### Extraction

Each supported language has its own extractor:

| Language | Extractor | What it finds |
|----------|-----------|---------------|
| Rust (Leptos) | `rust_ast::ComponentVisitor` | `#[component] fn ...` |
| Rust (Dioxus) | `dioxus_ast::DioxusVisitor` | `#[derive(Props)]` struct + `#[component] fn` |
| Rust (GPUI) | `rust_ast::GpuiComponentVisitor` | `#[derive(IntoElement)]` struct + methods |
| Rust (Struct‑props) | `rust_ast::StructComponentVisitor` | `XxxProps` struct + `impl Xxx { fn render }` |
| TypeScript/TSX | `tsx_ast::extract_tsx_components` | `interface/type Props`, function/const exports |
| Vue | `vue_ast::extract_vue_components` | `<script setup> defineProps/defineEmits` |
| Svelte | `svelte_ast::extract_svelte_components` | `<script> $props()` |

### Unification

Raw types (e.g., `RwSignal<String>`, `Option<bool>`, `fn()`) are mapped
to CAM abstract types (`ControlledValue`, `UncontrolledValue`, `ControlFlag`,
`AsyncEventHandler`, etc.) via `unify.rs`.

### Conflict Detection

When merging, components with the same semantic fingerprint are compared.
Props with different abstract types across sources are flagged as conflicts.
See [Merge Algorithm](merge-algorithm.md) for details.

### LLM Enrichment

If an Ollama URL is provided, the pipeline sends component source code
to the LLM with a prompt asking for a description, SMDL state machine,
and keywords. The response is parsed into `EnrichmentResponse`.
