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


pub mod contract;
pub mod curation;
pub mod dashboard;
pub mod discovery;
pub mod export;
pub mod extract;
pub mod generate;
pub mod import;
#[cfg(feature = "llm")]
pub mod llm;
#[cfg(not(feature = "llm"))]
pub mod llm {
    // stub
}
pub mod merge;
pub mod pipeline;
pub mod security;
pub mod unify;
pub mod utils;
