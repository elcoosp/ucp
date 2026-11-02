//! Cross-convention naming normalization tests.
//!
//! These tests define the contract for [`ucp_core::naming`]: the same logical
//! prop must normalize identically whether a port spells it `onClick`,
//! `on_click`, `on-click`, or `OnClick`.

use ucp_core::naming::{normalize_component_name, normalize_event_name, normalize_prop_name};

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn snake_camel_kebab_converge() {
    assert_eq!(
        normalize_prop_name("on_click"),
        normalize_prop_name("onClick")
    );
    assert_eq!(
        normalize_prop_name("on-click"),
        normalize_prop_name("onClick")
    );
    assert_eq!(
        normalize_prop_name("OnClick"),
        normalize_prop_name("onClick")
    );
    assert_eq!(
        normalize_prop_name("on_open_change"),
        normalize_prop_name("onOpenChange")
    );
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn ordinary_props_ignore_case_and_separators() {
    assert_eq!(normalize_prop_name("disabled"), "disabled");
    assert_eq!(normalize_prop_name("aria-label"), "arialabel");
    assert_eq!(normalize_prop_name("variant"), "variant");
    assert_eq!(normalize_prop_name("defaultChecked"), "defaultchecked");
    assert_eq!(normalize_prop_name("default_checked"), "defaultchecked");
}

/// `on`-prefixed props reduce to their bare event stem so callback props from
/// different frameworks cluster together.
///
/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn on_prefix_reduces_to_event_stem() {
    assert_eq!(normalize_prop_name("onClick"), "click");
    assert_eq!(normalize_prop_name("on_click"), "click");
    assert_eq!(normalize_prop_name("onOpenChange"), "openchange");
    // Bare event-style names are already stems.
    assert_eq!(normalize_prop_name("click"), "click");
}

/// The `on` prefix must only strip when it stands alone as a word, not when
/// it is part of a longer word (`once`, `onset`, `only`).
///
/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn on_prefix_requires_word_boundary() {
    assert_eq!(normalize_prop_name("once"), "once");
    assert_eq!(normalize_prop_name("onlyChild"), "onlychild");
    assert_eq!(normalize_prop_name("onset"), "onset");
}

/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn normalize_event_name_strips_on_prefix() {
    assert_eq!(normalize_event_name("onClick"), "click");
    assert_eq!(normalize_event_name("on_open_change"), "openchange");
    assert_eq!(normalize_event_name("change"), "change");
}

/// Component names normalize case-insensitively without separator folding
/// (`PascalCase` is the dominant convention and must stay readable in hashes).
///
/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn component_names_fold_case_only() {
    assert_eq!(normalize_component_name("Button"), "button");
    assert_eq!(normalize_component_name("BUTTON"), "button");
    assert_eq!(normalize_component_name("button"), "button");
    // Different words stay different even when separators differ in case.
    assert_ne!(
        normalize_component_name("AlertDialog"),
        normalize_component_name("Alert_Dialog")
    );
}

/// Edge cases: empty and odd identifiers must not panic and must remain
/// stable.
///
/// # Panics
///
/// Panics when an assertion fails.
#[test]
fn edge_cases_are_stable() {
    assert_eq!(normalize_prop_name(""), "");
    // Separators carry no identity on their own.
    assert_eq!(normalize_prop_name("___"), "");
    assert_eq!(normalize_prop_name("---"), "");
    assert_eq!(normalize_prop_name("a"), "a");
    assert_eq!(normalize_prop_name("2fa"), "2fa");
}

mod proptests {
    use proptest::prelude::*;
    use ucp_core::naming::{normalize_event_name, normalize_prop_name};

    proptest! {
        /// Normalization never panics and is idempotent.
        #[test]
        fn normalization_is_idempotent(name in "[a-zA-Z][a-zA-Z0-9_-]{0,30}") {
            let once = normalize_prop_name(&name);
            let twice = normalize_prop_name(&once);
            prop_assert_eq!(once, twice);
        }

        /// Separator style never changes the normalized form for on-prefixed
        /// props: `on_x`, `onX`, `on-x` all collapse to the same stem. Fully
        /// uppercase stems (SCREAMING_CASE) are excluded: a monocase remainder
        /// (`onABC`) is information-theoretically ambiguous against ordinary
        /// words (`ONCE`) and is by contract kept whole.
        #[test]
        fn separator_styles_converge(stem in "[a-zA-Z][a-zA-Z0-9]{0,20}") {
            let camel_first = {
                let mut c = stem.chars();
                match c.next() {
                    Some(f) => format!("on{}", f.to_uppercase().to_string() + c.as_str()),
                    None => return Ok(()),
                }
            };
            prop_assume!(camel_first[2..].chars().any(|c| c.is_ascii_lowercase()));
            let snake = format!("on_{stem}");
            let kebab = format!("on-{stem}");
            let a = normalize_prop_name(&snake);
            let b = normalize_prop_name(&camel_first);
            let c = normalize_prop_name(&kebab);
            prop_assert_eq!(a.clone(), b);
            prop_assert_eq!(a, c);
        }

        /// Event normalization is a prefix-preserving special case of prop
        /// normalization for on-prefixed names.
        #[test]
        fn event_normalization_matches_prop_normalization(name in "on[a-zA-Z0-9_]{1,30}") {
            prop_assert_eq!(normalize_event_name(&name), normalize_prop_name(&name));
        }
    }
}
