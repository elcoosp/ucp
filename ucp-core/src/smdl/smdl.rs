use pest::iterators::Pairs;
use pest::Parser;
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Parser)]
#[grammar = "smdl.pest"]
pub struct SmdlParser;

pub(crate) fn parse_smdl_internal(input: &str) -> crate::Result<Value> {
    let pairs = SmdlParser::parse(Rule::component, input)
        .map_err(|e| crate::UcpError::Parsing(e.to_string()))?;

    let mut initial_state = String::new();
    let mut states_json = BTreeMap::new();

    for pair in pairs {
        if pair.as_rule() == Rule::state {
            let mut state_name = String::new();
            let mut transitions = BTreeMap::new();

            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::IDENT => state_name = inner_pair.as_str().to_string(),
                    Rule::transition => {
                        let mut target = String::new();
                        let mut effects = Vec::new();
                        let mut is_target = true;

                        for t_pair in inner_pair.into_inner() {
                            match t_pair.as_rule() {
                                Rule::IDENT if is_target => {
                                    target = t_pair.as_str().to_string();
                                    is_target = false;
                                }
                                Rule::side_effect => {
                                    let effect_str = t_pair.as_str().trim_start_matches("+ ").trim();
                                    effects.push(effect_str.to_string());
                                }
                                _ => {}
                            }
                        }

                        transitions.insert(target, json!({
                            "target": target,
                            "sideEffects": effects
                        }));
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

    Ok(json!({
        "id": "ucp-smdl",
        "initial": initial_state,
        "states": states_json
    }))
}
