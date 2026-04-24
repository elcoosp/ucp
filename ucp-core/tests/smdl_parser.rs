use ucp_core::smdl::parse_smdl;

#[test]
fn parse_simple_dialog_smdl_to_typed() {
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

    assert_eq!(result.id, "ucp-smdl");
    assert_eq!(result.initial, "Closed");

    assert!(result.states.contains_key("Closed"));
    assert!(result.states.contains_key("Open"));

    let closed_on = result.states["Closed"].on.as_ref().unwrap();
    assert!(closed_on.contains_key("Open"));
    assert_eq!(closed_on["Open"].target, "Open");

    let side_effects = &closed_on["Open"].side_effects;
    assert!(
        side_effects.iter().any(|e| e.contains("focus: move_to")),
        "Missing 'focus: move_to' side effect, got: {:?}",
        side_effects
    );

    let open_on = result.states["Open"].on.as_ref().unwrap();
    assert_eq!(open_on["Closed"].target, "Closed");
}
