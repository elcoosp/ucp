use serde::{Deserialize, Serialize};

/// Abstract type ontology for component props.
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

/// Reactivity model for a prop.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AbstractReactivity {
    Controlled,
    Uncontrolled,
    Static,
    MaybeSignal,
    EntityBacked,
}

/// A detected conflict between equivalent props across codebases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub id: String,
    pub field: String,
    pub present_in: Vec<String>,
    pub absent_in: Vec<String>,
    pub confidence: f32,
    pub resolution_suggestion: super::ResolutionStrategy,
}

/// Suggested resolution for a conflict.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ResolutionStrategy {
    IncludeMajority,
    ScopeToProfile(String),
    FlagForHumanReview,
}
