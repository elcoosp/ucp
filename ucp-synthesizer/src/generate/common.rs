use std::fmt::Write as _;

use ucp_core::cam::AbstractPropType;

/// Convert `PascalCase` or kebab-case to `snake_case`
#[must_use]
pub fn to_snake_case(s: &str) -> String {
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
            for lower in c.to_lowercase() {
                result.push(lower);
            }
            prev_lower = false;
        } else {
            result.push(c);
            prev_lower = true;
        }
    }
    result.trim_matches('_').to_string()
}

/// Map abstract type to a Rust type string (framework-agnostic default)
#[must_use]
pub fn abstract_to_rust_type(ty: &AbstractPropType) -> String {
    match ty {
        AbstractPropType::ControlFlag => "bool".to_string(),
        AbstractPropType::StaticValue(_) | AbstractPropType::Any => "String".to_string(),
        AbstractPropType::AsyncEventHandler(_) => "EventHandler<MouseEvent>".to_string(),
        AbstractPropType::Renderable => "Element".to_string(),
        AbstractPropType::ControlledValue(inner) => {
            format!("Signal<{}>", abstract_to_rust_type(inner))
        }
        AbstractPropType::UncontrolledValue(inner) => {
            format!("MaybeSignal<{}>", abstract_to_rust_type(inner))
        }
        AbstractPropType::SpreadAttributes => "Vec<Attribute>".to_string(),
    }
}

/// Map concrete type to Rust type, falling back to abstract type
#[must_use]
pub fn concrete_to_rust_type(concrete: Option<&str>, abstract_type: &AbstractPropType) -> String {
    match concrete {
        Some("bool") => "bool".to_string(),
        Some("String" | "&str") => "String".to_string(),
        Some("usize" | "i32" | "u32" | "i64" | "f64") => concrete.unwrap_or("f64").to_string(),
        Some(c) if c.starts_with("enum:") => "String".to_string(),
        Some(c) if c.contains("Fn") || c.contains("Callback") => {
            "Option<EventHandler<MouseEvent>>".to_string()
        }
        Some(c) if c.contains("Element") || c.contains("VNode") => "Element".to_string(),
        _ => abstract_to_rust_type(abstract_type),
    }
}

/// Generate the props derive attribute based on whether spread attributes are present
#[must_use]
pub fn generate_props_derive(has_spread: bool) -> &'static str {
    if has_spread {
        "#[derive(Props, Clone, PartialEq)]"
    } else {
        "#[derive(Clone, PartialEq, Props)]"
    }
}

/// Generate Cargo.toml content for a generated project (edition 2024)
#[must_use]
pub fn generate_cargo_toml(name: &str, version: &str, dependencies: &[(&str, &str)]) -> String {
    let mut deps = String::new();
    for (dep_name, dep_version) in dependencies {
        let _ = writeln!(deps, "{dep_name} = {dep_version}");
    }
    format!(
        r#"[package]
name = "{name}"
version = "{version}"
edition = "2024"

[dependencies]
{deps}"#
    )
}
