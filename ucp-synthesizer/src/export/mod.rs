//! Spec exporters: turn a [`crate::pipeline::SynthesisOutput`] into
//! partner formats (A2UI catalog, AG-UI events, W3C mapping, DESIGN.md,
//! LLMs.txt).

/// A2UI catalog export.
pub mod a2ui;
/// AG-UI protocol export.
pub mod ag_ui;
/// DESIGN.md writer.
pub mod design_md;
/// LLMs.txt writer.
pub mod llms_txt;
/// W3C design-components mapping export.
pub mod w3c;
