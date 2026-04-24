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

    let json_result = parse_smdl(input).unwrap();
    let json = serde_json::to_string_pretty(&json_result).unwrap();

    assert!(json.contains("\"initial\": \"Closed\""));
    assert!(json.contains("\"target\": \"Open\""));
    assert!(json.contains("\"focus: move_to [part=content]\""));
}
