use super::common::to_snake_case;
use super::traits::{generate_with, CodeGenerator};
use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;

pub struct WebComponentsGenerator;

impl CodeGenerator for WebComponentsGenerator {
    fn file_extension(&self) -> &str {
        "js"
    }

    fn map_prop_type(&self, prop: &CanonicalAbstractProp) -> String {
        if let Some(c) = &prop.concrete_type {
            match c.as_str() {
                "bool" | "boolean" => return "Boolean".to_string(),
                "String" | "string" | "&str" => return "String".to_string(),
                _ => return "Object".to_string(),
            }
        }
        match &prop.abstract_type {
            AbstractPropType::ControlFlag => "Boolean".to_string(),
            _ => "Object".to_string(),
        }
    }

    fn generate_component_code(&self, comp: &CanonicalAbstractComponent) -> String {
        let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let tag = format!("ucp-{}", to_snake_case(name));
        let cls = format!("{}Element", name);
        let mut props = Vec::new();
        let mut events = Vec::new();

        for p in &comp.props {
            if p.abstract_type == AbstractPropType::SpreadAttributes
                || p.abstract_type == AbstractPropType::Renderable
            {
                continue;
            }
            if let AbstractPropType::AsyncEventHandler(_) = &p.abstract_type {
                continue;
            }
            let js = self.map_prop_type(p);
            let dflt = if p.reactivity == AbstractReactivity::Static {
                match p.concrete_type.as_deref() {
                    Some("bool") => "false",
                    Some("String") => "''",
                    _ => "null",
                }
            } else {
                "null"
            };
            props.push(format!(
                "  @property({{ type: {} }}) {} = {};",
                js, p.canonical_name, dflt
            ));
        }
        for e in &comp.events {
            events.push(format!("  _on_{}(ev) {{ this.dispatchEvent(new CustomEvent('{}', {{ detail: ev, bubbles: true, composed: true }})); }}", e.canonical_name, e.canonical_name));
        }
        let slot = if comp
            .props
            .iter()
            .any(|p| p.abstract_type == AbstractPropType::Renderable)
        {
            "\n      <slot></slot>\n    "
        } else {
            "\n      <!-- no slot -->\n    "
        };
        format!(
            r#"import {{ LitElement, html, css }} from 'lit';
import {{ customElement, property }} from 'lit/decorators.js';

@customElement('{tag}')
export class {cls} extends LitElement {{
  static styles = css` :host {{ display: inline-block; }} `;
{props}
{events}
  render() {{ return html`{slot}`; }}
}}"#,
            props = props.join("\n\n"),
            events = events.join("\n\n"),
            slot = slot,
        )
    }

    fn write_project_files(&self, manifest: &PackageManifest, dir: &Path) -> Result<()> {
        let pkg = format!(
            r#"{{"name":"{}","version":"{}","type":"module","dependencies":{{"lit":"^3.0.0"}}}}"#,
            manifest.name, manifest.version
        );
        fs::write(dir.join("package.json"), pkg).map_err(ucp_core::UcpError::Io)?;
        Ok(())
    }
}

pub fn generate_web_components(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    generate_with(manifest, output_dir, &WebComponentsGenerator)
}
