use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// An extracted state machine (e.g. from SMDL or LLM inference).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    pub id: String,
    pub initial: String,
    pub states: BTreeMap<String, StateNode>,
}

/// A single state with outgoing transitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<BTreeMap<String, Transition>>,
}

/// A transition from one state to another, optionally with side effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub target: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effects: Vec<String>,
}
