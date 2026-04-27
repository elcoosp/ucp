# How‑to: Add a New Framework Extractor

Add support for extracting components from a new framework.

## 1. Understand the Data Model

An extractor produces `RawComponentExtraction` or `RawTsxExtraction`
structs, which the unifier converts to `CanonicalAbstractComponent`.

Key fields:
- `name`: component name (e.g., `Button`)
- `line_start`: line number in source file
- `props`: vector of `RawPropExtraction` (name, raw_type, has_default, is_event)
- `is_struct_pattern`: true if the component uses a props struct (Dioxus/Leptos)

## 2. Create the Extractor Module

Add a new file: `ucp-synthesizer/src/extract/myframework_ast.rs`

``` rust
use super::rust_ast::RawComponentExtraction;
use ucp_core::Result;

pub fn extract_myframework_components(source: &str) -> Result<Vec<RawComponentExtraction>> {
    let mut components = Vec::new();
    // Parse the source and extract components
    // ...
    Ok(components)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_basic_component() {
        let source = r#"... your framework's component syntax ..."#;
        let comps = extract_myframework_components(source).unwrap();
        assert!(!comps.is_empty());
        assert_eq!(comps[0].name, "ExpectedName");
    }
}
```

## 3. Register the Extractor

Add the new module in `ucp-synthesizer/src/extract/mod.rs`:

``` rust
pub mod myframework_ast;
```

Then integrate it into the pipeline in `ucp-synthesizer/src/pipeline/extraction.rs`:

``` rust
// After the existing framework extractions:
match ext {
    // ... existing match arms ...
    Some("myext") => {
        if let Ok(components) = myframework_ast::extract_myframework_components(&content) {
            if !components.is_empty() {
                my_extractions.insert(path_str, components);
                files_parsed += 1;
            }
        }
    }
    // ...
}
```

## 4. Add Type Mapping (if needed)

If your framework uses types not covered by the existing `unify.rs`
mapping, extend the `map_raw_type_with_concrete` function.

## 5. Write Tests

Include:
- Unit tests for parsing individual components
- Tests for edge cases (empty source, malformed code, nested components)
- A pipeline integration test that bootstraps a real source file

## 6. Update Documentation

Add your framework to:
- This guide's table of supported extractors
- `docs/user/tutorial-bootstrap.md` (example syntax)
- `docs/maintainer/pipeline.md` (extractor table)
