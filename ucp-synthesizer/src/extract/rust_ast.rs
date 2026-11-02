//! Rust source extraction: Leptos `#[component]` functions, struct-props
//! components, and GPUI `IntoElement` builder structs.

use quote::ToTokens;
use syn::visit::Visit;
use syn::{parse_file, FnArg, ItemFn, Meta, Pat};
use ucp_core::Result;

use super::context;

/// A component extracted from Rust source before unification.
#[derive(Debug, Clone)]
pub struct RawComponentExtraction {
    /// Component name (fn or struct identifier).
    pub name: String,
    /// 1-based line where the definition starts.
    pub line_start: usize,
    /// Raw props found on the component.
    pub props: Vec<RawPropExtraction>,
    /// Whether the component is a struct-props pattern (Leptos/Dioxus
    /// `XProps` struct + render impl) rather than a plain function.
    pub is_struct_pattern: bool,
    /// Type name of a provided context, if detected.
    pub provided_context: Option<String>,
    /// Type names of consumed contexts.
    pub consumed_contexts: Vec<String>,
}

/// A single prop extracted from Rust source before unification.
#[derive(Debug, Clone)]
pub struct RawPropExtraction {
    /// Prop name.
    pub name: String,
    /// Type as written in the source, normalized.
    pub raw_type: String,
    /// Whether the prop has a default value.
    pub has_default: bool,
    /// Whether the prop is an event handler.
    pub is_event: bool,
    /// Whether the prop spreads HTML attributes.
    pub is_spread_attributes: bool,
}

/// Extract all components from a Rust source string.
///
/// Combines four extraction strategies (Leptos `#[component]` fns,
/// `XProps` struct patterns, GPUI builder structs, and Dioxus
/// `derive(Props)` structs) and deduplicates by component name, keeping
/// the last strategy that matched (Dioxus > GPUI > struct > fn).
///
/// # Errors
///
/// Returns [`UcpError::Parsing`](ucp_core::UcpError::Parsing) when the
/// source is not valid Rust.
pub fn extract_rust_components(code: &str) -> Result<Vec<RawComponentExtraction>> {
    let ast = parse_file(code).map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;

    let mut fn_visitor = ComponentVisitor::default();
    syn::visit::visit_file(&mut fn_visitor, &ast);

    let mut all = fn_visitor.0;
    all.extend(StructComponentVisitor::extract(code)?);
    all.extend(GpuiComponentVisitor::extract(code)?);
    all.extend(super::dioxus_ast::extract_dioxus_components(code)?);

    // Deduplicate by name, deterministically. Later strategies win, matching
    // the historical precedence of the four passes above.
    let mut seen = std::collections::BTreeMap::new();
    for comp in all {
        seen.insert(comp.name.clone(), comp);
    }
    Ok(seen.into_values().collect())
}

/// Extract props from `#[component]`-annotated functions.
#[derive(Default)]
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

        let props = func
            .sig
            .inputs
            .iter()
            .filter_map(|input| match input {
                FnArg::Typed(pat_type) => Some(fn_prop(pat_type)),
                FnArg::Receiver(_) => None,
            })
            .collect();

        let usage = context::detect_leptos_context(&func.to_token_stream().to_string());

        self.0.push(RawComponentExtraction {
            name: func.sig.ident.to_string(),
            line_start: func.sig.ident.span().start().line,
            props,
            is_struct_pattern: false,
            provided_context: usage.provided,
            consumed_contexts: usage.consumed,
        });
    }
}

/// Extract one typed function parameter as a raw prop.
fn fn_prop(pat_type: &syn::PatType) -> RawPropExtraction {
    let name = match &*pat_type.pat {
        Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
        _ => String::new(),
    };
    let has_default = pat_type.attrs.iter().any(|attr| {
        if let Meta::List(list) = &attr.meta {
            list.path.is_ident("prop") && list.tokens.to_string().contains("default")
        } else {
            false
        }
    });
    RawPropExtraction {
        name,
        raw_type: pat_type.ty.to_token_stream().to_string().replace(' ', ""),
        has_default,
        is_event: false,
        is_spread_attributes: false,
    }
}

// ── Struct-props visitor (Leptos `XProps` + render impl) ─────────────────

/// Extract Leptos-style struct-props components:
/// `struct ButtonProps { .. }` + `impl Button { pub fn render(props: ButtonProps) }`.
pub struct StructComponentVisitor {
    components: Vec<RawComponentExtraction>,
    /// `Button` → (struct name, props, line)
    props_structs: std::collections::HashMap<String, (String, Vec<RawPropExtraction>, usize)>,
}

impl StructComponentVisitor {
    /// Extract struct-props components from Rust source.
    ///
    /// # Errors
    ///
    /// Returns [`UcpError::Parsing`](ucp_core::UcpError::Parsing) when the
    /// source is not valid Rust.
    pub fn extract(code: &str) -> Result<Vec<RawComponentExtraction>> {
        let ast = parse_file(code).map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;
        let mut visitor = Self {
            components: Vec::new(),
            props_structs: std::collections::HashMap::new(),
        };
        visitor.visit_file(&ast);
        Ok(visitor.components)
    }
}

/// Last path-segment name of a type, or `""` for non-path types.
fn type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map_or_else(String::new, |seg| seg.ident.to_string()),
        _ => String::new(),
    }
}

impl Visit<'_> for StructComponentVisitor {
    fn visit_item_struct(&mut self, item: &syn::ItemStruct) {
        let struct_name = item.ident.to_string();
        let Some(stem) = struct_name.strip_suffix("Props") else {
            return;
        };
        if stem.is_empty() {
            return;
        }
        let props: Vec<RawPropExtraction> = item
            .fields
            .iter()
            .filter_map(|field| {
                let name = field.ident.as_ref().map(ToString::to_string)?;
                Some(struct_prop(&name, &field.ty))
            })
            .collect();
        if props.is_empty() {
            return;
        }
        self.props_structs.insert(
            stem.to_string(),
            (struct_name, props, item.ident.span().start().line),
        );
    }

    fn visit_item_impl(&mut self, item: &syn::ItemImpl) {
        let impl_type_name = type_name(&item.self_ty);
        if impl_type_name.is_empty() {
            return;
        }
        if let Some((struct_name, props, line_start)) = self.props_structs.remove(&impl_type_name) {
            let has_render = item.items.iter().any(|impl_item| {
                let syn::ImplItem::Fn(method) = impl_item else {
                    return false;
                };
                method.sig.ident == "render"
                    && matches!(method.vis, syn::Visibility::Public(_))
                    && method.sig.inputs.len() == 1
                    && matches!(
                        method.sig.inputs.first(),
                        Some(syn::FnArg::Typed(pt)) if param_matches(&type_name(&pt.ty), &struct_name)
                    )
            });
            if has_render {
                let usage = context::detect_leptos_context(&item.to_token_stream().to_string());
                self.components.push(RawComponentExtraction {
                    name: impl_type_name,
                    line_start,
                    props,
                    is_struct_pattern: true,
                    provided_context: usage.provided,
                    consumed_contexts: usage.consumed,
                });
            } else {
                eprintln!(
                    "  ⚠ Skipping component in struct {struct_name}: no matching pub fn render found"
                );
            }
        }
        syn::visit::visit_item_impl(self, item);
    }
}

/// True when the render impl's parameter type refers to the props struct
/// (either exactly or as a qualified suffix).
fn param_matches(param_type_name: &str, struct_name: &str) -> bool {
    param_type_name == struct_name || param_type_name.ends_with(struct_name)
}

/// Build a raw prop from a named struct field.
fn struct_prop(name: &str, ty: &syn::Type) -> RawPropExtraction {
    let raw_type = crate::utils::normalize_type_string(&ty.to_token_stream().to_string());
    RawPropExtraction {
        name: name.to_string(),
        has_default: raw_type.starts_with("Option"),
        raw_type,
        is_event: false,
        is_spread_attributes: false,
    }
}

// ── GPUI visitor ──────────────────────────────────────────────────────────

/// Extract GPUI-style builder components: `#[derive(IntoElement)]` structs
/// with `&mut self` setter methods.
pub struct GpuiComponentVisitor;

impl GpuiComponentVisitor {
    /// Extract all GPUI components from Rust source.
    ///
    /// # Errors
    ///
    /// Returns [`UcpError::Parsing`](ucp_core::UcpError::Parsing) when the
    /// source is not valid Rust.
    pub fn extract(code: &str) -> Result<Vec<RawComponentExtraction>> {
        let ast = parse_file(code).map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;
        let mut components: Vec<RawComponentExtraction> = Vec::new();

        // Structs deriving IntoElement.
        let candidates: Vec<&syn::ItemStruct> = ast
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Struct(s) if derives_into_element(s) => Some(s),
                _ => None,
            })
            .collect();

        // type name -> impl items across all impl blocks.
        let mut impl_methods: std::collections::HashMap<String, Vec<&syn::ImplItem>> =
            std::collections::HashMap::new();
        for item in &ast.items {
            if let syn::Item::Impl(impl_block) = item {
                let name = type_name(&impl_block.self_ty);
                if !name.is_empty() {
                    impl_methods
                        .entry(name)
                        .or_default()
                        .extend(&impl_block.items);
                }
            }
        }

        for item_struct in candidates {
            let struct_name = item_struct.ident.to_string();
            let mut props: Vec<RawPropExtraction> = item_struct
                .fields
                .iter()
                .filter_map(|field| {
                    let name = field.ident.as_ref().map(ToString::to_string)?;
                    Some(struct_prop(&name, &field.ty))
                })
                .collect();

            apply_builder_setters(
                &mut props,
                impl_methods
                    .get(&struct_name)
                    .map_or(&[] as &[_], |methods| methods.as_slice()),
            );
            if implements_parent_element(&ast, &struct_name, impl_methods.get(&struct_name)) {
                props.push(RawPropExtraction {
                    name: "children".to_string(),
                    raw_type: "Children".to_string(),
                    has_default: false,
                    is_event: false,
                    is_spread_attributes: false,
                });
            }
            if props.is_empty() {
                continue;
            }

            components.push(RawComponentExtraction {
                line_start: item_struct.ident.span().start().line,
                name: struct_name,
                props,
                is_struct_pattern: false,
                provided_context: None,
                consumed_contexts: vec![],
            });
        }

        for comp in &mut components {
            group_variant_props(&mut comp.props);
        }
        Ok(components)
    }
}

/// True when the struct derives `IntoElement`.
fn derives_into_element(item_struct: &syn::ItemStruct) -> bool {
    item_struct.attrs.iter().any(|attr| {
        matches!(&attr.meta, syn::Meta::List(ml) if ml.path.is_ident("derive"))
            && attr
                .meta
                .to_token_stream()
                .to_string()
                .contains("IntoElement")
    })
}

/// Overlay builder setter methods (`&mut self` methods) onto the raw props,
/// upgrading types and marking event handlers.
fn apply_builder_setters(props: &mut Vec<RawPropExtraction>, methods: &[&syn::ImplItem]) {
    for method in methods.iter().filter_map(|m| match m {
        syn::ImplItem::Fn(f) => Some(f),
        _ => None,
    }) {
        if method.sig.inputs.len() != 2 || method.sig.ident == "new" {
            continue;
        }
        let is_mut_self = matches!(method.sig.inputs.first(), Some(syn::FnArg::Receiver(r)) if r.mutability.is_some());
        if !is_mut_self {
            continue;
        }
        let Some(syn::FnArg::Typed(param)) = method.sig.inputs.get(1) else {
            continue;
        };
        let arg_type = crate::utils::normalize_type_string(&param.ty.to_token_stream().to_string());
        let name = method.sig.ident.to_string();
        let is_event = arg_type.contains("Fn")
            || arg_type.contains("Callback")
            || arg_type.contains("fn(")
            || name.starts_with("on_");

        if let Some(prop) = props.iter_mut().find(|p| p.name == name) {
            prop.raw_type = arg_type;
            prop.has_default = false;
            prop.is_event = is_event;
        } else if is_event {
            props.push(RawPropExtraction {
                name,
                raw_type: arg_type,
                has_default: false,
                is_event: true,
                is_spread_attributes: false,
            });
        }
    }
}

/// True when the struct participates in GPUI's `ParentElement` trait system
/// (associated-type alias or trait impl), which implies a `children` prop.
fn implements_parent_element(
    ast: &syn::File,
    struct_name: &str,
    methods: Option<&Vec<&syn::ImplItem>>,
) -> bool {
    let via_alias = methods.is_some_and(|ms| {
        ms.iter()
            .any(|m| matches!(m, syn::ImplItem::Type(ty) if ty.ident == "ParentElement"))
    });
    via_alias
        || ast.items.iter().any(|item| match item {
            syn::Item::Impl(impl_block) => {
                let trait_is_parent = impl_block
                    .trait_
                    .as_ref()
                    .and_then(|(path, _)| path.segments.last())
                    .is_some_and(|seg| seg.ident == "ParentElement");
                trait_is_parent && type_name(&impl_block.self_ty) == struct_name
            }
            _ => false,
        })
}

/// Collapse GPUI variant setters (`size_small`, `size_large`) into a single
/// enum-shaped prop.
fn group_variant_props(props: &mut Vec<RawPropExtraction>) {
    let mut field_setters: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for p in props.iter() {
        let parts: Vec<&str> = p.name.splitn(2, '_').collect();
        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            field_setters
                .entry(parts[0].to_string())
                .or_default()
                .push(parts[1].to_string());
        }
    }

    let variant_fields: Vec<String> = field_setters
        .into_iter()
        .filter(|(_, variants)| variants.len() > 1)
        .map(|(field, _)| field)
        .collect();

    for field in variant_fields {
        let prefix = format!("{field}_");
        let enum_values: Vec<String> = props
            .iter()
            .filter_map(|p| p.name.strip_prefix(&prefix).map(str::to_string))
            .collect();
        let default = props
            .iter()
            .find(|p| p.name == format!("{field}_default") || p.name == field)
            .filter(|p| p.has_default)
            .map(|p| p.name.clone());

        props.retain(|p| !p.name.starts_with(&field));
        props.push(RawPropExtraction {
            name: field.clone(),
            raw_type: format!("enum {{ {} }}", enum_values.join(" | ")),
            has_default: default.is_some(),
            is_event: false,
            is_spread_attributes: false,
        });
    }
}
