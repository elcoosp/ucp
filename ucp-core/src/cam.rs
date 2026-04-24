use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAbstractComponent {
    pub id: String,
    pub semantic_fingerprint: SemanticFingerprint,
    pub props: Vec<CanonicalAbstractProp>,
    pub events: Vec<CanonicalAbstractEvent>,
    pub extracted_state_machine: Option<StateMachine>,
    pub extracted_parts: Vec<ExtractedPart>,
    pub source_repos: Vec<SourceAttribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticFingerprint {
    pub purpose_hash: String,
    pub normalized_prop_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAbstractProp {
    pub canonical_name: String,
    pub abstract_type: AbstractPropType,
    pub reactivity: AbstractReactivity,
    pub sources: Vec<PropSourceMapping>,
    pub confidence: f32,
    pub conflicts: Vec<Conflict>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AbstractPropType {
    ControlFlag,
    ControlledValue(Box<AbstractPropType>),
    UncontrolledValue(Box<AbstractPropType>),
    StaticValue(Box<AbstractPropType>),
    AsyncEventHandler(Vec<AbstractPropType>),
    Renderable,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AbstractReactivity {
    Controlled,
    Uncontrolled,
    Static,
    MaybeSignal,
    EntityBacked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropSourceMapping {
    pub repo_id: String,
    pub original_name: String,
    pub original_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub id: String,
    pub field: String,
    pub present_in: Vec<String>,
    pub absent_in: Vec<String>,
    pub confidence: f32,
    pub resolution_suggestion: ResolutionStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ResolutionStrategy {
    IncludeMajority,
    ScopeToProfile(String),
    FlagForHumanReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAbstractEvent {
    pub canonical_name: String,
    pub abstract_payload: AbstractPropType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedPart {
    pub name: String,
    pub selectable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAttribution {
    pub repo_url: String,
    pub file_path: String,
    pub line_start: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    pub id: String,
    pub initial: String,
    pub states: BTreeMap<String, StateNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<BTreeMap<String, Transition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub target: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effects: Vec<String>,
}
