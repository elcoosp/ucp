use ucp_synthesizer::extract::rust_ast::extract_rust_components;

const DIOXUS_BUTTON: &str = r#"
#[derive(Props)]
pub struct ButtonProps {
    #[props(default)]
    disabled: bool,
    label: String,
    variant: ButtonVariant,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! { <button>{props.label}</button> }
}
"#;

#[test]
fn dioxus_component_extracts_props_and_spread() {
    let components = extract_rust_components(DIOXUS_BUTTON).unwrap();
    assert_eq!(components.len(), 1);
    let comp = &components[0];
    assert_eq!(comp.name, "Button");
    assert!(comp.is_struct_pattern);

    let props = &comp.props;
    assert_eq!(props.len(), 4);

    let disabled = props.iter().find(|p| p.name == "disabled").unwrap();
    assert!(disabled.has_default);
    assert!(!disabled.is_spread_attributes);

    let attr = props.iter().find(|p| p.name == "attributes").unwrap();
    assert!(attr.is_spread_attributes);
    assert!(!attr.is_event);
}
