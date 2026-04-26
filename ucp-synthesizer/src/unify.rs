use ucp_core::{cam::AbstractPropType, Result};

/// Map a raw type string to CAM abstract type, and also produce a normalised concrete type.
pub fn map_raw_type_to_cam(raw_type: &str) -> Result<AbstractPropType> {
    let (abstract_type, _) = map_raw_type_with_concrete(raw_type)?;
    Ok(abstract_type)
}

/// Same as map_raw_type_to_cam, but also returns an optional concrete type string.
pub fn map_raw_type_with_concrete(raw_type: &str) -> Result<(AbstractPropType, Option<String>)> {
    let clean = raw_type.replace(' ', "");

    // Reactive signal wrappers → ControlledValue(inner)
    if clean.starts_with("RwSignal<") || clean.starts_with("Signal<") {
        let inner = extract_generic_inner(&clean);
        let (inner_abs, inner_conc) = map_raw_type_with_concrete(&inner)?;
        return Ok((
            AbstractPropType::ControlledValue(Box::new(inner_abs)),
            inner_conc.map(|c| format!("Signal<{}>", c)),
        ));
    }

    // Maybe-reactive wrappers → UncontrolledValue(inner)
    if clean.starts_with("MaybeSignal<") || clean.starts_with("MaybeProp<") {
        let inner = extract_generic_inner(&clean);
        let (inner_abs, inner_conc) = map_raw_type_with_concrete(&inner)?;
        return Ok((
            AbstractPropType::UncontrolledValue(Box::new(inner_abs)),
            inner_conc.map(|c| format!("MaybeSignal<{}>", c)),
        ));
    }

    // Option<T> → UncontrolledValue(inner)
    if clean.starts_with("Option<") {
        let inner = extract_generic_inner(&clean);
        let (inner_abs, inner_conc) = map_raw_type_with_concrete(&inner)?;
        return Ok((
            AbstractPropType::UncontrolledValue(Box::new(inner_abs)),
            inner_conc.map(|c| format!("Option<{}>", c)),
        ));
    }

    // Vec<T>, Array<T> → StaticValue(inner)
    if clean.starts_with("Vec<") || clean.starts_with("Array<") {
        let inner = extract_generic_inner(&clean);
        let (inner_abs, inner_conc) = map_raw_type_with_concrete(&inner)?;
        return Ok((
            AbstractPropType::StaticValue(Box::new(inner_abs)),
            inner_conc.map(|c| format!("Vec<{}>", c)),
        ));
    }

    // Record / Map → StaticValue(Any)
    if clean.starts_with("Record<")
        || clean.starts_with("HashMap<")
        || clean.starts_with("BTreeMap<")
        || clean.starts_with("Map<")
    {
        return Ok((
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some(clean.to_string()),
        ));
    }

    // Callback / event handler patterns → AsyncEventHandler
    if clean.starts_with("Callback<")
        || clean.starts_with("EventHandler<")
        || clean.starts_with("Fn(")
        || clean.starts_with("fn(")
        || clean.contains("=>")
    {
        return Ok((AbstractPropType::AsyncEventHandler(vec![]), Some(clean.to_string())));
    }

    // Renderable children / slots → Renderable
    if clean == "Children"
        || clean == "View"
        || clean == "Element"
        || clean == "ReactNode"
        || clean == "ReactElement"
        || clean.contains("IntoElement")
        || clean.contains("IntoView")
        || clean.contains("HtmlElement")
        || clean == "VNode"
    {
        return Ok((AbstractPropType::Renderable, Some(clean.to_string())));
    }

    // Leaf types
    match clean.as_str() {
        "bool" => Ok((AbstractPropType::ControlFlag, Some("bool".to_string()))),
        "String" | "&str" | "SharedString" | "Cow<str>" => Ok((
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some("String".to_string()),
        )),
        "usize" | "isize" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32"
        | "f64" => Ok((
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some(clean.clone()),
        )),
        // impl Into<T> → StaticValue(Any)
        clean if clean.starts_with("implInto<") => Ok((
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some(clean.to_string()),
        )),
        // impl Fn(...) → AsyncEventHandler
        clean if clean.starts_with("implFn(") || clean.starts_with("implfor<") || clean.starts_with("fn(") => Ok((
            AbstractPropType::AsyncEventHandler(vec![]),
            Some(clean.to_string()),
        )),
        // Box<dyn Fn...> → AsyncEventHandler
        clean if clean.starts_with("Box<dynFn(") || clean.starts_with("Box<dynfor<") => Ok((
            AbstractPropType::AsyncEventHandler(vec![]),
            Some(clean.to_string()),
        )),
        // unknown type starting with uppercase → StaticValue(Any) with concrete
        clean if clean.chars().next().map_or(false, |c| c.is_uppercase()) => Ok((
            AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)),
            Some(clean.to_string()),
        )),
        _ => Ok((AbstractPropType::Any, Some(clean.to_string()))),
    }
}

/// Extract the inner type from a generic like `Signal<String>` → `String`.
fn extract_generic_inner(type_str: &str) -> String {
    if let Some(start) = type_str.find('<') {
        if let Some(end) = type_str.rfind('>') {
            if start < end {
                return type_str[start + 1..end].to_string();
            }
        }
    }
    type_str.to_string()
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
        let cam = map_raw_type_to_cam("bool").unwrap();
        assert_eq!(cam, AbstractPropType::ControlFlag);
    }

    #[test]
    fn string_to_static_value() {
        let cam = map_raw_type_to_cam("String").unwrap();
        assert!(matches!(cam, AbstractPropType::StaticValue(_)));
    }

    #[test]
    fn react_node_to_renderable() {
        let cam = map_raw_type_to_cam("ReactNode").unwrap();
        assert_eq!(cam, AbstractPropType::Renderable);
    }

    #[test]
    fn vnode_to_renderable() {
        let cam = map_raw_type_to_cam("VNode").unwrap();
        assert_eq!(cam, AbstractPropType::Renderable);
    }

    #[test]
    fn children_to_renderable() {
        let cam = map_raw_type_to_cam("Children").unwrap();
        assert_eq!(cam, AbstractPropType::Renderable);
    }

    #[test]
    fn unknown_type_to_any() {
        let cam = map_raw_type_to_cam("CustomWidget").unwrap();
        assert_eq!(cam, AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)));
    }

    #[test]
    fn nested_signal_unwraps_recursively() {
        let cam = map_raw_type_to_cam("Signal < MaybeSignal < String > >").unwrap();
        match cam {
            AbstractPropType::ControlledValue(inner) => {
                assert!(matches!(*inner, AbstractPropType::UncontrolledValue(_)));
            }
            other => panic!("Expected ControlledValue, got {:?}", other),
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
                other => panic!(
                    "Expected ControlledValue inside UncontrolledValue, got {:?}",
                    other
                ),
            },
            other => panic!("Expected UncontrolledValue, got {:?}", other),
        }
    }

    // New tests for concrete type return
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
}
