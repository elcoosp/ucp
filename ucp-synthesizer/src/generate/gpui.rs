use super::common::{concrete_to_rust_type, generate_cargo_toml, to_snake_case};
use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

/// Generate GPUI component code from a package manifest.
pub fn generate_gpui(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir.join("src")).map_err(ucp_core::UcpError::Io)?;

    for comp in &manifest.components {
        let raw_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let file_name = to_snake_case(raw_name);
        let file_path = dir.join("src").join(format!("{}.rs", file_name));
        let code = generate_component_code(comp);
        fs::write(&file_path, code).map_err(ucp_core::UcpError::Io)?;
    }

    let cargo_toml = generate_cargo_toml(
        &manifest.name,
        &manifest.version,
        &[(
            "gpui",
            r#"{ git = "https://github.com/zed-industries/zed", package = "gpui" }"#,
        )],
    );
    fs::write(dir.join("Cargo.toml"), cargo_toml).map_err(ucp_core::UcpError::Io)?;

    Ok(())
}

fn generate_component_code(comp: &CanonicalAbstractComponent) -> String {
    let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
    let struct_name = name.to_string();
    let mut fields = Vec::new();
    let mut builders = Vec::new();
    let mut has_children = false;

    for prop in &comp.props {
        let rust_type = concrete_to_rust_type(prop.concrete_type.as_deref(), &prop.abstract_type);
        let field_name = &prop.canonical_name;

        if prop.abstract_type == AbstractPropType::SpreadAttributes {
            // GPUI doesn't have a direct spread equivalent; emit as attributes map
            continue;
        }

        if prop.abstract_type == AbstractPropType::Renderable {
            has_children = true;
            fields.push(format!("    {}: Option<AnyElement>,", field_name));
            builders.push(format!(
                r#"    pub fn {name}(mut self, {name}: impl IntoElement) -> Self {{
        self.{name} = Some({name}.into_any_element());
        self
    }}"#,
                name = field_name
            ));
            continue;
        }

        if let AbstractPropType::AsyncEventHandler(_) = &prop.abstract_type {
            fields.push(format!(
                "    {}: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,",
                field_name
            ));
            builders.push(format!(
                r#"    pub fn {name}(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {{
        self.{name} = Some(Box::new(handler));
        self
    }}"#,
                name = field_name
            ));
            continue;
        }

        if let AbstractPropType::ControlledValue(inner) = &prop.abstract_type {
            let inner_type = abstract_to_rust_type(inner);
            fields.push(format!("    {}: Model<{}>,", field_name, inner_type));
            continue;
        }

        fields.push(format!("    {}: {},", field_name, rust_type));
        builders.push(format!(
            r#"    pub fn {name}(mut self, value: {rust_type}) -> Self {{
        self.{name} = value;
        self
    }}"#,
            name = field_name,
            rust_type = rust_type
        ));
    }

    let render_children = if has_children {
        fields.iter()
            .filter(|f| f.contains("Option<AnyElement>"))
            .map(|f| {
                let name = f.split(':').next().unwrap().trim();
                format!("            if let Some({name}) = self.{name}.clone() {{\n                this = this.child({name});\n            }}", name = name)
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    };

    let parent_element_impl = if has_children {
        format!(
            r#"
impl ParentElement for {struct_name} {{
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {{
        for element in elements {{
            self.children = Some(element);
        }}
    }}
}}"#
        )
    } else {
        String::new()
    };

    format!(
        r#"use gpui::*;

#[derive(IntoElement)]
pub struct {struct_name} {{
{fields_body}
}}

impl {struct_name} {{
    pub fn new() -> Self {{
        Self {{
            // TODO: Initialize fields with defaults
            // All fields are expected to be set via builder methods
        }}
    }}

{builders_body}
}}

impl Render for {struct_name} {{
    fn render(&self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {{
        let mut this = div();
{render_children}
        // TODO: Implement {struct_name} rendering
        this
    }}
}}
{parent_element_impl}
"#,
        fields_body = fields.join("\n"),
        builders_body = builders.join("\n\n"),
        render_children = render_children,
        parent_element_impl = parent_element_impl,
    )
}

fn abstract_to_rust_type(ty: &AbstractPropType) -> String {
    match ty {
        AbstractPropType::ControlFlag => "bool".to_string(),
        AbstractPropType::StaticValue(_) | AbstractPropType::Any => "SharedString".to_string(),
        AbstractPropType::AsyncEventHandler(_) => "EventHandler<MouseEvent>".to_string(),
        AbstractPropType::Renderable => "AnyElement".to_string(),
        AbstractPropType::ControlledValue(inner) => {
            format!("Model<{}>", abstract_to_rust_type(inner))
        }
        AbstractPropType::UncontrolledValue(inner) => {
            format!("Model<{}>", abstract_to_rust_type(inner))
        }
        AbstractPropType::SpreadAttributes => "Vec<Attribute>".to_string(),
    }
}
