use std::fs;
use std::path::Path;
use ucp_core::cam::*;
use ucp_core::Result;
use super::common::to_snake_case;

pub fn generate_web_components(manifest: &PackageManifest, output_dir: &str) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir.join("src")).map_err(ucp_core::UcpError::Io)?;
    for comp in &manifest.components {
        let raw_name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let file_name = to_snake_case(raw_name);
        let file_path = dir.join("src").join(format!("{}.js", file_name));
        let code = generate_component_code(comp);
        fs::write(&file_path, code).map_err(ucp_core::UcpError::Io)?;
    }
    let pkg = format!(r#"{{"name":"{}","version":"{}","type":"module","dependencies":{{"lit":"^3.0.0"}}}}"#, manifest.name, manifest.version);
    fs::write(dir.join("package.json"), pkg).map_err(ucp_core::UcpError::Io)?;
    Ok(())
}

fn generate_component_code(comp: &CanonicalAbstractComponent) -> String {
    let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
    let tag = format!("ucp-{}", to_snake_case(name));
    let cls = format!("{}Element", name);
    let mut props = Vec::new();
    let mut events = Vec::new();

    for p in &comp.props {
        if p.abstract_type == AbstractPropType::SpreadAttributes || p.abstract_type == AbstractPropType::Renderable { continue; }
        if let AbstractPropType::AsyncEventHandler(_) = &p.abstract_type { continue; }
        let js = map_js(p);
        let dflt = if p.reactivity == AbstractReactivity::Static {
            match p.concrete_type.as_deref() { Some("bool") => "false", Some("String") => "''", _ => "null" }
        } else { "null" };
        props.push(format!("  @property({{ type: {} }}) {} = {};", js, p.canonical_name, dflt));
    }
    for e in &comp.events {
        events.push(format!("  _on_{}(ev) {{ this.dispatchEvent(new CustomEvent('{}', {{detail:ev, bubbles:true, composed:true}})); }}", e.canonical_name, e.canonical_name));
    }
    let slot = if comp.props.iter().any(|p| p.abstract_type == AbstractPropType::Renderable) { "\n      <slot></slot>\n    " } else { "\n      <!-- no slot -->\n    " };
    format!(r#"import {{ LitElement, html, css }} from 'lit';
import {{ customElement, property }} from 'lit/decorators.js';

@customElement('{tag}')
export class {cls} extends LitElement {{
  static styles = css` :host {{ display: inline-block; }} `;
{props}
{events}
  render() {{ return html`{slot}`; }}
}}"#, props=props.join("\n\n"), events=events.join("\n\n"), slot=slot)
}

fn map_js(p: &CanonicalAbstractProp) -> &'static str {
    if let Some(c) = &p.concrete_type {
        match c.as_str() { "bool"|"boolean" => "Boolean", "String"|"string"|"&str" => "String", "usize"|"i32"|"u32"|"f64"|"number" => "Number", _ if c.starts_with("enum:") => "String", _ => "Object" }
    } else { match &p.abstract_type { AbstractPropType::ControlFlag => "Boolean", _ => "Object" } }
}
