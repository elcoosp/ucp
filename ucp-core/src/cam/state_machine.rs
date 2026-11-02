//! Behavioral state machines extracted from SMDL or LLM inference.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// An extracted state machine (e.g. from SMDL or LLM inference).
///
/// States and transitions are stored in `BTreeMap`s so serialization is
/// deterministic regardless of insertion order.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StateMachine {
    /// Identifier, usually the component name the machine belongs to.
    pub id: String,
    /// Name of the initial state.
    pub initial: String,
    /// All states keyed by name.
    pub states: BTreeMap<String, StateNode>,
}

impl StateMachine {
    /// Return the initial state node, if the machine is well-formed.
    #[must_use]
    pub fn initial_node(&self) -> Option<&StateNode> {
        self.states.get(&self.initial)
    }

    /// Resolve the transition taken from `state` when `event` fires.
    #[must_use]
    pub fn transition(&self, state: &str, event: &str) -> Option<&Transition> {
        self.states.get(state)?.on.as_ref()?.get(event)
    }

    /// Collect all side effects reachable from the initial state via a
    /// breadth-first walk, sorted and deduplicated.
    #[must_use]
    pub fn reachable_side_effects(&self) -> Vec<String> {
        let mut effects = std::collections::BTreeSet::new();
        let mut queue: std::collections::VecDeque<&str> = std::collections::VecDeque::new();
        let mut visited: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();

        if self.states.contains_key(&self.initial) {
            queue.push_back(self.initial.as_str());
            visited.insert(self.initial.as_str());
        }

        while let Some(state) = queue.pop_front() {
            let Some(node) = self.states.get(state) else {
                continue;
            };
            if let Some(on) = &node.on {
                for transition in on.values() {
                    for effect in &transition.side_effects {
                        effects.insert(effect.clone());
                    }
                    if visited.insert(transition.target.as_str()) {
                        queue.push_back(transition.target.as_str());
                    }
                }
            }
        }

        effects.into_iter().collect()
    }
}

/// A single state with outgoing transitions.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StateNode {
    /// Outgoing transitions keyed by event name, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<BTreeMap<String, Transition>>,
}

/// A transition from one state to another, optionally with side effects.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Transition {
    /// Target state name.
    pub target: String,
    /// Named side effects triggered by the transition (e.g. `+ lockScroll`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effects: Vec<String>,
}
