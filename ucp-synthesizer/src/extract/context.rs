//! Detection of provided/consumed contexts (React `createContext`,
//! Leptos `provide_context`) in source files.

/// Contexts provided and consumed by the code in one source file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContextUsage {
    /// Type name of the provided context, if any.
    pub provided: Option<String>,
    /// Type names of consumed contexts.
    pub consumed: Vec<String>,
}

impl ContextUsage {
    /// True when neither a provided nor consumed context was found.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.provided.is_none() && self.consumed.is_empty()
    }
}

/// Extract the type argument of the first generic annotation in `after`,
/// e.g. `::<Theme>` or `<Theme>` → `Some("Theme")`.
fn generic_type_arg(after: &str) -> Option<String> {
    let trimmed = after.trim_start();
    let rest = trimmed
        .strip_prefix("::<")
        .or_else(|| trimmed.strip_prefix('<'))?;
    let end = rest.find('>')?;
    let ty = rest[..end].trim();
    (!ty.is_empty()).then(|| ty.to_string())
}

/// Detect Leptos context usage: `provide_context::<T>(...)` and
/// `use_context::<T>()`.
#[must_use]
pub fn detect_leptos_context(source: &str) -> ContextUsage {
    let mut usage = ContextUsage::default();
    for line in source.lines() {
        let t = line.trim();
        if let Some(pos) = t.find("provide_context") {
            if let Some(ty) = generic_type_arg(&t[pos + "provide_context".len()..]) {
                usage.provided.get_or_insert(ty);
            }
        }
        if let Some(pos) = t.find("use_context") {
            if let Some(ty) = generic_type_arg(&t[pos + "use_context".len()..]) {
                usage.consumed.push(ty);
            }
        }
    }
    usage
}

/// Detect React context usage: `createContext<T>(...)` and
/// `useContext<T>()`.
#[must_use]
pub fn detect_react_context(source: &str) -> ContextUsage {
    let mut usage = ContextUsage::default();
    for line in source.lines() {
        let t = line.trim();
        if let Some(pos) = t.find("createContext") {
            let after = &t[pos + "createContext".len()..];
            let ty = generic_type_arg(after).or_else(|| {
                // Fallback: `createContext(SomeValue)` — only accept
                // capitalized values, matching the original heuristic.
                let open = after.find('(')?;
                let inner = &after[open + 1..];
                let close = inner.find(')')?;
                let value = inner[..close].trim();
                (!value.is_empty() && value.chars().next().is_some_and(char::is_uppercase))
                    .then(|| value.to_string())
            });
            if let Some(ty) = ty {
                usage.provided.get_or_insert(ty);
            }
        }
        if let Some(pos) = t.find("useContext") {
            if let Some(ty) = generic_type_arg(&t[pos + "useContext".len()..]) {
                usage.consumed.push(ty);
            }
        }
    }
    usage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leptos_provide_and_use_context() {
        let src = r"
            fn app() {
                provide_context::<ThemeConfig>(dark_theme);
                let style = use_context::<StyleGuide>();
            }
        ";
        let usage = detect_leptos_context(src);
        assert_eq!(usage.provided.as_deref(), Some("ThemeConfig"));
        assert_eq!(usage.consumed, vec!["StyleGuide"]);
    }

    #[test]
    fn leptos_non_generic_calls_are_ignored() {
        let usage = detect_leptos_context("provide_context(value); use_context();");
        assert!(usage.is_empty());
    }

    #[test]
    fn react_create_and_use_context() {
        let src = "
            const ThemeCtx = createContext<Theme>(defaultTheme);
            const theme = useContext<Theme>(ThemeCtx);
        ";
        let usage = detect_react_context(src);
        assert_eq!(usage.provided.as_deref(), Some("Theme"));
        assert_eq!(usage.consumed, vec!["Theme"]);
    }

    #[test]
    fn react_lowercase_context_values_are_ignored() {
        let usage = detect_react_context("const Ctx = createContext(null);");
        assert!(usage.is_empty());
    }

    #[test]
    fn empty_source_is_empty_usage() {
        assert!(detect_leptos_context("").is_empty());
        assert!(detect_react_context("").is_empty());
    }
}
