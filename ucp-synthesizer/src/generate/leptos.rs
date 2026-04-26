use super::common::{
    abstract_to_rust_type, concrete_to_rust_type, generate_cargo_toml, generate_props_derive,
};
use super::traits::{generate_with, CodeGenerator};
use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

pub struct LeptosGenerator;

impl CodeGenerator for LeptosGenerator {
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
        let mut fn_params: Vec<String> = Vec::new();
        let mut has_spread = false;

        for prop in &comp.props {
            let rust_type = self.map_prop_type(prop);
            let field_name = &prop.canonical_name;

            if prop.abstract_type == AbstractPropType::SpreadAttributes {
                has_spread = true;
                props_fields.push(format!(
                    "#[prop(attrs)]\n    pub {}: Vec<Attribute>,",
                    field_name
                ));
                fn_params.push(format!("#[prop(attrs)] {}: Vec<Attribute>", field_name));
                continue;
            }
            if prop.abstract_type == AbstractPropType::Renderable {
                props_fields.push(format!("    pub {}: Children,", field_name));
                fn_params.push(format!("    {}: Children", field_name));
                continue;
            }
            if let AbstractPropType::AsyncEventHandler(_) = &prop.abstract_type {
                props_fields.push(format!(
                    "    pub {}: Option<Callback<MouseEvent>>,",
                    field_name
                ));
                fn_params.push(format!("    {}: Option<Callback<MouseEvent>>", field_name));
                continue;
            }
            if let AbstractPropType::ControlledValue(inner) = &prop.abstract_type {
                let inner_type = abstract_to_rust_type(inner).replace("Signal<", "RwSignal<");
                props_fields.push(format!("    pub {}: {},", field_name, inner_type));
                fn_params.push(format!("    {}: {}", field_name, inner_type));
                continue;
            }
            if let AbstractPropType::UncontrolledValue(inner) = &prop.abstract_type {
                let inner_type = abstract_to_rust_type(inner);
                props_fields.push(format!("    pub {}: {},", field_name, inner_type));
                fn_params.push(format!("    {}: {}", field_name, inner_type));
                continue;
            }

            let default_attr = if prop.reactivity == AbstractReactivity::Static
                || prop.reactivity == AbstractReactivity::Uncontrolled
            {
                "#[prop(default)] "
            } else {
                ""
            };
            props_fields.push(format!(
                "    {}{}: {},",
                default_attr, field_name, rust_type
            ));
            fn_params.push(format!("    {}{}: {}", default_attr, field_name, rust_type));
        }

        let props_derive = generate_props_derive(has_spread);
        format!(
            r#"use leptos::*;

{props_derive}
pub struct {props_struct} {{
{props_body}
}}

#[component]
pub fn {name}(
{fn_params}
) -> impl IntoView {{
    view! {{ <div></div> }}
}}
"#,
            props_body = props_fields.join("\n"),
            fn_params = fn_params.join(",\n"),
        )
    }

    fn write_project_files(&self, manifest: &PackageManifest, dir: &Path) -> Result<()> {
        let cargo_toml = generate_cargo_toml(
            &manifest.name,
            &manifest.version,
            &[("leptos", r#"{ version = "0.7", features = ["csr"] }"#)],
        );
        fs::write(dir.join("Cargo.toml"), cargo_toml).map_err(ucp_core::UcpError::Io)?;
        Ok(())
    }
}

pub fn generate_leptos(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    generate_with(manifest, output_dir, &LeptosGenerator)
}
