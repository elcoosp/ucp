use pest::Parser;
use pest_derive::Parser;
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Parser)]
#[grammar = "src/smdl/smdl.pest"]
pub struct SmdlParser;

pub(crate) fn parse_smdl_internal(input: &str) -> crate::Result<Value> {
    let pairs = SmdlParser::parse(Rule::component, input.trim())
        .map_err(|e| crate::UcpError::Parsing(e.to_string()))?;

    let mut initial_state = String::new();
    let mut states_json = BTreeMap::new();

    for component_pair in pairs {
        for inner_pair in component_pair.into_inner() {
            if inner_pair.as_rule() == Rule::state {
                let mut state_name = String::new();
                let mut transitions = BTreeMap::new();

                for state_inner in inner_pair.into_inner() {
                    match state_inner.as_rule() {
                        Rule::IDENT => state_name = state_inner.as_str().to_string(),
                        Rule::transition => {
                            let mut target = String::new();
                            let mut effects = Vec::new();
                            // Grammar: "on" IDENT "->" IDENT "{" side_effect* "}"
                            // First IDENT = event name, second IDENT = target state
                            let mut ident_index = 0;

                            for t_pair in state_inner.into_inner() {
                                match t_pair.as_rule() {
                                    Rule::IDENT => {
                                        if ident_index == 1 {
                                            target = t_pair.as_str().to_string();
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

                            if !target.is_empty() {
                                transitions.insert(target.clone(), json!({
                                    "target": target,
                                    "sideEffects": effects
                                }));
                            }
                        }
                        _ => {}
                    }
                }

                if initial_state.is_empty() {
                    initial_state = state_name.clone();
                }

                states_json.insert(state_name, json!({ "on": transitions }));
            }
        }
    }

    Ok(json!({
        "id": "ucp-smdl",
        "initial": initial_state,
        "states": states_json
    }))
}
