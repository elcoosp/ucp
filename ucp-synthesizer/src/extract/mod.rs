//! Framework-specific raw extractors.
//!
//! Each `*_ast` module knows one source flavor and produces raw
//! extraction structs; [`crate::pipeline::unification`] then reduces
//! them to the CAM. [`context`] detects provided/consumed context usage
//! shared by several flavors.

/// Shared context (provide/use) detection across frameworks.
pub mod context;
/// Dioxus raw extractor.
pub mod dioxus_ast;
/// Rust (Dioxus/Leptos/GPUI-style) raw extractor.
pub mod rust_ast;
/// Svelte raw extractor.
pub mod svelte_ast;
/// DTCG design-token extraction.
pub mod tokens;
/// TSX/TS/JSX raw extractor.
pub mod tsx_ast;
/// Vue raw extractor.
pub mod vue_ast;
