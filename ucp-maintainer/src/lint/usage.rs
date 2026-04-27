//! Component usage representation extracted from source files.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum DetectedType {
    String,
    Number,
    Bool,
    Object,
    Expression,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropAssignment {
    pub name: String,
    pub value_text: String,
    pub detected_type: DetectedType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentUsage {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub element_name: String,
    pub resolved_name: Option<String>,
    pub props: Vec<PropAssignment>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_usage() -> ComponentUsage {
        ComponentUsage {
            file: PathBuf::from("src/App.jsx"),
            line: 1,
            column: 0,
            element_name: "Button".into(),
            resolved_name: None,
            props: vec![],
        }
    }

    #[test]
    fn usage_creation() {
        let u = empty_usage();
        assert_eq!(u.element_name, "Button");
        assert!(u.props.is_empty());
    }

    #[test]
    fn detected_type_default_is_unknown() {
        assert_eq!(DetectedType::Unknown as u32, 0);
    }
}
