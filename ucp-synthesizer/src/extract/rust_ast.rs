use quote::ToTokens;
use syn::{parse_file, visit::Visit, FnArg, ItemFn, Meta, Pat};
use ucp_core::Result;

#[derive(Debug, Clone)]
pub struct RawComponentExtraction {
    pub name: String,
    pub line_start: usize,
    pub props: Vec<RawPropExtraction>,
    /// True if this extraction came from the struct-props visitor.
    pub is_struct_pattern: bool,
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
            is_struct_pattern: false,
        });
    }
}

pub fn extract_rust_components(code: &str) -> Result<Vec<RawComponentExtraction>> {
    // Parse the AST once for both visitors
    let ast = parse_file(code).map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;

    // Original visitor for #[component] functions
    let mut fn_visitor = ComponentVisitor(Vec::new());
    syn::visit::visit_file(&mut fn_visitor, &ast);

    // Compute line_start for fn-based components (existing logic)
    for comp in &mut fn_visitor.0 {
        if comp.line_start == 0 {
            let search = format!("fn {}", comp.name);
            if let Some(pos) = code.find(&search) {
                comp.line_start = code[..pos].matches('\n').count() + 1;
            }
        }
    }

    // New visitor for struct-props pattern
    let struct_components = StructComponentVisitor::extract(code)?;

    // Combine results
    let mut all = fn_visitor.0;
    all.extend(struct_components);

    Ok(all)
}

// ── Struct-props visitor ──────────────────────────────────────────────────

/// A visitor that extracts components defined via the struct-props pattern.
///
/// Recognised pattern:
///   pub struct <Name>Props { ... }
///   impl <Name> {
///       pub fn render(props: <Name>Props) -> impl IntoView { ... }
///   }
pub struct StructComponentVisitor {
    pub components: Vec<RawComponentExtraction>,
    props_structs: std::collections::HashMap<String, (String, Vec<RawPropExtraction>, usize)>,
    source: String,
}

impl StructComponentVisitor {
    pub fn extract(code: &str) -> Result<Vec<RawComponentExtraction>> {
        use syn::visit::Visit;
        let ast = parse_file(code).map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;
        let mut visitor = StructComponentVisitor {
            components: Vec::new(),
            props_structs: std::collections::HashMap::new(),
            source: code.to_string(),
        };
        visitor.visit_file(&ast);
        Ok(visitor.components)
    }
}

/// Extract a simple type name from a `syn::Type`, handling paths and generics.
fn extract_type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|seg| seg.ident.to_string())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

impl Visit<'_> for StructComponentVisitor {
    fn visit_item_struct(&mut self, item: &syn::ItemStruct) {
        let struct_name = item.ident.to_string();
        if !struct_name.ends_with("Props") || struct_name == "Props" {
            return;
        }
        let stem = struct_name
            .strip_suffix("Props")
            .unwrap_or(&struct_name)
            .to_string();
        if stem.is_empty() {
            return;
        }
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
            let raw_type = field.ty.to_token_stream().to_string();
            let has_default = raw_type.trim().starts_with("Option");
            props.push(RawPropExtraction {
                name: field_name,
                raw_type,
                has_default,
            });
        }
        if props.is_empty() {
            return;
        }
        let search = format!("pub struct {}", struct_name);
        let line_start = self
            .source
            .find(&search)
            .map(|pos| self.source[..pos].matches('\n').count() + 1)
            .unwrap_or(0);
        self.props_structs
            .insert(stem, (struct_name, props, line_start));
    }

    fn visit_item_impl(&mut self, item: &syn::ItemImpl) {
        let impl_type_name = extract_type_name(&item.self_ty);
        if impl_type_name.is_empty() {
            return;
        }
        if let Some((struct_name, props, line_start)) = self.props_structs.remove(&impl_type_name) {
            let mut found_render = false;
            for item_impl in &item.items {
                if let syn::ImplItem::Fn(method) = item_impl {
                    if method.sig.ident == "render"
                        && matches!(method.vis, syn::Visibility::Public(_))
                        && method.sig.inputs.len() == 1
                    {
                        if let Some(syn::FnArg::Typed(pat_type)) = method.sig.inputs.first() {
                            let param_type_name = extract_type_name(&pat_type.ty);
                            if param_type_name == struct_name
                                || param_type_name.ends_with(&struct_name)
                            {
                                found_render = true;
                                self.components.push(RawComponentExtraction {
                                    name: impl_type_name.clone(),
                                    line_start,
                                    props: props.clone(),
                                    is_struct_pattern: true,
                                });
                                break;
                            }
                        }
                    }
                }
            }
            if !found_render {
                eprintln!(
                    "  ⚠ Skipping component in struct {}: no matching pub fn render found",
                    struct_name
                );
            }
        }
        syn::visit::visit_item_impl(self, item);
    }
}
