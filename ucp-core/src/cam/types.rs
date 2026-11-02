//! Core CAM entity types: components, props, events, parts, and manifests.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{AbstractPropType, AbstractReactivity, Conflict, SemanticFingerprint, StateMachine};

/// Top-level canonical representation of a UI component across codebases.
///
/// A `CanonicalAbstractComponent` is produced by unifying one or more raw
/// framework-specific extractions (a Leptos `#[component]` fn, a React FC,
/// a GPUI struct, ...) into a single framework-agnostic description.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[must_use = "discarding a synthesized component is likely a bug"]
pub struct CanonicalAbstractComponent {
    /// Namespaced identifier, e.g. `rust:src/button.rs:Button` or
    /// `unified:Button` after merging.
    pub id: String,
    /// Hash-based semantic identity used to cluster the same component
    /// across different codebases.
    pub semantic_fingerprint: SemanticFingerprint,
    /// All props (attributes / parameters) the component accepts.
    pub props: Vec<CanonicalAbstractProp>,
    /// Semantic events the component emits (derived from callback props).
    pub events: Vec<CanonicalAbstractEvent>,
    /// Optional behavioral state machine (from SMDL or LLM inference).
    pub extracted_state_machine: Option<StateMachine>,
    /// Selectable sub-regions (slots / parts) of the component.
    pub extracted_parts: Vec<ExtractedPart>,
    /// Where this component was extracted from.
    pub source_repos: Vec<SourceAttribution>,
    /// If the component provides a React/Dioxus/generic context, its type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_context: Option<String>,
    /// Types of contexts consumed by this component.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumed_contexts: Vec<String>,
}

/// A single abstract prop in the canonical model.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CanonicalAbstractProp {
    /// Canonical prop name (e.g. `disabled`, `variant`).
    pub canonical_name: String,
    /// Abstract type in the CAM ontology.
    pub abstract_type: AbstractPropType,
    /// Reactivity model of the prop.
    pub reactivity: AbstractReactivity,
    /// The normalised concrete type (e.g. `String`, `bool`,
    /// `enum: Default, Destructive`). Preserves original type information
    /// beyond the abstract mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concrete_type: Option<String>,
    /// Mapping from each source that declares this prop to its canonical form.
    pub sources: Vec<PropSourceMapping>,
    /// Extraction confidence in `[0.0, 1.0]`.
    pub confidence: f32,
    /// Conflicts detected for this prop across sources.
    pub conflicts: Vec<Conflict>,
}

/// A semantic event emitted by a component (extracted from callback props).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CanonicalAbstractEvent {
    /// Canonical event name (e.g. `click`, `change`).
    pub canonical_name: String,
    /// Abstract payload type carried by the event.
    pub abstract_payload: AbstractPropType,
}

/// Maps an original prop name/type from a specific source to its canonical form.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PropSourceMapping {
    /// Identifier of the source repository or file.
    pub repo_id: String,
    /// Prop name as written in the original source.
    pub original_name: String,
    /// Prop type as written in the original source.
    pub original_type: String,
}

/// A selectable sub-region of a component (slots, parts, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExtractedPart {
    /// Part name (e.g. `icon`, `label`).
    pub name: String,
    /// Whether the part can be independently replaced/selected.
    pub selectable: bool,
}

/// Records where a component was extracted from.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SourceAttribution {
    /// Repository URL, or `local` for local-directory extractions.
    pub repo_url: String,
    /// Path of the source file, relative to the scan root.
    pub file_path: String,
    /// 1-based line where the component definition starts.
    pub line_start: usize,
}

// -------------------------------------------------------------------------
// Package Manifest
// -------------------------------------------------------------------------

/// Metadata for a library of UI components (e.g. shadcn-dioxus, MUI).
///
/// A manifest is the unit consumed by the code generators and the
/// shadcn-compatible registry builder.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PackageManifest {
    /// The library name (e.g. `shadcn-dioxus`).
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
