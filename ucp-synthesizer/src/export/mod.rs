//! Spec exporters: turn a [`crate::pipeline::SynthesisOutput`] into
//! partner formats (W3C mapping, DESIGN.md, LLMs.txt, DTCG).

/// DESIGN.md writer.
pub mod design_md;
/// LLMs.txt writer.
pub mod llms_txt;
/// W3C design-components mapping export.
pub mod w3c;
