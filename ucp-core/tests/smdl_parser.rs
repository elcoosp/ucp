use ucp_core::smdl::parse_smdl;

#[test]
fn parse_simple_dialog_smdl_to_json() {
    let input = r#"
    component Dialog {
        state Closed {
            on OPEN -> Open {
                + focus: move_to [part=content]
                + scroll: lock
            }
        }
        state Open {
            on CLOSE -> Closed { + focus: return }
        }
    }
    "#;

    let result = parse_smdl(input).unwrap();

    // Structural assertions on the serde_json Value – no string formatting dependency
    assert_eq!(result["id"], "ucp-smdl", "id mismatch");
    assert_eq!(result["initial"], "Closed", "initial state mismatch");

    // Verify both states exist
    assert!(result["states"]["Closed"].is_object(), "Closed state missing");
    assert!(result["states"]["Open"].is_object(), "Open state missing");

    // Verify Closed -> Open transition
    let closed_transitions = &result["states"]["Closed"]["on"];
    assert!(closed_transitions["Open"].is_object(), "Closed->Open transition missing");
    assert_eq!(closed_transitions["Open"]["target"], "Open");

    // Verify side effects contain the focus directive
    let side_effects = closed_transitions["Open"]["sideEffects"].as_array()
        .expect("sideEffects should be an array");
    assert!(side_effects.iter().any(|e| e.as_str().unwrap().contains("focus: move_to")),
        "Missing 'focus: move_to' side effect, got: {:?}", side_effects);

    // Verify Open -> Closed transition
    let open_transitions = &result["states"]["Open"]["on"];
    assert_eq!(open_transitions["Closed"]["target"], "Closed");
}
