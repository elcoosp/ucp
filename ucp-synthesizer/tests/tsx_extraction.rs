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

#[test]
fn extract_multiline_prop_type_with_nested_braces() {
    let code = r#"
export interface TableProps {
  data: Array<{
    id: string;
    label: string;
  }>;
}
export const Table = (props: TableProps) => {
  return <table>{props.data.map(r => <tr key={r.id}>{r.label}</tr>)}</table>;
};
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);

    let table = &components[0];
    assert_eq!(table.name, "Table");
    assert_eq!(table.props.len(), 1);

    let data = &table.props[0];
    assert_eq!(data.name, "data");
    assert!(data.raw_type.contains("Array<{"));
    assert!(data.raw_type.contains("id: string"));
    assert!(data.raw_type.contains("label: string"));
}

#[test]
fn extract_export_type_interface() {
    let code = r#"
export type CardProps = {
  title: string;
  description?: string;
}
export const Card = (props: CardProps) => <div>{props.title}</div>;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);

    let card = &components[0];
    assert_eq!(card.name, "Card");
    assert_eq!(card.props.len(), 2);

    assert_eq!(card.props[0].name, "title");
    assert!(!card.props[0].is_optional);

    assert_eq!(card.props[1].name, "description");
    assert!(card.props[1].is_optional);
}

#[test]
fn extract_function_component() {
    let code = r#"
export interface InputProps {
  value: string;
}
export function Input(props: InputProps) {
  return <input value={props.value} />;
}
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].name, "Input");
    assert_eq!(components[0].props.len(), 1);
    assert_eq!(components[0].props[0].name, "value");
}

#[test]
fn extract_nested_object_prop_type() {
    let code = r#"
export interface ConfigProps {
  theme: {
    primary: string;
    secondary: string;
  };
}
export const Config = (props: ConfigProps) => null;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);

    let theme = &components[0].props[0];
    assert_eq!(theme.name, "theme");
    assert!(!theme.is_optional);
    assert!(theme.raw_type.contains("primary: string"));
    assert!(theme.raw_type.contains("secondary: string"));
}

#[test]
fn skips_union_type_without_braces() {
    let code = r#"
export type Status = "active" | "inactive" | "pending";
export const StatusBadge = (props: { status: Status }) => null;
"#;
    let components = extract_tsx_components(code).unwrap();
    // Status is a union type (no braces), so it's skipped
    // StatusBadge has no preceding Props interface
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].name, "StatusBadge");
    assert_eq!(components[0].props.len(), 0);
}

#[test]
fn handles_generic_interface() {
    let code = r#"
export interface ListProps<T> {
  items: Array<T>;
  renderItem: (item: T) => string;
}
export const List = (props: ListProps<string>) => null;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);

    let list = &components[0];
    assert_eq!(list.props.len(), 2);

    let items = &list.props[0];
    assert_eq!(items.name, "items");
    assert!(items.raw_type.contains("Array<T>"));

    let render = &list.props[1];
    assert_eq!(render.name, "renderItem");
    assert!(render.raw_type.contains("=>"));
}

#[test]
fn handles_comma_separated_props() {
    let code = r#"
export interface FormProps {
  name: string,
  email: string,
  required: boolean,
}
export const Form = (props: FormProps) => null;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].props.len(), 3);
    assert_eq!(components[0].props[0].name, "name");
    assert_eq!(components[0].props[1].name, "email");
    assert_eq!(components[0].props[2].name, "required");
}

#[test]
fn handles_readonly_modifier() {
    let code = r#"
export interface FieldProps {
  readonly id: string;
  readonly label: string;
  value: string;
}
export const Field = (props: FieldProps) => null;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].props.len(), 3);
    assert_eq!(components[0].props[0].name, "id");
    assert_eq!(components[0].props[1].name, "label");
    assert_eq!(components[0].props[2].name, "value");
}

#[test]
fn handles_record_type_with_comma() {
    let code = r#"
export interface MapProps {
  data: Record<string, number>;
}
export const MapView = (props: MapProps) => null;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);

    let data = &components[0].props[0];
    assert_eq!(data.name, "data");
    // The comma inside Record<string, number> must not split the prop
    assert_eq!(data.raw_type, "Record<string, number>");
}

#[test]
fn tracks_real_line_start() {
    let code = r#"
export interface BadgeProps {
  label: string;
}
export const Badge = (props: BadgeProps) => null;
"#;
    let components = extract_tsx_components(code).unwrap();
    // "export const Badge" is on line 5 (1-indexed: blank, interface, label, }, const)
    assert!(components[0].line_start > 0);
    assert_eq!(components[0].line_start, 5);
}

#[test]
fn extract_default_function_component() {
    let code = r#"
export interface AlertProps {
  message: string;
  visible?: boolean;
}
export default function Alert(props: AlertProps) {
  return <div>{props.message}</div>;
}
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].name, "Alert");
    assert_eq!(components[0].props.len(), 2);

    let message = components[0]
        .props
        .iter()
        .find(|p| p.name == "message")
        .unwrap();
    assert!(!message.is_optional);

    let visible = components[0]
        .props
        .iter()
        .find(|p| p.name == "visible")
        .unwrap();
    assert!(visible.is_optional);
}

#[test]
fn extract_default_arrow_skips_anonymous() {
    let code = r#"
export interface AlertProps {
  message: string;
}
export default (props: AlertProps) => <div>{props.message}</div>;
"#;
    let components = extract_tsx_components(code).unwrap();
    // Anonymous default exports have no extractable name — skipped
    assert_eq!(components.len(), 0);
}

#[test]
fn extract_react_fc_component() {
    let code = r#"
export interface CardProps {
  title: string;
  children?: React.ReactNode;
}
const Card: React.FC<CardProps> = ({ title, children }) => {
  return <div className="card">{title}{children}</div>;
};
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].name, "Card");
    assert_eq!(components[0].props.len(), 2);

    let title = components[0]
        .props
        .iter()
        .find(|p| p.name == "title")
        .unwrap();
    assert!(!title.is_optional);

    let children = components[0]
        .props
        .iter()
        .find(|p| p.name == "children")
        .unwrap();
    assert!(children.is_optional);
}

#[test]
fn extract_fc_without_react_prefix() {
    let code = r#"
export interface BadgeProps {
  label: string;
}
const Badge: FC<BadgeProps> = (props) => <span>{props.label}</span>;
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].name, "Badge");
}

#[test]
fn extract_class_component() {
    let code = r#"
export interface ButtonProps {
  label: string;
  disabled?: boolean;
}
export class Button extends React.Component<ButtonProps> {
  render() {
    return <button disabled={this.props.disabled}>{this.props.label}</button>;
  }
}
"#;
    let components = extract_tsx_components(code).unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].name, "Button");
    assert_eq!(components[0].props.len(), 2);
}
