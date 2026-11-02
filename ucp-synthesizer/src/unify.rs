//! Mapping of raw framework type strings to the CAM abstract type ontology.

use ucp_core::cam::AbstractPropType;
use ucp_core::Result;

/// Map a raw type string to CAM abstract type (convenience wrapper).
///
/// # Errors
///
/// Never fails today; the signature is kept for forward compatibility with
/// stricter type parsing. Any unclassifiable type maps to
/// [`AbstractPropType::Any`].
pub fn map_raw_type_to_cam(raw_type: &str) -> Result<AbstractPropType> {
    Ok(map_raw_type_with_concrete(raw_type)?.0)
}

/// Map a raw type string to CAM abstract type and an optional concrete type.
///
/// The mapping is recursive: wrappers like `Option<Signal<String>>` unwrap
/// layer by layer, combining reactivity wrappers with their payload type.
///
/// # Errors
///
/// Never fails today; see [`map_raw_type_to_cam`].
pub fn map_raw_type_with_concrete(raw_type: &str) -> Result<(AbstractPropType, Option<String>)> {
    let clean = raw_type.replace(' ', "");

    // 1. Generic reactivity wrappers unwrap recursively.
    if let Some((wrapper, inner)) = split_generic(&clean) {
        let (inner_abs, inner_conc) = map_raw_type_with_concrete(&inner)?;
        let abs = match wrapper {
            "RwSignal" | "Signal" => AbstractPropType::ControlledValue(Box::new(inner_abs)),
            "MaybeSignal" | "MaybeProp" | "Option" => {
                AbstractPropType::UncontrolledValue(Box::new(inner_abs))
            }
            // `Vec`/`Array` are list shapes; TS `Array<T>` normalizes to `Vec<T>`.
            "Vec" | "Array" => {
                return Ok((
                    AbstractPropType::StaticValue(Box::new(inner_abs)),
                    inner_conc.map(|c| format!("Vec<{c}>")),
                ));
            }
            _ => unreachable!("split_generic only yields reactivity wrappers"),
        };
        let conc = inner_conc.map(|c| format!("{wrapper}<{c}>"));
        return Ok((abs, conc));
    }

    // 2. Event-handler-like types.
    if is_callback_like(&clean) {
        return Ok((AbstractPropType::AsyncEventHandler(vec![]), Some(clean)));
    }

    // 3. Renderable / children types.
    if is_renderable(&clean) {
        return Ok((AbstractPropType::Renderable, Some(clean)));
    }

    // 4. Map-like collections hold arbitrary payloads.
    if is_map_like(&clean) {
        return Ok((
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some(clean),
        ));
    }

    // 5. Primitives and simple scalar shapes.
    match clean.as_str() {
        "bool" => return Ok((AbstractPropType::ControlFlag, Some("bool".to_string()))),
        "String" | "&str" | "SharedString" | "Cow<str>" => {
            return Ok((
                AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                Some("String".to_string()),
            ));
        }
        "usize" | "isize" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32"
        | "f64" => {
            return Ok((
                AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
                Some(clean.clone()),
            ));
        }
        _ => {}
    }

    // 6. Custom types: capitalized identifiers are treated as named types.
    //     `implInto<T>` bounds are Leptos shorthand for "anything string-like".
    if clean.starts_with("implInto<")
        || clean.chars().next().is_some_and(|c| c.is_ascii_uppercase())
    {
        return Ok((
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some(clean),
        ));
    }

    Ok((AbstractPropType::Any, Some(clean)))
}

/// If `s` has the shape `Wrapper<inner>`, return `(Wrapper, inner)`.
///
/// Only exact `Wrapper<` prefixes match — `VecDeque<u8>` is not a `Vec`.
fn split_generic(s: &str) -> Option<(&str, String)> {
    let (wrapper, rest) = KNOWN_WRAPPERS.iter().find_map(|w| {
        let mut prefixed = String::with_capacity(w.len() + 1);
        prefixed.push_str(w);
        prefixed.push('<');
        s.strip_prefix(prefixed.as_str()).map(|r| (*w, r))
    })?;
    // Unwrap exactly one closing angle bracket (the outermost group).
    let inner = rest.strip_suffix('>')?;
    Some((wrapper, inner.to_string()))
}

/// Reactivity/collection wrappers that unwrap recursively. Callbacks,
/// event handlers and map-like types are intentionally **not** listed here:
/// they are matched by [`is_callback_like`] / [`is_map_like`] with
/// different concrete-type semantics.
const KNOWN_WRAPPERS: &[&str] = &[
    "RwSignal",
    "Signal",
    "MaybeSignal",
    "MaybeProp",
    "Option",
    "Vec",
    "Array",
];

/// Types that behave as event handlers / callbacks.
fn is_callback_like(t: &str) -> bool {
    t.starts_with("Callback<")
        || t.starts_with("EventHandler<")
        || t.starts_with("Fn(")
        || t.starts_with("fn(")
        || t.starts_with("implFn(")
        || t.starts_with("implfor<")
        || t.starts_with("Box<dynFn(")
        || t.starts_with("Box<dynfor<")
        || t.contains("=>")
}

/// Types that carry renderable children (slots).
fn is_renderable(t: &str) -> bool {
    matches!(
        t,
        "Children" | "View" | "Element" | "ReactNode" | "ReactElement" | "VNode"
    ) || t.contains("IntoElement")
        || t.contains("IntoView")
        || t.contains("HtmlElement")
}

/// Types that are key→value collections.
fn is_map_like(t: &str) -> bool {
    t.starts_with("Record<")
        || t.starts_with("HashMap<")
        || t.starts_with("BTreeMap<")
        || t.starts_with("Map<")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rw_signal_string_to_controlled_value() {
        let cam = map_raw_type_to_cam("RwSignal < String >").unwrap();
        assert!(matches!(cam, AbstractPropType::ControlledValue(_)));
    }

    #[test]
    fn maybe_signal_bool_to_uncontrolled_value() {
        let cam = map_raw_type_to_cam("MaybeSignal < bool >").unwrap();
        assert!(matches!(cam, AbstractPropType::UncontrolledValue(_)));
    }

    #[test]
    fn option_string_to_uncontrolled_value() {
        let cam = map_raw_type_to_cam("Option < String >").unwrap();
        assert!(matches!(cam, AbstractPropType::UncontrolledValue(_)));
    }

    #[test]
    fn vec_string_to_static_value() {
        let cam = map_raw_type_to_cam("Vec < String >").unwrap();
        assert!(matches!(cam, AbstractPropType::StaticValue(_)));
    }

    #[test]
    fn array_to_static_value() {
        let cam = map_raw_type_to_cam("Array<string>").unwrap();
        assert!(matches!(cam, AbstractPropType::StaticValue(_)));
    }

    #[test]
    fn record_to_static_any() {
        let cam = map_raw_type_to_cam("Record<string, number>").unwrap();
        assert!(matches!(cam, AbstractPropType::StaticValue(_)));
    }

    #[test]
    fn callback_generic_to_event_handler() {
        let cam = map_raw_type_to_cam("Callback<String>").unwrap();
        assert!(matches!(cam, AbstractPropType::AsyncEventHandler(_)));
    }

    #[test]
    fn plain_bool_to_control_flag() {
        assert_eq!(
            map_raw_type_to_cam("bool").unwrap(),
            AbstractPropType::ControlFlag
        );
    }

    #[test]
    fn string_to_static_value() {
        let cam = map_raw_type_to_cam("String").unwrap();
        assert!(matches!(cam, AbstractPropType::StaticValue(_)));
    }

    #[test]
    fn react_node_to_renderable() {
        assert_eq!(
            map_raw_type_to_cam("ReactNode").unwrap(),
            AbstractPropType::Renderable
        );
    }

    #[test]
    fn vnode_to_renderable() {
        assert_eq!(
            map_raw_type_to_cam("VNode").unwrap(),
            AbstractPropType::Renderable
        );
    }

    #[test]
    fn children_to_renderable() {
        assert_eq!(
            map_raw_type_to_cam("Children").unwrap(),
            AbstractPropType::Renderable
        );
    }

    #[test]
    fn unknown_type_to_any() {
        let cam = map_raw_type_to_cam("CustomWidget").unwrap();
        assert_eq!(
            cam,
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any))
        );
    }

    #[test]
    fn nested_signal_unwraps_recursively() {
        let cam = map_raw_type_to_cam("Signal < MaybeSignal < String > >").unwrap();
        match cam {
            AbstractPropType::ControlledValue(inner) => {
                assert!(matches!(*inner, AbstractPropType::UncontrolledValue(_)));
            }
            other => panic!("Expected ControlledValue, got {other:?}"),
        }
    }

    #[test]
    fn option_wrapping_signal_unwraps_both() {
        let cam = map_raw_type_to_cam("Option < Signal < String > >").unwrap();
        match cam {
            AbstractPropType::UncontrolledValue(outer) => match outer.as_ref() {
                AbstractPropType::ControlledValue(inner) => {
                    assert!(matches!(inner.as_ref(), AbstractPropType::StaticValue(_)));
                }
                other => panic!("Expected ControlledValue inside UncontrolledValue, got {other:?}"),
            },
            other => panic!("Expected UncontrolledValue, got {other:?}"),
        }
    }

    #[test]
    fn concrete_type_for_bool() {
        let (abs, conc) = map_raw_type_with_concrete("bool").unwrap();
        assert_eq!(abs, AbstractPropType::ControlFlag);
        assert_eq!(conc, Some("bool".to_string()));
    }

    #[test]
    fn concrete_type_for_string() {
        let (abs, conc) = map_raw_type_with_concrete("String").unwrap();
        assert!(matches!(abs, AbstractPropType::StaticValue(_)));
        assert_eq!(conc, Some("String".to_string()));
    }

    #[test]
    fn concrete_type_for_enum() {
        let (abs, conc) = map_raw_type_with_concrete("ButtonVariant").unwrap();
        assert!(matches!(abs, AbstractPropType::StaticValue(_)));
        assert_eq!(conc, Some("ButtonVariant".to_string()));
    }

    #[test]
    fn kind_name_and_innermost_helpers() {
        assert_eq!(
            AbstractPropType::ControlledValue(Box::new(AbstractPropType::ControlFlag)).innermost(),
            &AbstractPropType::ControlFlag
        );
        assert_eq!(
            AbstractPropType::ControlledValue(Box::new(AbstractPropType::ControlFlag)).kind_name(),
            "controlledValue"
        );
        assert_eq!(AbstractPropType::Any.kind_name(), "any");
        assert_eq!(AbstractPropType::Renderable.kind_name(), "renderable");
    }

    #[test]
    fn handler_impl_and_box_forms_map_to_events() {
        for ty in [
            "implFn(Event) -> bool",
            "implfor<'a>Fn()",
            "Box<dynFn(Event)>",
            "EventHandler<Click>",
            "fn(bool)",
        ] {
            let cam = map_raw_type_to_cam(ty).unwrap();
            assert!(
                matches!(cam, AbstractPropType::AsyncEventHandler(_)),
                "{ty} should be an event handler"
            );
        }
    }
}
