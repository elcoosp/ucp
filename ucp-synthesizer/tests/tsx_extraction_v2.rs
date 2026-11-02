//! Regression tests for the 0.13.0 tokenizer-based TSX extractor.
//!
//! The 0.12 extractor split interface members with a hand-rolled character
//! counter whose angle-bracket depth went negative on `=>` arrow types,
//! silently swallowing every prop declared after an inline callback. These
//! tests pin the corrected behavior.

use ucp_synthesizer::extract::tsx_ast::extract_tsx_components;

fn prop_names(code: &str) -> Vec<String> {
    extract_tsx_components(code)
        .unwrap()
        .into_iter()
        .flat_map(|c| c.props.into_iter().map(|p| p.name))
        .collect()
}

/// Props declared *after* an inline callback type must survive extraction.
#[test]
fn props_after_arrow_callback_type_are_not_swallowed() {
    let code = r"
export interface ButtonProps {
  onClick?: (e: React.MouseEvent) => void;
  children?: React.ReactNode;
  className?: string;
}

export const Button = (props: ButtonProps) => <button />;
";
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(
        prop_names(code),
        vec!["onClick", "children", "className"],
        "props after an arrow-callback type must all be extracted"
    );
}

/// The canonical shadcn-style interface: literals, callbacks, children,
/// spread attributes — every prop present exactly once.
#[test]
fn shadcn_style_interface_extracts_all_props() {
    let code = r#"import * as React from "react"

export interface DialogProps {
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  modal?: boolean;
  children?: React.ReactNode;
}

export const Dialog = (props: DialogProps) => {
  return props.open ? <div role="dialog">{props.children}</div> : null;
};
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    let names = prop_names(code);
    for expected in ["open", "onOpenChange", "modal", "children"] {
        assert!(
            names.iter().any(|n| n == expected),
            "missing prop {expected}; got {names:?}"
        );
    }
    assert_eq!(names.len(), 4, "each prop exactly once: {names:?}");
}

/// Semicolons and braces inside string literals must not split members.
#[test]
fn string_literals_with_semicolons_and_braces_are_preserved() {
    let code = r#"
export interface BadgeProps {
  label: string;
  template: `a{b};c`;
  message?: "done; ok {yes}";
}

export const Badge = (props: BadgeProps) => null;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    let names = prop_names(code);
    assert_eq!(names, vec!["label", "template", "message"]);
    let badge = &components[0];
    let template = badge.props.iter().find(|p| p.name == "template").unwrap();
    assert_eq!(template.raw_type, "`a{b};c`");
    let message = badge.props.iter().find(|p| p.name == "message").unwrap();
    assert_eq!(message.raw_type, "\"done; ok {yes}\"");
}

/// Comments inside interface bodies are ignored, never parsed as props.
#[test]
fn comments_inside_interface_bodies_are_skipped() {
    let code = r"
export interface CardProps {
  // title: string;  -- commented out
  /* block: comment; with; semicolons */
  title: string; // trailing comment with: colon
  footer?: string;
}

export const Card = (props: CardProps) => null;
";
    assert_eq!(prop_names(code), vec!["title", "footer"]);
}

/// Interface members separated by newlines only (no semicolons) parse fine.
#[test]
fn newline_separated_members_without_semicolons() {
    let code = r#"
export interface TabsProps {
  value: string
  onValueChange?: (value: string) => void
  activationMode?: "automatic" | "manual"
}

export const Tabs = (props: TabsProps) => null;
"#;
    let names = prop_names(code);
    assert_eq!(names, vec!["value", "onValueChange", "activationMode"]);
    let components = extract_tsx_components(code).unwrap();
    let mode = components[0]
        .props
        .iter()
        .find(|p| p.name == "activationMode")
        .unwrap();
    assert_eq!(mode.raw_type, "\"automatic\"|\"manual\"");
}

/// Method-signature props are normalized to arrow-callback type text.
#[test]
fn method_signatures_become_callback_types() {
    let code = r"
export interface TriggerProps {
  onPress(e: MouseEvent): void;
  label: string;
}

export const Trigger = (props: TriggerProps) => null;
";
    let components = extract_tsx_components(code).unwrap();
    let names = prop_names(code);
    assert_eq!(names, vec!["onPress", "label"]);
    let on_press = components[0]
        .props
        .iter()
        .find(|p| p.name == "onPress")
        .unwrap();
    assert!(on_press.raw_type.contains("=>"));
    assert!(on_press.raw_type.contains("void"));
}

/// Generic interfaces and intersection types (common React patterns).
#[test]
fn generic_and_intersection_props_interfaces() {
    let code = r#"
export interface SelectProps<T = string> {
  value?: T;
  options: T[];
  onChange?: (value: T) => void;
}

export type ButtonOwnProps = {
  variant?: "solid" | "ghost";
}

export interface ButtonProps extends React.ComponentProps<"button"> {
  variant?: "solid" | "ghost";
  children?: React.ReactNode;
}

export const Button = (props: ButtonProps) => <button />;
export const Select = (props: SelectProps<string>) => <select />;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 2);
    let button = components.iter().find(|c| c.name == "Button").unwrap();
    let button_names: Vec<_> = button.props.iter().map(|p| p.name.clone()).collect();
    assert!(
        button_names.contains(&"variant".to_string()),
        "{button_names:?}"
    );
    assert!(
        button_names.contains(&"children".to_string()),
        "{button_names:?}"
    );
    let select = components.iter().find(|c| c.name == "Select").unwrap();
    let select_names: Vec<_> = select.props.iter().map(|p| p.name.clone()).collect();
    assert_eq!(select_names, vec!["value", "options", "onChange"]);
}

/// Two components in one file each bind to their own preceding interface.
#[test]
fn multiple_interface_component_pairs_per_file() {
    let code = r"
export interface AProps {
  alpha: string;
}

export interface BProps {
  beta: number;
}

export const A = (props: AProps) => null;
export const B = (props: BProps) => null;
";
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 2);
    let a = components.iter().find(|c| c.name == "A").unwrap();
    let b = components.iter().find(|c| c.name == "B").unwrap();
    assert_eq!(
        a.props.iter().map(|p| p.name.clone()).collect::<Vec<_>>(),
        vec!["alpha"]
    );
    assert_eq!(
        b.props.iter().map(|p| p.name.clone()).collect::<Vec<_>>(),
        vec!["beta"]
    );
}

/// Nested object types keep their full text with balanced braces.
#[test]
fn nested_object_types_keep_balanced_text() {
    let code = r"
export interface TableProps {
  columns: Array<{
    id: string;
    label: string;
  }>;
  rowProps?: {
    striped: boolean;
  };
}

export const Table = (props: TableProps) => null;
";
    let components = extract_tsx_components(code).unwrap();
    let names = prop_names(code);
    assert_eq!(names, vec!["columns", "rowProps"]);
    let columns = components[0]
        .props
        .iter()
        .find(|p| p.name == "columns")
        .unwrap();
    assert!(columns.raw_type.starts_with("Array<{"));
    assert!(columns.raw_type.ends_with("}>"));
    let row_props = components[0]
        .props
        .iter()
        .find(|p| p.name == "rowProps")
        .unwrap();
    assert_eq!(row_props.raw_type, "{striped:boolean;}");
}

mod proptests {
    use std::fmt::Write as _;

    use proptest::prelude::*;

    proptest! {
        /// For any generated well-formed interface with N members followed by
        /// a component, exactly N props are extracted with the declared names
        /// in declaration order.
        #[test]
        fn well_formed_interfaces_extract_exactly_declared_props(
            members in proptest::collection::vec(
                ("[a-z][a-zA-Z]{0,12}",
                 prop_oneof![
                     Just("boolean".to_string()),
                     Just("string".to_string()),
                     Just("number".to_string()),
                     Just("(e: React.MouseEvent) => void".to_string()),
                     Just("React.ReactNode".to_string()),
                     Just("\"a\" | \"b\"".to_string()),
                     Just("string[]".to_string()),
                 ]),
                0..8
            )
        ) {
            let mut body = String::new();
            for (i, (name, ty)) in members.iter().enumerate() {
                let _ = writeln!(body, "  prop{i}_{name}: {ty};");
            }
            let code = format!(
                "export interface WidgetProps {{\n{body}}}\nexport const Widget = (p: WidgetProps) => null;\n"
            );
            let components = ucp_synthesizer::extract::tsx_ast::extract_tsx_components(&code).unwrap();
            prop_assert_eq!(components.len(), 1);
            let props = &components[0].props;
            prop_assert_eq!(props.len(), members.len());
            for (i, (name, _ty)) in members.iter().enumerate() {
                prop_assert_eq!(&props[i].name, &format!("prop{i}_{name}"));
            }
        }
    }
}
