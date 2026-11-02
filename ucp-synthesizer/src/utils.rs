//! Small shared utilities used across the synthesizer.

use std::time::{SystemTime, UNIX_EPOCH};

use ucp_core::cam::CanonicalAbstractComponent;

/// Prefix marking a concrete type as an enum value list.
pub const ENUM_TYPE_PREFIX: &str = "enum: ";

/// True when the concrete type is an enum value list (`enum: a, b, c`).
#[must_use]
pub fn is_enum_type(concrete: &str) -> bool {
    concrete.starts_with(ENUM_TYPE_PREFIX)
}

/// Extract the allowed values from an `enum: a, b, c` concrete type.
///
/// Returns an empty vec for non-enum concrete types.
#[must_use]
pub fn parse_enum_values(concrete: &str) -> Vec<String> {
    match concrete.strip_prefix(ENUM_TYPE_PREFIX) {
        Some(rest) => rest.split(',').map(str::trim).map(String::from).collect(),
        None => Vec::new(),
    }
}

/// Map every enum-valued prop of a component to a variant record.
///
/// This is the shared shape used by the A2UI, AI-contract and W3C
/// exporters: props whose concrete type is an enum list become variants
/// carrying the canonical prop name and the allowed values.
#[must_use]
pub fn enum_variant_map<T>(
    comp: &CanonicalAbstractComponent,
    mut to_variant: impl FnMut(&str, Vec<String>) -> T,
) -> Vec<T> {
    comp.props
        .iter()
        .filter_map(|p| {
            let conc = p.concrete_type.as_deref()?;
            is_enum_type(conc).then(|| to_variant(&p.canonical_name, parse_enum_values(conc)))
        })
        .collect()
}

/// Normalises a type string produced by `syn`'s `ToTokens` output.
///
/// Strips all spacing around generic separators and preserves a single
/// space between words (e.g. `impl Into < T >` → `impl Into<T>`).
/// Runs in a single pass over the input characters.
#[must_use]
pub fn normalize_type_string(input: &str) -> String {
    // 1. Collapse all whitespace to single spaces.
    let collapsed: String = input.split_whitespace().collect::<Vec<_>>().join(" ");

    // 2. Remove spaces around `<>(),&` and after `<(&` in one pass.
    let mut out = String::with_capacity(collapsed.len());
    let mut chars = collapsed.chars().peekable();
    while let Some(ch) = chars.next() {
        if matches!(ch, '<' | '>' | '(' | ')' | ',' | '&') && out.ends_with(' ') {
            out.pop();
        }
        out.push(ch);
        if matches!(ch, '<' | '(' | '&') && chars.peek() == Some(&' ') {
            chars.next(); // skip the space
        }
    }

    // 3. Remove space before `+` (still one pass over the built string).
    if out.contains(" +") {
        out = out.replace(" +", "+");
    }

    out.trim().to_string()
}

/// Current UNIX timestamp as a string, defaulting to `"0"` if the clock is
/// before the epoch (cannot happen on supported platforms).
#[must_use]
pub fn unix_timestamp_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_generic_spacing() {
        assert_eq!(normalize_type_string("impl Into < T >"), "impl Into<T>");
        assert_eq!(
            normalize_type_string("impl Into < SharedString >"),
            "impl Into<SharedString>"
        );
        assert_eq!(
            normalize_type_string("Option < Signal < String > >"),
            "Option<Signal<String>>"
        );
        assert_eq!(normalize_type_string("dyn Fn ( T ) + 'a"), "dyn Fn(T)+ 'a");
    }

    #[test]
    fn timestamp_string_is_numeric() {
        let ts = unix_timestamp_string();
        assert_ne!(ts, "");
        assert!(ts.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn enum_type_helpers_round_trip() {
        assert!(is_enum_type("enum: A, B"));
        assert!(!is_enum_type("String"));
        assert_eq!(
            parse_enum_values("enum: Default, Destructive, Outline"),
            vec!["Default", "Destructive", "Outline"]
        );
        assert_eq!(parse_enum_values("bool"), Vec::<String>::new());
    }
}
