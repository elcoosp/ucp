use super::common::{concrete_to_rust_type, generate_cargo_toml};
use super::traits::{generate_with, CodeGenerator};
use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

pub struct GpuiGenerator;

impl CodeGenerator for GpuiGenerator {
    fn file_extension(&self) -> &str {
        "rs"
    }

    fn map_prop_type(&self, prop: &CanonicalAbstractProp) -> String {
        concrete_to_rust_type(prop.concrete_type.as_deref(), &prop.abstract_type)
    }

    fn generate_component_code(&self, comp: &CanonicalAbstractComponent) -> String {
        let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let struct_name = name.to_string();
        let mut fields = Vec::new();
        let mut builders = Vec::new();
        let mut has_children = false;

        for prop in &comp.props {
            let rust_type = self.map_prop_type(prop);
            let field_name = &prop.canonical_name;

            if prop.abstract_type == AbstractPropType::SpreadAttributes {
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
                let inner_type = self.map_prop_type(&CanonicalAbstractProp {
                    canonical_name: String::new(),
                    abstract_type: inner.as_ref().clone(),
                    reactivity: AbstractReactivity::Static,
                    concrete_type: None,
                    sources: vec![],
                    confidence: 0.0,
                    conflicts: vec![],
                });
                fields.push(format!("    {}: Model<{}>,", field_name, inner_type));
                continue;
            }
            fields.push(format!("    {}: {},", field_name, rust_type));
            builders.push(format!(
                r#"    pub fn {name}(mut self, value: {rust_type}) -> Self {{
        self.{name} = value;
        self
    }}"#,
                name = field_name
            ));
        }

        let render_children = if has_children {
            fields.iter()
                .filter(|f| f.contains("Option<AnyElement>"))
                .map(|f| {
                    let name = f.split(':').next().unwrap().trim();
                    format!("            if let Some({name}) = self.{name}.clone() {{\n                this = this.child({name});\n            }}")
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
        }}
    }}

{builders_body}
}}

impl Render for {struct_name} {{
    fn render(&self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {{
        let mut this = div();
{render_children}
        this
    }}
}}
{parent_element_impl}
"#,
            fields_body = fields.join("\n"),
            builders_body = builders.join("\n\n"),
            render_children = render_children,
        )
    }

    fn write_project_files(&self, manifest: &PackageManifest, dir: &Path) -> Result<()> {
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
}

pub fn generate_gpui(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    generate_with(manifest, output_dir, &GpuiGenerator)
}
