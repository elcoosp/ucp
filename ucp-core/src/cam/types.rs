use serde::{Deserialize, Serialize};

/// Top‑level canonical representation of a UI component across codebases.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "discarding a synthesized component is likely a bug"]
pub struct CanonicalAbstractComponent {
    pub id: String,
    pub semantic_fingerprint: super::SemanticFingerprint,
    pub props: Vec<super::CanonicalAbstractProp>,
    pub events: Vec<super::CanonicalAbstractEvent>,
    pub extracted_state_machine: Option<super::StateMachine>,
    pub extracted_parts: Vec<super::ExtractedPart>,
    pub source_repos: Vec<super::SourceAttribution>,
    /// If the component provides a React/Dioxus/generic context, its type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_context: Option<String>,
    /// Types of contexts consumed by this component.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumed_contexts: Vec<String>,
}

/// A single abstract prop in the canonical model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAbstractProp {
    pub canonical_name: String,
    pub abstract_type: super::AbstractPropType,
    pub reactivity: super::AbstractReactivity,
    /// The normalised concrete type (e.g. "String", "bool", "enum: Default, Destructive").
    /// Preserves original type information beyond the abstract mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concrete_type: Option<String>,
    pub sources: Vec<super::PropSourceMapping>,
    pub confidence: f32,
    pub conflicts: Vec<super::Conflict>,
}

/// A semantic event emitted by a component (extracted from callback props).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAbstractEvent {
    pub canonical_name: String,
    pub abstract_payload: super::AbstractPropType,
}

/// Maps an original prop name/type from a specific source to its canonical form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropSourceMapping {
    pub repo_id: String,
    pub original_name: String,
    pub original_type: String,
}

/// A selectable sub‑region of a component (slots, parts, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedPart {
    pub name: String,
    pub selectable: bool,
}

/// Records where a component was extracted from.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAttribution {
    pub repo_url: String,
    pub file_path: String,
    pub line_start: usize,
}

// -------------------------------------------------------------------------
// Package Manifest
// -------------------------------------------------------------------------

/// Metadata for a library of UI components (e.g., shadcn‑dioxus, MUI).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    /// The library name (e.g., "shadcn-dioxus").
    pub name: String,
    /// Version of the library at extraction time.
    pub version: String,
    /// Frameworks / platforms supported.
    pub frameworks: Vec<String>,
    /// List of component definitions exported by this library.
    pub components: Vec<CanonicalAbstractComponent>,
    /// Optional global styles, theming tokens, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_styles: Option<String>,
    /// The extraction tool and version.
    pub generated_by: String,
    /// Timestamp of generation (ISO 8601).
    pub generated_at: String,
}
