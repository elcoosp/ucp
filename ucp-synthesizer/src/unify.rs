use regex::Regex;
use ucp_core::{cam::AbstractPropType, Result};

/// Map a raw type string (e.g. from syn Debug output) to the CAM AbstractPropType
/// using strict regex-based ontology rules.
pub fn map_raw_type_to_cam(raw_type: &str) -> Result<AbstractPropType> {
    // syn Debug formats types with spaces around angle brackets:
    //   "RwSignal < String >", "MaybeSignal < bool >"
    // Normalize by removing all spaces for consistent matching.
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

    // Callback / event handler patterns → AsyncEventHandler
    if clean.contains("Callback")
        || clean.contains("EventHandler")
        || clean.contains("Fn(")
        || clean.contains("fn(")
    {
        return Ok(AbstractPropType::AsyncEventHandler(vec![]));
    }

    // Renderable children / slots → Renderable
    if clean == "Children"
        || clean == "View"
        || clean == "Element"
        || clean.contains("IntoElement")
        || clean.contains("IntoView")
        || clean.contains("HtmlElement")
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

/// Extract the inner type from a generic like `Signal<String>` → `String`
fn extract_generic_inner(type_str: &str) -> String {
    let re = Regex::new(r"<(.+)>$").unwrap();
    re.captures(type_str)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| type_str.to_string())
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
    fn callback_to_async_event_handler() {
        let cam = map_raw_type_to_cam("Callback<String>").unwrap();
        assert!(matches!(cam, AbstractPropType::AsyncEventHandler(_)));
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
}
