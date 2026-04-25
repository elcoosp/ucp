use ucp_synthesizer::extract::rust_ast::StructComponentVisitor;

const STANDARDIZED_BUTTON_SRC: &str = r#"
use leptos_shadcn_api_standards::*;

pub struct StandardizedButtonProps {
    pub id: Option<String>,
    pub disabled: Option<bool>,
    pub variant: Option<StandardVariant>,
    pub size: Option<StandardSize>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub children: Option<Children>,
}

pub struct StandardizedButton;

impl StandardizedButton {
    pub fn render(props: StandardizedButtonProps) -> impl IntoView {
        view! { <button>{}</button> }
    }
}
"#;

#[test]
fn extract_standardized_button_component() {
    let components = StructComponentVisitor::extract(STANDARDIZED_BUTTON_SRC).unwrap();
    assert_eq!(components.len(), 1);
    let comp = &components[0];
    assert_eq!(comp.name, "StandardizedButton");
    assert!(comp.line_start > 0);
    assert_eq!(comp.props.len(), 6);
    let names: Vec<&str> = comp.props.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"id"));
    assert!(names.contains(&"disabled"));
    assert!(names.contains(&"variant"));
    assert!(names.contains(&"size"));
    assert!(names.contains(&"onclick"));
    assert!(names.contains(&"children"));
    let d = comp.props.iter().find(|p| p.name == "disabled").unwrap();
    assert!(d.raw_type.contains("Option") && d.raw_type.contains("bool"));
    let o = comp.props.iter().find(|p| p.name == "onclick").unwrap();
    assert!(o.raw_type.contains("Callback"));
    let c = comp.props.iter().find(|p| p.name == "children").unwrap();
    assert!(c.raw_type.contains("Children"));
}

#[test]
fn ignore_non_props_struct() {
    let c = StructComponentVisitor::extract(r#"pub struct WidgetConfig { pub w: u32 }"#).unwrap();
    assert!(c.is_empty());
}

#[test]
fn skip_props_struct_without_render() {
    let c = StructComponentVisitor::extract(r#"pub struct OrphanProps { pub d: String }"#).unwrap();
    assert!(c.is_empty());
}

#[test]
fn extract_multiple_components() {
    let src = r#"
pub struct ButtonProps { pub l: String }
pub struct Button;
impl Button { pub fn render(props: ButtonProps) -> impl IntoView { view! { <button/> } } }
pub struct InputProps { pub p: String }
pub struct Input;
impl Input { pub fn render(props: InputProps) -> impl IntoView { view! { <input/> } } }
"#;
    let c = StructComponentVisitor::extract(src).unwrap();
    assert_eq!(c.len(), 2);
}

#[test]
fn skip_zero_param_render() {
    let src = r#"
pub struct BadProps { pub l: String }
pub struct Bad;
impl Bad { pub fn render() -> impl IntoView { view! { <div/> } } }
"#;
    let c = StructComponentVisitor::extract(src).unwrap();
    assert!(c.is_empty());
}

#[test]
fn struct_visitor_ignores_component_fn() {
    let src = r#"
#[component]
pub fn Button(#[prop(default)] disabled: bool) -> impl IntoView { view! { <button/> } }
"#;
    let c = StructComponentVisitor::extract(src).unwrap();
    assert!(c.is_empty());
}
