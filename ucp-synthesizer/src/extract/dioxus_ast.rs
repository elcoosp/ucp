use syn::{parse_file, visit::Visit, FnArg, ItemFn, ItemStruct};
use quote::ToTokens;
use ucp_core::Result;

use super::rust_ast::{RawComponentExtraction, RawPropExtraction};
use crate::utils::normalize_type_string;

/// Extract Dioxus components that use a derive(Props) struct.
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

    /// Search the full source for `use_context_provider(|| Type { ... })`
    /// and `use_context::<Type>()`. Returns (provided_context, consumed list).
    fn find_context_types(&self) -> (Option<String>, Vec<String>) {
        let full_text = self.source;
        let mut provided = None;
        let mut consumed = Vec::new();

        // Look for `use_context_provider(|| Type {`
        if let Some(pos) = full_text.find("use_context_provider") {
            let after = &full_text[pos + "use_context_provider".len()..];
            if let Some(pipes) = after.find("||") {
                let after_pipes = &after[pipes + 2..];
                if let Some(brace) = after_pipes.find('{') {
                    let ty = after_pipes[..brace].trim();
                    if !ty.is_empty()
                        && ty.chars().next().unwrap().is_uppercase()
                        && ty != "|"
                    {
                        provided = Some(ty.to_string());
                    }
                }
            }
        }

        // Look for `use_context::<Type>()`
        let mut search_start = 0;
        while let Some(pos) = full_text[search_start..].find("use_context::<") {
            let actual_pos = search_start + pos;
            let after = &full_text[actual_pos + "use_context::<".len()..];
            if let Some(end) = after.find('>') {
                let ty = after[..end].trim();
                if !ty.is_empty() {
                    consumed.push(ty.to_string());
                }
            }
            search_start = actual_pos + "use_context::<".len() + 1;
        }

        (provided, consumed)
    }
}

impl<'a> Visit<'a> for DioxusVisitor<'a> {
    fn visit_item_struct(&mut self, item: &ItemStruct) {
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
            let raw_type =
                normalize_type_string(&field.ty.to_token_stream().to_string());
            let has_default = field.attrs.iter().any(|a| {
                if let syn::Meta::List(ml) = &a.meta {
                    if ml.path.is_ident("props") {
                        ml.tokens.to_string().contains("default")
                    } else {
                        false
                    }
                } else {
                    false
                }
            });
            let is_spread_attributes = field.attrs.iter().any(|a| {
                if let syn::Meta::List(ml) = &a.meta {
                    if ml.path.is_ident("props") {
                        ml.tokens.to_string().contains("extends")
                            && ml.tokens.to_string().contains("GlobalAttributes")
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
        if !func
            .attrs
            .iter()
            .any(|a| a.path().is_ident("component"))
        {
            return;
        }
        if func.sig.inputs.len() != 1 {
            return;
        }
        let param = match &func.sig.inputs[0] {
            FnArg::Typed(pat_type) => pat_type,
            _ => return,
        };
        let param_type_name =
            normalize_type_string(&param.ty.to_token_stream().to_string());
        if let Some((props, line_start)) = self.props_structs.remove(&param_type_name) {
            let (provided_ctx, consumed_ctx) = self.find_context_types();
            self.components.push(RawComponentExtraction {
                name: func.sig.ident.to_string(),
                line_start,
                props,
                is_struct_pattern: true,
                provided_context: provided_ctx,
                consumed_contexts: consumed_ctx,
            });
        }
    }
}
