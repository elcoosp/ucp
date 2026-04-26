use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

/// Generate Dioxus component code from a package manifest.
pub fn generate_dioxus(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir.join("src")).map_err(ucp_core::UcpError::Io)?;

    for comp in &manifest.components {
        let raw_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let file_name = to_snake_case(raw_name);
        let file_path = dir.join("src").join(format!("{}.rs", file_name));
        let code = generate_component_code(comp);
        fs::write(&file_path, code).map_err(ucp_core::UcpError::Io)?;
    }

    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "{}"
edition = "2021"

[dependencies]
dioxus = "0.7"
"#,
        manifest.name, manifest.version
    );
    fs::write(dir.join("Cargo.toml"), cargo_toml).map_err(ucp_core::UcpError::Io)?;

    Ok(())
}

/// Simple PascalCase/kebab-case to snake_case conversion
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_lower = false;
    for c in s.chars() {
        if c == '-' || c == ' ' {
            result.push('_');
            prev_lower = false;
        } else if c.is_uppercase() {
            if prev_lower {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_lower = false;
        } else {
            result.push(c);
            prev_lower = true;
        }
    }
    result.trim_matches('_').to_string()
}

/// Generate the Rust source for a single canonical component.
fn generate_component_code(comp: &CanonicalAbstractComponent) -> String {
    let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
    let props_struct = format!("{}Props", name);

    let mut props_fields = Vec::new();

    for prop in &comp.props {
        let rust_type = concrete_to_rust_type(prop.concrete_type.as_deref(), &prop.abstract_type);
        let field_name = &prop.canonical_name;

        if prop.abstract_type == AbstractPropType::SpreadAttributes {
            props_fields.push(format!(
                "    #[props(extends = GlobalAttributes)]\n    pub {}: Vec<Attribute>,",
                field_name
            ));
            continue;
        }

        if prop.abstract_type == AbstractPropType::Renderable {
            props_fields.push(format!("    pub {}: Element,", field_name));
            continue;
        }

        if let AbstractPropType::AsyncEventHandler(_) = &prop.abstract_type {
            props_fields.push(format!(
                "    pub {}: Option<EventHandler<MouseEvent>>,",
                field_name
            ));
            continue;
        }

        if let AbstractPropType::ControlledValue(inner) = &prop.abstract_type {
            let inner_type = abstract_to_rust_type(inner);
            props_fields.push(format!("    pub {}: Signal<{}>,", field_name, inner_type));
            continue;
        }

        let default_attr = if prop.reactivity == AbstractReactivity::Static || prop.reactivity == AbstractReactivity::Uncontrolled {
            "#[props(default)] "
        } else {
            ""
        };

        props_fields.push(format!("    {}{}: {},", default_attr, field_name, rust_type));
    }

    let props_derive = if props_fields.iter().any(|f| f.contains("extends")) {
        "#[derive(Props, Clone, PartialEq)]"
    } else {
        "#[derive(Clone, PartialEq, Props)]"
    };

    format!(
        r#"use dioxus::prelude::*;

{props_derive}
pub struct {props_struct} {{
{props_body}
}}

#[component]
pub fn {name}(props: {props_struct}) -> Element {{
    rsx! {{
        // TODO: Implement {name} component
        div {{ }}
    }}
}}
"#,
        props_body = props_fields.join("\n"),
    )
}

fn abstract_to_rust_type(ty: &AbstractPropType) -> String {
    match ty {
        AbstractPropType::ControlFlag => "bool".to_string(),
        AbstractPropType::StaticValue(_) | AbstractPropType::Any => "String".to_string(),
        AbstractPropType::AsyncEventHandler(_) => "EventHandler<MouseEvent>".to_string(),
        AbstractPropType::Renderable => "Element".to_string(),
        AbstractPropType::ControlledValue(inner) => format!("Signal<{}>", abstract_to_rust_type(inner)),
        AbstractPropType::UncontrolledValue(inner) => format!("MaybeSignal<{}>", abstract_to_rust_type(inner)),
        AbstractPropType::SpreadAttributes => "Vec<Attribute>".to_string(),
    }
}

fn concrete_to_rust_type(concrete: Option<&str>, abstract_type: &AbstractPropType) -> String {
    match concrete {
        Some("bool") => "bool".to_string(),
        Some("String") | Some("&str") => "String".to_string(),
        Some("usize") | Some("i32") | Some("u32") | Some("i64") | Some("f64") => "f64".to_string(),
        Some(c) if c.starts_with("enum:") => "String".to_string(),
        Some(c) if c.contains("Fn") || c.contains("Callback") => "Option<EventHandler<MouseEvent>>".to_string(),
        Some(c) if c.contains("Element") || c.contains("VNode") => "Element".to_string(),
        _ => abstract_to_rust_type(abstract_type),
    }
}
