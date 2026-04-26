use ucp_synthesizer::extract::rust_ast::extract_rust_components;

const DIOXUS_WITH_CONTEXT: &str = r#"
#[derive(Props)]
pub struct DialogProps {
    open: bool,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    use_context_provider(|| DialogContext {
        open: Signal::new(props.open),
        set_open: Callback::new(|_| {}),
        content_id: String::new(),
        title_id: String::new(),
        description_id: String::new(),
    });

    rsx! { {props.children} }
}
"#;

#[test]
fn dioxus_extracts_provided_context() {
    let components = extract_rust_components(DIOXUS_WITH_CONTEXT).unwrap();
    assert_eq!(components.len(), 1);
    let comp = &components[0];
    assert_eq!(comp.provided_context, Some("DialogContext".to_string()));
    assert!(comp.consumed_contexts.is_empty());
}
