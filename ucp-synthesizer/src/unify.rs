use ucp_core::{cam::AbstractPropType, Result};

/// Map a raw type string (e.g. from ToTokens output) to the CAM AbstractPropType
/// using strict pattern matching rules.
pub fn map_raw_type_to_cam(raw_type: &str) -> Result<AbstractPropType> {
    // Normalize: syn's ToTokens puts spaces around angle brackets:
    //   "RwSignal < String >", "MaybeSignal < bool >", "Option < Vec < String > >"
    // Remove all spaces for consistent matching.
    let clean = raw_type.replace(' ', "");

    // Reactive signal wrappers → ControlledValue(inner)
    if clean.starts_with("RwSignal<") || clean.starts_with("Signal<") {
        let inner = extract_generic_inner(&clean);
        return Ok(AbstractPropType::ControlledValue(Box::new(
            map_raw_type_to_cam(&inner)?,
        )));
    }

    // Maybe-reactive wrappers → UncontrolledValue(inner)
    if clean.starts_with("MaybeSignal<") || clean.starts_with("MaybeProp<") {
        let inner = extract_generic_inner(&clean);
        return Ok(AbstractPropType::UncontrolledValue(Box::new(
            map_raw_type_to_cam(&inner)?,
        )));
    }

    // Option<T> → UncontrolledValue(inner)
    if clean.starts_with("Option<") {
        let inner = extract_generic_inner(&clean);
        return Ok(AbstractPropType::UncontrolledValue(Box::new(
            map_raw_type_to_cam(&inner)?,
        )));
    }

    // Vec<T>, Array<T> → StaticValue(inner)
    if clean.starts_with("Vec<") || clean.starts_with("Array<") {
        let inner = extract_generic_inner(&clean);
        return Ok(AbstractPropType::StaticValue(Box::new(
            map_raw_type_to_cam(&inner)?,
        )));
    }

    // Record<K, V>, HashMap<K, V>, BTreeMap<K, V>, Map<K, V> → StaticValue(Any)
    if clean.starts_with("Record<")
        || clean.starts_with("HashMap<")
        || clean.starts_with("BTreeMap<")
        || clean.starts_with("Map<")
    {
        return Ok(AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)));
    }

    // Callback / event handler patterns → AsyncEventHandler
    if clean.starts_with("Callback<")
        || clean.starts_with("EventHandler<")
        || clean.starts_with("Fn(")
        || clean.starts_with("fn(")
    {
        return Ok(AbstractPropType::AsyncEventHandler(vec![]));
    }

    // Arrow function patterns (TSX)
    if clean.contains("=>") {
        return Ok(AbstractPropType::AsyncEventHandler(vec![]));
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
        return Ok(AbstractPropType::Renderable);
    }

    // Leaf types
    match clean.as_str() {
        "bool" => Ok(AbstractPropType::ControlFlag),
        "String" | "&str" | "SharedString" | "Cow<str>" => {
            Ok(AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)))
        }
        "usize" | "isize" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32"
        | "i64" | "f32" | "f64" => {
            Ok(AbstractPropType::StaticValue(Box::new(AbstractPropType::Any)))
        }
        _ => Ok(AbstractPropType::Any),
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
        assert_eq!(cam, AbstractPropType::Any);
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
            AbstractPropType::UncontrolledValue(outer) => {
                match outer.as_ref() {
                    AbstractPropType::ControlledValue(inner) => {
                        assert!(matches!(inner.as_ref(), AbstractPropType::StaticValue(_)));
                    }
                    other => panic!("Expected ControlledValue inside UncontrolledValue, got {:?}", other),
                }
            }
            other => panic!("Expected UncontrolledValue, got {:?}", other),
        }
    }
}
