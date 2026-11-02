//! Abstract prop type ontology, reactivity model, and conflict types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Abstract type ontology for component props.
///
/// The ontology deliberately avoids framework names: a Leptos `RwSignal<T>`
/// and a React controlled `value` prop both reduce to
/// [`AbstractPropType::ControlledValue`].
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum AbstractPropType {
    /// Boolean flag that toggles behavior (e.g. `disabled`).
    ControlFlag,
    /// Value controlled by the parent/state (e.g. `RwSignal<T>`, controlled `value`).
    ControlledValue(Box<AbstractPropType>),
    /// Value with an uncontrolled default (e.g. `Option<T>`, `MaybeSignal<T>`).
    UncontrolledValue(Box<AbstractPropType>),
    /// Static, non-reactive value (e.g. `String`, enums).
    StaticValue(Box<AbstractPropType>),
    /// Event handler callback; payload describes the argument types.
    AsyncEventHandler(Vec<AbstractPropType>),
    /// Renderable children/slots (e.g. `Children`, `ReactNode`).
    Renderable,
    /// Represents a spread of HTML attributes (e.g. `Vec<Attribute>` in Dioxus).
    #[serde(rename = "spreadAttributes")]
    SpreadAttributes,
    /// Type could not be classified.
    Any,
}

impl AbstractPropType {
    /// Human-readable short name for this abstract type, used by exporters
    /// and generators that need a compact label.
    #[must_use]
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::ControlFlag => "controlFlag",
            Self::ControlledValue(_) => "controlledValue",
            Self::UncontrolledValue(_) => "uncontrolledValue",
            Self::StaticValue(_) => "staticValue",
            Self::AsyncEventHandler(_) => "asyncEventHandler",
            Self::Renderable => "renderable",
            Self::SpreadAttributes => "spreadAttributes",
            Self::Any => "any",
        }
    }

    /// Strip any controlled/uncontrolled/static wrapper and return the
    /// innermost payload type.
    #[must_use]
    pub fn innermost(&self) -> &Self {
        match self {
            Self::ControlledValue(inner)
            | Self::UncontrolledValue(inner)
            | Self::StaticValue(inner) => inner.innermost(),
            other => other,
        }
    }
}

/// Reactivity model for a prop.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum AbstractReactivity {
    /// Two-way bound to parent state.
    Controlled,
    /// Read-only value the component owns locally.
    Uncontrolled,
    /// Set once and never changes.
    Static,
    /// Leptos-style `MaybeSignal` that may or may not be reactive.
    MaybeSignal,
    /// GPUI-style entity-backed state.
    EntityBacked,
}

/// A detected conflict between equivalent props across codebases.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Conflict {
    /// Unique conflict identifier (e.g. `conf_001`, `cross_004`).
    pub id: String,
    /// Spec field the conflict applies to (e.g. `props.disabled`).
    pub field: String,
    /// Sources where the conflicting value is present.
    pub present_in: Vec<String>,
    /// Sources where the prop is absent.
    pub absent_in: Vec<String>,
    /// Confidence that this is a true conflict, in `[0.0, 1.0]`.
    pub confidence: f32,
    /// Suggested resolution strategy.
    pub resolution_suggestion: ResolutionStrategy,
}

/// Suggested resolution for a conflict.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ResolutionStrategy {
    /// Keep the value declared by the majority of sources.
    IncludeMajority,
    /// Keep the value but scope it to a named platform profile.
    ScopeToProfile(String),
    /// Escalate to a human spec editor.
    FlagForHumanReview,
}
