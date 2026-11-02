//! # UCP Synthesizer
//!
//! The core synthesis engine of the Universal Component Protocol.
//!
//! Pipeline:
//! 1. **Extract** raw components from source files (Rust, TSX, Svelte, Vue, Dioxus).
//! 2. **Unify** raw types into the Canonical Abstract Model (CAM).
//! 3. **Merge** multiple specs, detect conflicts, deduplicate by semantic fingerprint.
//! 4. **Enrich** (optional) with LLM‑provided descriptions and state machines.
//! 5. **Export** to various formats (A2UI, AG‑UI, W3C, DESIGN.md, LLMs.txt, Registry).
//!
//! This crate also hosts:
//! - Framework code generators (Dioxus, Leptos, GPUI, React, Web Components)
//! - Design token extraction and DTCG export
//! - Security and path‑safety utilities
//! - DESIGN.md import

/// AI contract + MCP server surfaces.
pub mod contract;
/// Conflict curation decision application.
pub mod curation;
/// Static HTML curation dashboard.
pub mod dashboard;
/// GitHub repository discovery (feature `github-discovery`).
pub mod discovery;
/// Spec exporters (A2UI, AG-UI, W3C, DESIGN.md, LLMs.txt).
pub mod export;
/// Framework-specific raw extractors.
pub mod extract;
/// Framework code generators.
pub mod generate;
/// External format importers (DESIGN.md).
pub mod import;
/// LLM enrichment client and prompts (feature `llm`).
#[cfg(feature = "llm")]
pub mod llm;
/// LLM stub when the `llm` feature is disabled.
#[cfg(not(feature = "llm"))]
pub mod llm {
    // stub
}
/// Spec merging with fingerprint deduplication.
pub mod merge;
/// The extract -> unify -> enrich -> export pipeline.
pub mod pipeline;
/// SPDX and path-safety checks.
pub mod security;
/// Raw-type -> CAM type ontology mapping.
pub mod unify;
/// Small shared string/timestamp helpers.
pub mod utils;
