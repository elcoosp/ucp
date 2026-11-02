//! Integration tests for the SMDL parser.

use ucp_core::cam::StateMachine;
use ucp_core::smdl::parse_smdl;

#[test]
fn parse_simple_dialog_smdl_to_typed() {
    let input = r"
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
    ";
    let result = parse_smdl(input, "test-dialog").unwrap();
    assert_eq!(result.id, "test-dialog");
    assert_eq!(result.initial, "Closed");
    assert!(result.states.contains_key("Closed"));
    assert!(result.states.contains_key("Open"));
    // Transitions are keyed by EVENT name.
    let closed_on = result.states["Closed"].on.as_ref().unwrap();
    assert!(closed_on.contains_key("OPEN"));
    assert_eq!(closed_on["OPEN"].target, "Open");
    let side_effects = &closed_on["OPEN"].side_effects;
    assert!(side_effects.iter().any(|e| e.contains("focus: move_to")));
    let open_on = result.states["Open"].on.as_ref().unwrap();
    assert_eq!(open_on["CLOSE"].target, "Closed");
}

#[test]
fn two_events_may_target_the_same_state() {
    let input = r"
    component Dialog {
        state Closed {
            on OPEN -> Open {}
            on FOCUS -> Open {}
        }
        state Open {}
    }
    ";
    let result = parse_smdl(input, "dialog").unwrap();
    let closed_on = result.states["Closed"].on.as_ref().unwrap();
    assert_eq!(closed_on.len(), 2);
    assert_eq!(closed_on["OPEN"].target, "Open");
    assert_eq!(closed_on["FOCUS"].target, "Open");
}

#[test]
fn state_without_transitions_has_no_on_map() {
    let input = r"
    component Toggle {
        state On {}
        state Off { on SWITCH -> On {} }
    }
    ";
    let result = parse_smdl(input, "toggle").unwrap();
    assert!(result.states["On"].on.is_none());
    assert!(result.states["Off"].on.is_some());
}

#[test]
fn empty_input_is_a_parse_error() {
    assert!(parse_smdl("", "x").is_err());
    assert!(parse_smdl("not smdl at all", "x").is_err());
}

#[test]
fn smdl_roundtrips_into_cam_state_machine() {
    let input = r"
    component Accordion {
        state Collapsed {
            on EXPAND + announce: open => Expanded
        }
        state Expanded {
            on COLLAPSE => Collapsed
        }
    }
    ";
    // NOTE: the grammar uses `->` for transitions, so the `=>` above must fail.
    assert!(parse_smdl(input, "accordion").is_err());
}

#[test]
fn state_machine_helpers_navigate_transitions() {
    let smdl = parse_smdl(
        r"
        component Wizard {
            state Step1 { on NEXT -> Step2 { + save: draft } }
            state Step2 { on NEXT -> Step3 { + save: final } on BACK -> Step1 {} }
            state Step3 {}
        }
        ",
        "wizard",
    )
    .unwrap();

    let machine = StateMachine::from(smdl);

    assert_eq!(machine.id, "wizard");
    assert_eq!(machine.initial, "Step1");
    assert!(machine.initial_node().is_some());
    assert!(machine.transition("Step1", "NEXT").is_some());
    assert!(machine.transition("Step1", "NOPE").is_none());
    assert!(machine.transition("Nowhere", "NEXT").is_none());
    let effects = machine.reachable_side_effects();
    assert!(effects.contains(&"save: draft".to_string()));
    assert!(effects.contains(&"save: final".to_string()));
    assert_eq!(effects.len(), 2);
}
