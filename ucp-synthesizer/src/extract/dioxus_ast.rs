use syn::{parse_file, visit::Visit, FnArg, ItemFn, ItemStruct, ExprCall, ExprClosure, Stmt};
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

// ── AST-based Context Visitor ───────────────────────────────────────────

/// Walks a function body to find `use_context_provider(|| Type { ... })`
/// and `use_context::<Type>()` calls.
struct ContextVisitor {
    provided: Option<String>,
    consumed: Vec<String>,
}

impl<'ast> Visit<'ast> for ContextVisitor {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        if let Stmt::Expr(expr, _) = stmt {
            self.scan_expr(expr);
        }
        syn::visit::visit_stmt(self, stmt);
    }

    fn visit_expr_call(&mut self, call: &'ast ExprCall) {
        // Check the function path
        if let Some(path) = extract_call_path(&call.func) {
            // Detect `use_context_provider(|| SomeType { ... })`
            if path == "use_context_provider" {
                if let Some(first_arg) = call.args.first() {
                    // The argument should be a closure `|| { ... }`
                    if let syn::Expr::Closure(closure) = first_arg {
                        self.scan_closure_for_struct(closure);
                    }
                }
            }
            // Detect `use_context::<SomeType>()`
            if path == "use_context" && call.args.is_empty() {
                // The type is in the turbofish
                if let syn::Expr::Path(expr_path) = &*call.func {
                    if let Some(seg) = expr_path.path.segments.last() {
                        if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                            if let Some(syn::GenericArgument::Type(syn::Type::Path(tp))) = args.args.first() {
                                if let Some(ident) = tp.path.get_ident() {
                                    self.consumed.push(ident.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        syn::visit::visit_expr_call(self, call);
    }
}

impl ContextVisitor {
    /// Scan any expression we encounter (for non-call patterns)
    fn scan_expr(&mut self, _expr: &syn::Expr) {
        // Future: detect context via macros or helper functions
    }

    /// Look inside a closure body for a struct literal that matches the provided context type
    fn scan_closure_for_struct(&mut self, closure: &ExprClosure) {
        let body = &closure.body;
        // The body is an expression — most commonly a block or a struct literal
        match &**body {
            syn::Expr::Block(block) => {
                for stmt in &block.block.stmts {
                    if let Stmt::Expr(expr, _) = stmt {
                        if let Some(type_name) = extract_struct_type(expr) {
                            self.provided = Some(type_name);
                            return;
                        }
                    }
                }
            }
            syn::Expr::Struct(strukt) => {
                if let Some(type_name) = extract_type_name(&strukt.path) {
                    self.provided = Some(type_name);
                }
            }
            _ => {}
        }
    }
}

/// Extract a function path as a string, e.g., `use_context_provider` or `module::function`.
fn extract_call_path(expr: &syn::Expr) -> Option<String> {
    if let syn::Expr::Path(expr_path) = expr {
        let segments: Vec<String> = expr_path
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
        Some(segments.join("::"))
    } else {
        None
    }
}

/// Try to get the type name from a struct literal expression.
fn extract_struct_type(expr: &syn::Expr) -> Option<String> {
    if let syn::Expr::Struct(strukt) = expr {
        extract_type_name(&strukt.path)
    } else {
        None
    }
}

/// Get the last segment of a path as a string.
fn extract_type_name(path: &syn::Path) -> Option<String> {
    path.segments.last().map(|s| s.ident.to_string())
}

// ── Dioxus Visitor (unchanged except for context detection) ────────────

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
        let has_props_derive = item.attrs.iter().any(|attr| {
            if let syn::Meta::List(ml) = &attr.meta {
                if ml.path.is_ident("derive") {
                    ml.tokens.to_string().contains("Props")
                } else { false }
            } else { false }
        });
        if !has_props_derive { return; }

        let struct_name = item.ident.to_string();
        let mut props = Vec::new();
        for field in &item.fields {
            let field_name = field.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
            if field_name.is_empty() { continue; }
            let raw_type = normalize_type_string(&field.ty.to_token_stream().to_string());
            let has_default = field.attrs.iter().any(|a| {
                if let syn::Meta::List(ml) = &a.meta {
                    if ml.path.is_ident("props") {
                        ml.tokens.to_string().contains("default")
                    } else { false }
                } else { false }
            });
            let is_spread_attributes = field.attrs.iter().any(|a| {
                if let syn::Meta::List(ml) = &a.meta {
                    if ml.path.is_ident("props") {
                        ml.tokens.to_string().contains("extends")
                            && ml.tokens.to_string().contains("GlobalAttributes")
                    } else { false }
                } else { false }
            });
            props.push(RawPropExtraction {
                name: field_name,
                raw_type,
                has_default,
                is_event: false,
                is_spread_attributes,
            });
        }
        if props.is_empty() { return; }
        let line_start = self.source
            .find(&format!("pub struct {}", struct_name))
            .map(|pos| self.source[..pos].matches('\n').count() + 1)
            .unwrap_or(0);
        self.props_structs.insert(struct_name, (props, line_start));
    }

    fn visit_item_fn(&mut self, func: &ItemFn) {
        if !func.attrs.iter().any(|a| a.path().is_ident("component")) { return; }
        if func.sig.inputs.len() != 1 { return; }
        let param = match &func.sig.inputs[0] {
            FnArg::Typed(pat_type) => pat_type,
            _ => return,
        };
        let param_type_name = normalize_type_string(&param.ty.to_token_stream().to_string());
        if let Some((props, line_start)) = self.props_structs.remove(&param_type_name) {
            // AST-based context detection
            let mut ctx_visitor = ContextVisitor { provided: None, consumed: vec![] };
            ctx_visitor.visit_item_fn(func);
            self.components.push(RawComponentExtraction {
                name: func.sig.ident.to_string(),
                line_start,
                props,
                is_struct_pattern: true,
                provided_context: ctx_visitor.provided,
                consumed_contexts: ctx_visitor.consumed,
            });
        }
    }
}
