//! SMDL — the State Machine Definition Language.
//!
//! SMDL is a small DSL for describing UI component state machines:
//!
//! ```text
//! dialog {
//!     closed {
//!         on_open + lockScroll => open
//!     }
//!     open {
//!         on_close + unlockScroll => closed
//!     }
//! }
//! ```
//!
//! The grammar is implemented with `pest` (see `smdl.pest`). Parsing
//! produces a typed [`SmdlComponent`] which can be converted into a CAM
//! [`StateMachine`](ucp_core::cam::StateMachine).

use std::collections::BTreeMap;

use crate::cam::{StateMachine, StateNode, Transition};

mod parser;

/// Parsed SMDL component with typed fields.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmdlComponent {
    /// Identifier given when parsing (usually the component name).
    pub id: String,
    /// Name of the first declared state (the initial state).
    pub initial: String,
    /// All states keyed by name, in declaration order.
    pub states: BTreeMap<String, SmdlState>,
}

/// A single SMDL state with its outgoing transitions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmdlState {
    /// Transitions keyed by event name, if the state has any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on: Option<BTreeMap<String, SmdlTransition>>,
}

/// A transition with an optional list of named side effects.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmdlTransition {
    /// Target state name.
    pub target: String,
    /// Side effects declared with `+` (e.g. `+ lockScroll`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effects: Vec<String>,
}

/// Parse an SMDL string into a typed [`SmdlComponent`].
///
/// # Arguments
///
/// * `input` - the SMDL source text
/// * `component_id` - identifier to attach to the resulting component
///
/// # Errors
///
/// Returns [`UcpError::Parsing`](crate::UcpError::Parsing) if the input is
/// not valid SMDL according to the grammar.
pub fn parse_smdl(input: &str, component_id: &str) -> crate::Result<SmdlComponent> {
    parser::parse_smdl_internal(input, component_id)
}

impl From<SmdlComponent> for StateMachine {
    fn from(smdl: SmdlComponent) -> Self {
        let states = smdl
            .states
            .into_iter()
            .map(|(state_name, state_value)| {
                let transitions: BTreeMap<String, Transition> = state_value
                    .on
                    .map(|on_map| {
                        on_map
                            .into_iter()
                            .map(|(event_name, trans_value)| {
                                (
                                    event_name,
                                    Transition {
                                        target: trans_value.target,
                                        side_effects: trans_value.side_effects,
                                    },
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                (
                    state_name,
                    StateNode {
                        on: if transitions.is_empty() {
                            None
                        } else {
                            Some(transitions)
                        },
                    },
                )
            })
            .collect();
        StateMachine {
            id: smdl.id,
            initial: smdl.initial,
            states,
        }
    }
}
