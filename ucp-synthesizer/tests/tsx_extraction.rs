use ucp_synthesizer::extract::tsx_ast::extract_tsx_components;

const MOCK_REACT_BUTTON: &str = r#"
export interface ButtonProps {
  variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link";
  disabled?: boolean;
  onClick?: () => void;
}

export const Button = (props: ButtonProps) => {
  return <button disabled={props.disabled}>{props.variant ?? "default"}</button>;
};
"#;

#[test]
fn extract_props_from_react_tsx() {
    let components = extract_tsx_components(MOCK_REACT_BUTTON).unwrap();
    assert_eq!(components.len(), 1);

    let btn = &components[0];
    assert_eq!(btn.name, "Button");

    let variant_prop = btn.props.iter().find(|p| p.name == "variant").unwrap();
    assert!(variant_prop.is_some());

    let disabled_prop = btn.props.iter().find(|p| p.name == "disabled").unwrap();
    assert!(disabled_prop.is_some());
}
