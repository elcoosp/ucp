use syn::{parse_file, visit::Visit, FnArg, ItemFn, ItemStruct};
use ucp_core::Result;

use super::rust_ast::{RawComponentExtraction, RawPropExtraction};
use crate::utils::normalize_type_string;

/// Extract components that follow the Dioxus pattern:
///   #[derive(Props)]
///   pub struct SomeProps { ... }
///
///   #[component]
///   pub fn SomeComponent(props: SomeProps) -> Element { ... }
pub fn extract_dioxus_components(code: &str) -> Result<Vec<RawComponentExtraction>> {
    let ast = parse_file(code).map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;

    let mut visitor = DioxusVisitor::new(code);
    syn::visit::visit_file(&mut visitor, &ast);

    Ok(visitor.components)
}

struct DioxusVisitor<'a> {
    components: Vec<RawComponentExtraction>,
    props_structs: std::collections::HashMap<String, (Vec<RawPropExtraction>, usize)>,
    source: &'a str,
}

impl<'a> DioxusVisitor<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            components: Vec::new(),
            props_structs: std::collections::HashMap::new(),
            source,
        }
    }
}

impl<'a> Visit<'a> for DioxusVisitor<'a> {
    fn visit_item_struct(&mut self, item: &ItemStruct) {
        // Only process structs that derive `Props`
        let has_props_derive = item.attrs.iter().any(|attr| {
            if let syn::Meta::List(ml) = &attr.meta {
                if ml.path.is_ident("derive") {
                    ml.tokens.to_string().contains("Props")
                } else {
                    false
                }
            } else {
                false
            }
        });

        if !has_props_derive {
            return;
        }

        let struct_name = item.ident.to_string();
        let mut props = Vec::new();

        for field in &item.fields {
            let field_name = field
                .ident
                .as_ref()
                .map(|i| i.to_string())
                .unwrap_or_default();
            if field_name.is_empty() {
                continue;
            }

            let raw_type = normalize_type_string(&field.ty.to_token_stream().to_string());
            let has_default = field.attrs.iter().any(|a| {
                if let syn::Meta::List(ml) = &a.meta {
                    if ml.path.is_ident("props") {
                        let tokens = ml.tokens.to_string();
                        tokens.contains("default")
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            // Detect spread attributes: #[props(extends = GlobalAttributes)] on attributes: Vec<Attribute>
            let is_spread_attributes = field.attrs.iter().any(|a| {
                if let syn::Meta::List(ml) = &a.meta {
                    if ml.path.is_ident("props") {
                        let tokens = ml.tokens.to_string();
                        tokens.contains("extends") && tokens.contains("GlobalAttributes")
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            props.push(RawPropExtraction {
                name: field_name,
                raw_type,
                has_default,
                is_event: false,
                is_spread_attributes,
            });
        }

        if props.is_empty() {
            return;
        }

        let line_start = self
            .source
            .find(&format!("pub struct {}", struct_name))
            .map(|pos| self.source[..pos].matches('\n').count() + 1)
            .unwrap_or(0);

        self.props_structs
            .insert(struct_name, (props, line_start));
    }

    fn visit_item_fn(&mut self, func: &ItemFn) {
        let is_component = func
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("component"));
        if !is_component {
            return;
        }

        // Dioxus component must have exactly one typed parameter
        if func.sig.inputs.len() != 1 {
            return;
        }

        let param = match &func.sig.inputs[0] {
            FnArg::Typed(pat_type) => pat_type,
            _ => return,
        };

        let param_type_name = normalize_type_string(&param.ty.to_token_stream().to_string());

        // If the parameter type name matches a known Props struct, extract it.
        if let Some((props, line_start)) = self.props_structs.remove(&param_type_name) {
            self.components.push(RawComponentExtraction {
                name: func.sig.ident.to_string(),
                line_start,
                props,
                is_struct_pattern: true,  // treat as struct pattern for unification
            });
        }
    }
}
