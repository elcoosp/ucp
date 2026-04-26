/// Normalises a type string produced by `syn`'s `ToTokens` output.
/// Strips all spacing around generic separators and preserves a single space
/// between words (e.g. `impl Into < T >` → `impl Into<T>`).
/// Normalises a type string produced by `syn`'s `ToTokens` output.
/// Strips all spacing around generic separators and preserves a single space
/// between words (e.g. `impl Into < SharedString >` → `impl Into<SharedString>`).
pub(crate) fn normalize_type_string(input: &str) -> String {
    // 1. Collapse all whitespace to single spaces
    let collapsed: String = input.split_whitespace().collect::<Vec<_>>().join(" ");

    // 2. Remove spaces around `<>`, `()`, `,`, `&`, `+`
    let mut out = String::with_capacity(collapsed.len());
    let chars: Vec<char> = collapsed.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        // If this char is '<', '>', '(', ')', ',' or '&', strip the space before it (if any)
        if matches!(ch, '<' | '>' | '(' | ')' | ',' | '&') && out.ends_with(' ') {
            out.pop();
        }
        out.push(ch);
        // If this char is '<', '(' or '&', also skip any space immediately after
        if matches!(ch, '<' | '(' | '&') && i + 1 < chars.len() && chars[i + 1] == ' ' {
            i += 1; // skip the space
        }
        i += 1;
    }

    // 3. Remove space before '+'
    out = out.replace(" +", "+");

    // 4. Remove any remaining double spaces
    while out.contains("  ") {
        out = out.replace("  ", " ");
    }

    out.trim().to_string()
}
