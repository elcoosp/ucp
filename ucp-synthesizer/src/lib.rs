//! # UCP Synthesizer
//!
//! The core synthesis engine of the Universal Component Protocol.
//!
//! Pipeline:
//! 1. **Extract** raw components from source files (Rust, TSX, Svelte, Vue, Dioxus).
//! 2. **Unify** raw types into the Canonical Abstract Model (CAM).
//! 3. **Merge** multiple specs, detect conflicts, deduplicate by semantic fingerprint.
//! 4. **Export** to various formats (W3C, DESIGN.md, LLMs.txt, DTCG, Registry).
//!
//! This crate also hosts:
//! - Framework code generators (Dioxus, Leptos, GPUI, React, Web Components)
//! - Design token extraction and DTCG export
//! - Security and path‑safety utilities
//! - DESIGN.md import

/// Spec exporters (W3C, DESIGN.md, LLMs.txt).
pub mod export;
/// Framework-specific raw extractors.
pub mod extract;
/// Framework code generators.
pub mod generate;
/// External format importers (DESIGN.md).
pub mod import;
/// Spec merging with fingerprint deduplication.
pub mod merge;
/// The extract -> unify -> export pipeline.
pub mod pipeline;
/// SPDX and path-safety checks.
pub mod security;
/// Raw-type -> CAM type ontology mapping.
pub mod unify;
/// Small shared string/timestamp helpers.
pub mod utils;
