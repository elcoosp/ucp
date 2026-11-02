//! Pest-based parser turning SMDL text into typed [`SmdlComponent`]s.

use std::collections::BTreeMap;

use pest::Parser as _;
use pest_derive::Parser;

use super::SmdlComponent;

/// Derived pest parser for the SMDL grammar (`smdl.pest`).
#[derive(Parser)]
#[grammar = "src/smdl/smdl.pest"]
pub struct SmdlParser;

pub(crate) fn parse_smdl_internal(input: &str, component_id: &str) -> crate::Result<SmdlComponent> {
    let pairs = SmdlParser::parse(Rule::component, input.trim())
        .map_err(|e| crate::UcpError::Parsing(e.to_string()))?;

    let mut initial_state = String::new();
    let mut states = BTreeMap::new();

    for component_pair in pairs {
        for inner_pair in component_pair.into_inner() {
            if inner_pair.as_rule() != Rule::state {
                continue;
            }
            let mut state_name = String::new();
            let mut transitions = BTreeMap::new();

            for state_inner in inner_pair.into_inner() {
                match state_inner.as_rule() {
                    Rule::IDENT => state_name = state_inner.as_str().trim().to_string(),
                    Rule::transition => {
                        parse_transition(state_inner, &mut transitions);
                    }
                    _ => {}
                }
            }

            // The first declared state is the initial state.
            if initial_state.is_empty() {
                initial_state.clone_from(&state_name);
            }

            states.insert(
                state_name,
                super::SmdlState {
                    on: if transitions.is_empty() {
                        None
                    } else {
                        Some(transitions)
                    },
                },
            );
        }
    }

    Ok(SmdlComponent {
        id: component_id.to_string(),
        initial: initial_state,
        states,
    })
}

/// Parse one `on EVENT -> TARGET { side_effects }` pair into `transitions`,
/// keyed by the event name (not the target) so that two events may share a
/// target without colliding.
fn parse_transition(
    transition_pair: pest::iterators::Pair<'_, Rule>,
    transitions: &mut BTreeMap<String, super::SmdlTransition>,
) {
    let mut event = String::new();
    let mut target = String::new();
    let mut effects = Vec::new();
    let mut ident_index = 0;

    for t_pair in transition_pair.into_inner() {
        match t_pair.as_rule() {
            Rule::IDENT => {
                if ident_index == 0 {
                    event = t_pair.as_str().trim().to_string();
                } else {
                    target = t_pair.as_str().trim().to_string();
                }
                ident_index += 1;
            }
            Rule::side_effect => {
                let effect_str = t_pair.as_str().trim_start_matches("+ ").trim();
                effects.push(effect_str.to_string());
            }
            _ => {}
        }
    }

    if !event.is_empty() && !target.is_empty() {
        transitions.insert(
            event,
            super::SmdlTransition {
                target,
                side_effects: effects,
            },
        );
    }
}
