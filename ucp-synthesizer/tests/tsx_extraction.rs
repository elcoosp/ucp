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
    assert_eq!(btn.props.len(), 3);

    let variant = btn.props.iter().find(|p| p.name == "variant").unwrap();
    assert!(variant.is_optional);
    assert!(variant.raw_type.contains("default"));

    let disabled = btn.props.iter().find(|p| p.name == "disabled").unwrap();
    assert!(disabled.is_optional);
    assert_eq!(disabled.raw_type, "boolean");

    let on_click = btn.props.iter().find(|p| p.name == "onClick").unwrap();
    assert!(on_click.is_optional);
    assert!(on_click.raw_type.contains("void"));
}

#[test]
fn extract_non_optional_props() {
    let code = r#"
export interface InputProps {
  value: string;
  label: string;
}
export const Input = (props: InputProps) => <input value={props.value} />;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    let input = &components[0];
    assert_eq!(input.name, "Input");
    assert!(!input.props.iter().any(|p| p.is_optional));
}

#[test]
fn no_components_in_empty_code() {
    let components = extract_tsx_components("").unwrap();
    assert!(components.is_empty());
}
