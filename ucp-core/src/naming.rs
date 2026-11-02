//! Cross-convention identifier normalization.
//!
//! Component ports in different frameworks spell the same prop differently:
//! React writes `onClick`, Leptos writes `on_click`, Vue writes `on-click`.
//! Cross-framework clustering must therefore compare *normalized* names, not
//! raw source spellings.
//!
//! Normalization rules:
//! - Case folding: `Variant` and `variant` are the same prop.
//! - Separator folding: `_` and `-` are dropped entirely.
//! - Event stems: a leading standalone `on` word is stripped from props
//!   (`onClick` → `click`), because callback props and their derived events
//!   (`click`) must cluster together. The strip applies only when `on` is a
//!   complete word in the *original* spelling — followed by a separator, an
//!   uppercase letter, a digit, or the end of the name. Plain lowercase words
//!   (`once`, `onset`, `onlyChild`) are therefore never mangled.

/// Normalize a component prop name for cross-framework comparison.
///
/// The transformation is idempotent and locale-independent (ASCII case
/// folding only), so the result is stable across platforms and releases.
#[must_use]
pub fn normalize_prop_name(name: &str) -> String {
    match event_stem(name) {
        Some(stem) => fold_name(stem),
        None => fold_name(name),
    }
}

/// Normalize a derived event name (same rules as [`normalize_prop_name`]).
///
/// Events are conventionally named `click`, `openChange`, ... but raw
/// callback props (`onClick`, `on_open_change`) also flow through here, so
/// both spellings collapse to one event identity.
#[must_use]
pub fn normalize_event_name(name: &str) -> String {
    normalize_prop_name(name)
}

/// Normalize a component name: case folding only, separators preserved.
///
/// Component names use `PascalCase` in every supported framework, so folding
/// case without dropping separators keeps distinct multi-word components
/// (`AlertDialog` vs `Alert_Dialog`) distinguishable.
#[must_use]
pub fn normalize_component_name(name: &str) -> String {
    name.to_lowercase()
}

/// Lowercase and remove `_`/`-` separators.
fn fold_name(name: &str) -> String {
    name.chars()
        .filter(|c| *c != '_' && *c != '-')
        .flat_map(char::to_lowercase)
        .collect()
}

/// If `name` begins with a standalone `on` word, return the remainder.
///
/// "Standalone" is judged on the original spelling so that ordinary words
/// starting with `on` (`once`, `onset`) are left intact. A mixed-case
/// remainder (`onClick`) marks a camelCase word boundary; a fully-uppercase
/// remainder (`ONCE`, `SCREAMING_CASE`) is treated as one word and never
/// split, which keeps screaming-case spellings consistent with their
/// lowercase folds.
fn event_stem(name: &str) -> Option<&str> {
    let bytes = name.as_bytes();
    if bytes.len() < 2 || bytes[0] | 0x20 != b'o' || bytes[1] | 0x20 != b'n' {
        return None;
    }
    let rest = &name[2..];
    let next = rest.chars().next()?;
    let camel_boundary = next.is_ascii_uppercase()
        && !rest
            .chars()
            .all(|c| !c.is_ascii_alphabetic() || c.is_ascii_uppercase());
    if matches!(next, '_' | '-') || next.is_ascii_digit() || camel_boundary {
        Some(rest)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold_name_drops_separators_and_case() {
        assert_eq!(fold_name("Default_Checked"), "defaultchecked");
        assert_eq!(fold_name("aria-label"), "arialabel");
    }

    #[test]
    fn event_stem_requires_word_boundary() {
        assert_eq!(event_stem("onClick"), Some("Click"));
        assert_eq!(event_stem("on_click"), Some("_click"));
        assert_eq!(event_stem("onOpenChange"), Some("OpenChange"));
        assert_eq!(event_stem("on2fa"), Some("2fa"));
        assert_eq!(event_stem("ONCE"), None);
        assert_eq!(event_stem("onset"), None);
        assert_eq!(event_stem("once"), None);
        assert_eq!(event_stem("onlyChild"), None);
        assert_eq!(event_stem("on"), None);
        assert_eq!(event_stem("a"), None);
        assert_eq!(event_stem(""), None);
    }
}
