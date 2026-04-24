use quote::ToTokens;
use syn::{parse_file, visit::Visit, FnArg, ItemFn, Meta, Pat};
use ucp_core::Result;

#[derive(Debug, Clone)]
pub struct RawComponentExtraction {
    pub name: String,
    pub line_start: usize,
    pub props: Vec<RawPropExtraction>,
}

#[derive(Debug, Clone)]
pub struct RawPropExtraction {
    pub name: String,
    pub raw_type: String,
    pub has_default: bool,
}

struct ComponentVisitor(Vec<RawComponentExtraction>);

impl Visit<'_> for ComponentVisitor {
    fn visit_item_fn(&mut self, func: &ItemFn) {
        let is_component = func
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("component"));
        if !is_component {
            return;
        }

        let name = func.sig.ident.to_string();
        let mut props = Vec::new();

        for input in &func.sig.inputs {
            if let FnArg::Typed(pat_type) = input {
                let prop_name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    pat_ident.ident.to_string()
                } else {
                    continue;
                };

                // Use ToTokens to get the source-level type string (e.g. "bool",
                // "MaybeSignal<String>") instead of syn's Debug which prints AST
                // structure (e.g. "Type::Path { ... }").
                let raw_type = pat_type.ty.to_token_stream().to_string();
                let has_default = pat_type.attrs.iter().any(|attr| {
                    if let Meta::List(list) = &attr.meta {
                        list.path.is_ident("prop") && list.tokens.to_string().contains("default")
                    } else {
                        false
                    }
                });

                props.push(RawPropExtraction {
                    name: prop_name,
                    raw_type,
                    has_default,
                });
            }
        }

        // line_start is set to 0 here; post-processed after visiting.
        self.0.push(RawComponentExtraction {
            name,
            line_start: 0,
            props,
        });
    }
}

pub fn extract_rust_components(code: &str) -> Result<Vec<RawComponentExtraction>> {
    let ast = parse_file(code).map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;

    let mut visitor = ComponentVisitor(Vec::new());
    syn::visit::visit_file(&mut visitor, &ast);

    // proc_macro2::Span does not expose line/column info outside of proc-macro
    // context. Compute line_start by searching for "fn <name>" in the source.
    for comp in &mut visitor.0 {
        let search = format!("fn {}", comp.name);
        if let Some(pos) = code.find(&search) {
            comp.line_start = code[..pos].matches('\n').count() + 1;
        }
    }

    Ok(visitor.0)
}
