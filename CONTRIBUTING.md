# Contributing to UCP (Universal Component Protocol)

First off, thank you for considering contributing to UCP! Because UCP v4.0 is an AI-synthesized standard built on strict security and auditing principles, the contribution process has specific constraints, particularly around the AI synthesis pipeline.

## Table of Contents
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security Policy](SECURITY.md) ⚠️ **Read this first, especially for `ucp-synthesizer` work**
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [The SEP Process (Spec Changes)](#the-sep-process)
- [PR Checklist](#pr-checklist)

## Getting Started

### Prerequisites
- **Rust:** Latest stable toolchain (`rustup default stable`).
- **Local LLM (Optional for Pipeline Dev):** [Ollama](https://ollama.ai/) running locally with a model pulled (e.g., `ollama pull glm-5:cloud`).
- **Node.js (Optional for Web Harness Testing):** Required if running WASM-based conformance tests for frameworks like Leptos.

### Building the Workspace
UCP is structured as a Cargo workspace containing three primary crates:
```bash
git clone https://github.com/ucp-project/ucp.git
cd ucp
cargo build
cargo test
```

## How to Contribute

UCP is divided into distinct subsystems. Please ensure your PR targets the correct crate:
- **`ucp-core`**: Data models (Canonical Abstract Model), SMDL state machine parser, and JSON Schema definitions.
- **`ucp-synthesizer`**: The AI pipeline (Discovery, AST extraction, LLM inference, Unification). *Subject to strict security review.*
- **`ucp-cli`**: Command-line interface and orchestration logic.

### Coding Standards
- Follow standard Rust `clippy` linting (`cargo clippy -- -D warnings`).
- Write tests for all new logic. We use a strict TDD approach in subagent workflows.
- **LLM Integration:** You MUST use raw `reqwest` calls to the local Ollama REST API (`http://localhost:11434/api/generate`). Do not introduce third-party Ollama crates (like `ollama-rs`), as they abstract away the strict `"format": "json"` payload requirements needed for models like `glm-5:cloud`.
- **Mocking:** When writing tests for LLM inference, you MUST use a mock struct implementing the `LlmProvider` trait (see `ucp-synthesizer/tests/llm_mock.rs`). Do not mock at the HTTP layer; mock at the trait boundary to ensure your tests validate the JSON structure, not the network stack.
- **AST Parsing:** Use `syn::visit::Visit` for Rust extraction and `biome_js_parser` for TSX extraction to keep build times and dependency trees minimal.

### Security Constraints (For `ucp-synthesizer` contributors)
If you are modifying the synthesis pipeline, you must adhere to the following non-negotiable rules:
1. **No External Network Calls:** The pipeline must never make network requests to external AI APIs (e.g., OpenAI) by default.
2. **Strict JSON Formatting:** When building the Ollama payload, you must include `"format": "json"` and `"stream": false`.
3. **Sandboxing:** Do not modify the `is_path_safe_to_parse` function to allow parsing of `.env`, `credentials`, or `tests/` directories.
4. **No Source Logging:** Do not log raw extracted source code from third-party repositories to standard output or local files during the synthesis phase.

## The SEP Process (Spec Changes)

UCP does not accept direct pull requests that modify the core specification (e.g., changing the `CanonicalAbstractComponent` struct in `ucp-core/src/cam.rs` or the generated JSON schemas) without a Spec Enhancement Proposal (SEP).

Because UCP is computed from existing code, a SEP is required to change *how* the AI computes the consensus. 

1. **Draft the SEP:** Open an issue using the `SEP: <Title>` template detailing what behavioral or API change is needed.
2. **Cross-Ecosystem Review:** SEPs require approval from maintainers of at least 3 different language ecosystems (e.g., JS/TS, Rust-Web, Rust-Desktop).
3. **Implement Code Changes:** Once the SEP is approved, submit a PR referencing the SEP issue number.
4. **Regenerate Schema:** If the SEP changes the CAM, run the pipeline locally to ensure the output UCP JSON schema updates correctly.

## PR Checklist

Before submitting your PR, ensure you have checked the following boxes:
- [ ] My code compiles without warnings (`cargo check`).
- [ ] I have added tests that prove my feature works (`cargo test`).
- [ ] I have run `cargo clippy` and resolved all warnings.
- [ ] **[Synthesizer Only]** I have not introduced third-party HTTP clients for external AI APIs (I used `reqwest` targeting `localhost`).
- [ ] **[Synthesizer Only]** My LLM mock tests utilize the `LlmProvider` trait, not HTTP mocks.
- [ ] **[Spec Changes Only]** My PR references an approved SEP issue number in the description.
