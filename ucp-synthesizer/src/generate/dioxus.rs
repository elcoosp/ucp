use super::common::{
    abstract_to_rust_type, concrete_to_rust_type, generate_cargo_toml, generate_props_derive,
};
use super::traits::{generate_with, CodeGenerator};
use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

pub struct DioxusGenerator;

impl CodeGenerator for DioxusGenerator {
    fn file_extension(&self) -> &str {
        "rs"
    }

    fn map_prop_type(&self, prop: &CanonicalAbstractProp) -> String {
        concrete_to_rust_type(prop.concrete_type.as_deref(), &prop.abstract_type)
    }

    fn generate_component_code(&self, comp: &CanonicalAbstractComponent) -> String {
        let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let props_struct = format!("{}Props", name);
        let mut props_fields = Vec::new();
        let mut has_spread = false;

        for prop in &comp.props {
            let rust_type = self.map_prop_type(prop);
            let field_name = &prop.canonical_name;

            if prop.abstract_type == AbstractPropType::SpreadAttributes {
                has_spread = true;
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

            let default_attr = if prop.reactivity == AbstractReactivity::Static
                || prop.reactivity == AbstractReactivity::Uncontrolled
            {
                "#[props(default)] "
            } else {
                ""
            };
            props_fields.push(format!(
                "    {}{}: {},",
                default_attr, field_name, rust_type
            ));
        }

        let props_derive = generate_props_derive(has_spread);
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

    fn write_project_files(&self, manifest: &PackageManifest, dir: &Path) -> Result<()> {
        let cargo_toml = generate_cargo_toml(
            &manifest.name,
            &manifest.version,
            &[("dioxus", r#"{ version = "0.7", features = ["router"] }"#)],
        );
        fs::write(dir.join("Cargo.toml"), cargo_toml).map_err(ucp_core::UcpError::Io)?;
        Ok(())
    }
}

pub fn generate_dioxus(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    generate_with(manifest, output_dir, &DioxusGenerator)
}

/// Public helper for the registry generator (which doesn't implement CodeGenerator).
pub fn generate_component_code_for_registry(comp: &CanonicalAbstractComponent) -> String {
    DioxusGenerator.generate_component_code(comp)
}
