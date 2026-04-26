use super::traits::{generate_with, CodeGenerator};
use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

pub struct ReactGenerator;

impl CodeGenerator for ReactGenerator {
    fn file_extension(&self) -> &str {
        "tsx"
    }

    fn map_prop_type(&self, prop: &CanonicalAbstractProp) -> String {
        if let Some(conc) = &prop.concrete_type {
            match conc.as_str() {
                "bool" | "boolean" => return "boolean".to_string(),
                "String" | "string" | "&str" => return "string".to_string(),
                "usize" | "i32" | "u32" | "f64" | "number" => return "number".to_string(),
                _ if conc.starts_with("enum: ") => return "string".to_string(),
                _ if conc.contains("Callback") || conc.contains("Fn") || conc.contains("=>") => {
                    return "() => void".to_string()
                }
                _ => return conc.clone(),
            }
        }
        match &prop.abstract_type {
            AbstractPropType::ControlFlag => "boolean".to_string(),
            AbstractPropType::AsyncEventHandler(_) => "() => void".to_string(),
            AbstractPropType::Renderable => "React.ReactNode".to_string(),
            _ => "any".to_string(),
        }
    }

    fn generate_component_code(&self, comp: &CanonicalAbstractComponent) -> String {
        let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let props_interface = format!("{}Props", name);
        let mut props_fields = Vec::new();
        let mut destructured: Vec<String> = Vec::new();
        let mut has_spread = false;

        for prop in &comp.props {
            let ts_type = self.map_prop_type(prop);
            let field_name = &prop.canonical_name;

            if prop.abstract_type == AbstractPropType::SpreadAttributes {
                has_spread = true;
                props_fields.push("  ...rest: React.HTMLAttributes<HTMLElement>;".to_string());
                destructured.push("...rest".to_string());
                continue;
            }
            if prop.abstract_type == AbstractPropType::Renderable {
                props_fields.push(format!("  {}: React.ReactNode;", field_name));
                destructured.push(field_name.clone());
                continue;
            }

            let optional = if prop.reactivity == AbstractReactivity::Static {
                "?"
            } else {
                ""
            };
            props_fields.push(format!("  {}{}: {};", field_name, optional, ts_type));
            let default_val = if prop.reactivity == AbstractReactivity::Static
                && prop.concrete_type.as_deref() == Some("bool")
            {
                " = false"
            } else {
                ""
            };
            destructured.push(format!("{}{}", field_name, default_val));
        }

        let spread = if has_spread { "{...rest}" } else { "" };
        format!(
            r#"import React from 'react';

interface {props_interface} {{
{props_body}
}}

export function {name}({{ {destructured} }}: {props_interface}) {{
  return (
    <div {spread}>
      {{/* TODO: Implement {name} */}}
    </div>
  );
}}
"#,
            props_body = props_fields.join("\n"),
            destructured = destructured.join(", "),
            spread = spread,
        )
    }

    fn write_project_files(&self, manifest: &PackageManifest, dir: &Path) -> Result<()> {
        let package_json = format!(
            r#"{{"name": "{}", "version": "{}", "dependencies": {{"react": "^19.0.0", "react-dom": "^19.0.0"}}}}"#,
            manifest.name, manifest.version
        );
        fs::write(dir.join("package.json"), package_json).map_err(ucp_core::UcpError::Io)?;
        let tsconfig = r#"{"compilerOptions": {"jsx": "react-jsx", "strict": true}}"#;
        fs::write(dir.join("tsconfig.json"), tsconfig).map_err(ucp_core::UcpError::Io)?;
        Ok(())
    }
}

pub fn generate_react(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    generate_with(manifest, output_dir, &ReactGenerator)
}
