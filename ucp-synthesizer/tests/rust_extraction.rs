use ucp_synthesizer::extract::rust_ast::extract_rust_components;

const MOCK_LEPTOS_BUTTON: &str = r#"
#[component]
pub fn Button(
    #[prop(into)] variant: MaybeSignal<String>,
    #[prop(default)] disabled: bool,
) -> impl IntoView {
    view! { <button>{}</button> }
}
"#;

#[test]
fn extract_props_using_visitor_trait() {
    let components = extract_rust_components(MOCK_LEPTOS_BUTTON).unwrap();
    assert_eq!(components.len(), 1);

    let btn = &components[0];
    assert_eq!(btn.name, "Button");

    let variant_prop = btn.props.iter().find(|p| p.name == "variant").unwrap();
    assert!(variant_prop.raw_type.contains("MaybeSignal"));

    let disabled_prop = btn.props.iter().find(|p| p.name == "disabled").unwrap();
    assert!(disabled_prop.has_default);
}
