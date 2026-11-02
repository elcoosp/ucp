use super::common::{concrete_to_rust_type, generate_cargo_toml};
use super::traits::{generate_with, CodeGenerator};
use std::fs;
use std::path::Path;
use ucp_core::cam::{
    AbstractPropType, AbstractReactivity, CanonicalAbstractComponent, CanonicalAbstractProp,
    PackageManifest,
};
use ucp_core::Result;

/// Generates GPUI (Rust) component code from a manifest.
pub struct GpuiGenerator;

impl CodeGenerator for GpuiGenerator {
    fn file_extension(&self) -> &'static str {
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
            if prop.abstract_type == AbstractPropType::SpreadAttributes {
                continue;
            }
            let (field, builder, is_children) =
                gpui_field_and_builder(self, prop, &self.map_prop_type(prop));
            if is_children {
                has_children = true;
            }
            if let Some(f) = field {
                fields.push(f);
            }
            if let Some(b) = builder {
                builders.push(b);
            }
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
                r"
impl ParentElement for {struct_name} {{
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {{
        for element in elements {{
            self.children = Some(element);
        }}
    }}
}}"
            )
        } else {
            String::new()
        };

        format!(
            r"use gpui::*;

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
",
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

/// Generate a full GPUI component library into `output_dir`.
///
/// # Errors
///
/// Errors on file-create or file-write failures.
pub fn generate_gpui(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    generate_with(manifest, output_dir, &GpuiGenerator)
}

/// Build the struct field declaration and builder method for one prop.
///
/// Returns `(field, builder, is_children)`. `field`/`builder` are `None`
/// for prop kinds that need no emitted code (spread attributes).
fn gpui_field_and_builder(
    gen: &GpuiGenerator,
    prop: &CanonicalAbstractProp,
    rust_type: &str,
) -> (Option<String>, Option<String>, bool) {
    let field_name = &prop.canonical_name;
    match &prop.abstract_type {
        AbstractPropType::SpreadAttributes => (None, None, false),
        AbstractPropType::Renderable => (
            Some(format!("    {field_name}: Option<AnyElement>,")),
            Some(format!(
                r"    pub fn {field_name}(mut self, {field_name}: impl IntoElement) -> Self {{
        self.{field_name} = Some({field_name}.into_any_element());
        self
    }}"
            )),
            true,
        ),
        AbstractPropType::AsyncEventHandler(_) => (
            Some(format!(
                "    {field_name}: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,"
            )),
            Some(format!(
                r"    pub fn {field_name}(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {{
        self.{field_name} = Some(Box::new(handler));
        self
    }}"
            )),
            false,
        ),
        AbstractPropType::ControlledValue(inner) => {
            let inner_type = gen.map_prop_type(&CanonicalAbstractProp {
                canonical_name: String::new(),
                abstract_type: inner.as_ref().clone(),
                reactivity: AbstractReactivity::Static,
                concrete_type: None,
                sources: vec![],
                confidence: 0.0,
                conflicts: vec![],
            });
            (
                Some(format!("    {field_name}: Model<{inner_type}>")),
                None,
                false,
            )
        }
        _ => (
            Some(format!("    {field_name}: {rust_type},")),
            Some(format!(
                r"    pub fn {field_name}(mut self, value: {rust_type}) -> Self {{
        self.{field_name} = value;
        self
    }}"
            )),
            false,
        ),
    }
}
