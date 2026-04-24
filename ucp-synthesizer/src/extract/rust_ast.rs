use syn::{parse_file, visit::Visit, Item, ItemFn, FnArg, Pat, Meta};
use ucp_core::Result;

#[derive(Debug, Clone)]
pub struct RawComponentExtraction {
    pub name: String,
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
        let is_component = func.attrs.iter().any(|attr| attr.path().is_ident("component"));
        if !is_component { return; }

        let name = func.sig.ident.to_string();
        let mut props = Vec::new();

        for input in &func.sig.inputs {
            if let FnArg::Typed(pat_type) = input {
                let prop_name = if let Pat::Ident(pat_ident) = &pat_type.pat { pat_ident.ident.to_string() } else { continue };

                let raw_type = format!("{:?}", &*pat_type.ty);
                let has_default = func.attrs.iter().any(|attr| {
                    if let Meta::List(list) = &attr.meta { list.path.is_ident("prop") && list.tokens.to_string().contains("default") } else { false }
                });

                props.push(RawPropExtraction {
                    name: prop_name,
                    raw_type,
                    has_default,
                });
            }
        }

        self.0.push(RawComponentExtraction { name, props });
    }
}

pub fn extract_rust_components(code: &str) -> Result<Vec<RawComponentExtraction>> {
    let ast = parse_file(code, Default::default())
        .map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;

    let mut visitor = ComponentVisitor(Vec::new());
    syn::visit::visit_file(&ast, &mut visitor);
    Ok(visitor.0)
}
