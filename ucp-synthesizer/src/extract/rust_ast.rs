use quote::ToTokens;
use syn::{parse_file, visit::Visit, FnArg, ItemFn, Meta, Pat};
use ucp_core::Result;

use super::dioxus_ast;

#[derive(Debug, Clone)]
pub struct RawComponentExtraction {
    pub name: String,
    pub line_start: usize,
    pub props: Vec<RawPropExtraction>,
    pub is_struct_pattern: bool,
}

#[derive(Debug, Clone)]
pub struct RawPropExtraction {
    pub name: String,
    pub raw_type: String,
    pub has_default: bool,
    pub is_event: bool,
    pub is_spread_attributes: bool,
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

                let raw_type = pat_type.ty.to_token_stream().to_string().replace(" ", "");
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
                    is_event: false,
                    is_spread_attributes: false,
                });
            }
        }

        self.0.push(RawComponentExtraction {
            name,
            line_start: 0,
            props,
            is_struct_pattern: false,
        });
    }
}

pub fn extract_rust_components(code: &str) -> Result<Vec<RawComponentExtraction>> {
    let ast = parse_file(code).map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;

    let mut fn_visitor = ComponentVisitor(Vec::new());
    syn::visit::visit_file(&mut fn_visitor, &ast);

    for comp in &mut fn_visitor.0 {
        if comp.line_start == 0 {
            let search = format!("fn {}", comp.name);
            if let Some(pos) = code.find(&search) {
                comp.line_start = code[..pos].matches('\n').count() + 1;
            }
        }
    }

    let struct_components = StructComponentVisitor::extract(code)?;
    let gpui_components = GpuiComponentVisitor::extract(code)?;
    let dioxus_components = dioxus_ast::extract_dioxus_components(code)?;

    let mut all = fn_visitor.0;
    all.extend(struct_components);
    all.extend(gpui_components);
    all.extend(dioxus_components);

    // Deduplicate by component name, preferring later entries (struct-pattern / Dioxus)
    let mut seen = std::collections::HashMap::new();
    for comp in all {
        seen.insert(comp.name.clone(), comp);
    }
    let deduped: Vec<_> = seen.into_values().collect();

    Ok(deduped)
}

// ── Struct-props visitor ──────────────────────────────────────────────────

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
            let raw_type = crate::utils::normalize_type_string(&field.ty.to_token_stream().to_string());
            let has_default = raw_type.trim().starts_with("Option");
            props.push(RawPropExtraction {
                name: field_name,
                raw_type,
                has_default,
                is_event: false,
                is_spread_attributes: false,
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

// PATCH-GPUI-VISITOR

// ── GPUI visitor ──────────────────────────────────────────────────────────

pub struct GpuiComponentVisitor;

impl GpuiComponentVisitor {
    pub fn extract(code: &str) -> Result<Vec<RawComponentExtraction>> {
        let ast = syn::parse_file(code)
            .map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;
        let mut components: Vec<RawComponentExtraction> = Vec::new();

        let mut struct_candidates: Vec<(&syn::Item, &syn::ItemStruct)> = Vec::new();
        for item in &ast.items {
            if let syn::Item::Struct(item_struct) = item {
                if item_struct.attrs.iter().any(|attr| {
                    if let syn::Meta::List(ml) = &attr.meta {
                        if ml.path.is_ident("derive") {
                            ml.tokens.to_string().contains("IntoElement")
                        } else { false }
                    } else { false }
                }) {
                    struct_candidates.push((item, item_struct));
                }
            }
        }

        let mut impl_methods: std::collections::HashMap<String, Vec<&syn::ImplItem>> = std::collections::HashMap::new();
        for item in &ast.items {
            if let syn::Item::Impl(impl_block) = item {
                let type_name = if let syn::Type::Path(tp) = &*impl_block.self_ty {
                    tp.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
                } else { continue };
                if type_name.is_empty() { continue; }
                impl_methods.entry(type_name)
                    .or_default()
                    .extend(impl_block.items.iter());
            }
        }

        for (_, item_struct) in struct_candidates {
            let struct_name = item_struct.ident.to_string();
            let mut props: Vec<RawPropExtraction> = Vec::new();

            for field in &item_struct.fields {
                let field_name = field.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
                if field_name.is_empty() { continue; }
                let raw_type = crate::utils::normalize_type_string(&field.ty.to_token_stream().to_string());
                let has_default = raw_type.trim().starts_with("Option");
                props.push(RawPropExtraction {
                    name: field_name.clone(),
                    raw_type,
                    has_default,
                    is_event: false,
                    is_spread_attributes: false,
                });
            }

            if let Some(methods) = impl_methods.get(&struct_name) {
                for item_impl in methods {
                    if let syn::ImplItem::Fn(method) = item_impl {
                        if method.sig.inputs.len() != 2 { continue; }
                        if method.sig.ident == "new" { continue; }
                        let is_mut_self = matches!(method.sig.inputs.first(), Some(syn::FnArg::Receiver(r)) if r.mutability.is_some());
                        if !is_mut_self { continue; }

                        let param = match &method.sig.inputs[1] {
                            syn::FnArg::Typed(pt) => pt,
                            _ => continue,
                        };
                        let arg_type = crate::utils::normalize_type_string(&param.ty.to_token_stream().to_string());

                        let is_event = arg_type.contains("Fn") || arg_type.contains("Callback") ||
                            arg_type.contains("fn(") || method.sig.ident.to_string().starts_with("on_");

                        let mut matched_field = false;
                        for prop in &mut props {
                            if prop.name == method.sig.ident.to_string() {
                                prop.raw_type = arg_type.clone();
                                prop.has_default = false;
                                prop.is_event = is_event;
                                matched_field = true;
                                break;
                            }
                        }

                        if !matched_field && is_event {
                            props.push(RawPropExtraction {
                                name: method.sig.ident.to_string(),
                                raw_type: arg_type,
                                has_default: false,
                                is_event: true,
                                is_spread_attributes: false,
                            });
                        }
                    }
                }
            }

            if impl_methods.get(&struct_name)
                .map_or(false, |methods| {
                    methods.iter().any(|m| {
                        if let syn::ImplItem::Type(ty) = m {
                            ty.ident == "ParentElement"
                        } else { false }
                    })
                })
                || ast.items.iter().any(|item| {
                    if let syn::Item::Impl(impl_block) = item {
                        if let Some(trait_) = &impl_block.trait_ {
                            if let Some(seg) = trait_.1.segments.last() {
                                if seg.ident == "ParentElement" {
                                    if let syn::Type::Path(tp) = &*impl_block.self_ty {
                                        if let Some(seg) = tp.path.segments.last() {
                                            return seg.ident.to_string() == struct_name;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    false
                })
            {
                props.push(RawPropExtraction {
                    name: "children".to_string(),
                    raw_type: "Children".to_string(),
                    has_default: false,
                    is_event: false,
                    is_spread_attributes: false,
                });
            }

            if props.is_empty() { continue; }

            let line_start = code.lines()
                .enumerate()
                .find(|(_, line)| line.contains(&format!("struct {}", struct_name)))
                .map(|(i, _)| i + 1)
                .unwrap_or(0);
            components.push(RawComponentExtraction {
                name: struct_name,
                line_start,
                props,
                is_struct_pattern: false,
            });
        }

        for comp in &mut components { Self::group_variant_props(&mut comp.props); }
        Ok(components)
    }

    fn group_variant_props(props: &mut Vec<RawPropExtraction>) {
        use std::collections::HashMap;
        let mut field_setters: HashMap<String, Vec<String>> = HashMap::new();
        for p in props.iter() {
            let parts: Vec<&str> = p.name.splitn(2, '_').collect();
            if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
                field_setters.entry(parts[0].to_string()).or_default().push(parts[1].to_string());
            }
        }
        let mut to_remove = Vec::new();
        for (field, variants) in field_setters.iter() {
            if variants.len() > 1 {
                let enum_values = variants.join(" | ");
                let default = props.iter()
                    .find(|p| p.name == format!("{}_default", field) || p.name == *field)
                    .and_then(|p| if p.has_default { Some(p.name.clone()) } else { None });
                to_remove.extend(props.iter().enumerate().filter(|(_, p)| p.name.starts_with(field)).map(|(i, _)| i));
                props.retain(|p| !p.name.starts_with(field));
                props.push(RawPropExtraction {
                    name: field.clone(),
                    raw_type: format!("enum {{ {} }}", enum_values),
                    has_default: default.is_some(),
                    is_event: false,
                    is_spread_attributes: false,
                });
            }
        }
    }
}
