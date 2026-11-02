//! Tokenizer-based TSX/TS component extractor.
//!
//! The 0.12 extractor scanned raw lines with hand-rolled character counters;
//! its angle-bracket depth went negative on `=>` arrow types and silently
//! swallowed every prop declared after an inline callback. This module
//! replaces that with a real lexer (strings, comments, `=>` as one token)
//! plus a structural scanner over the token stream:
//!
//! 1. **Lex** the whole file into tokens (identifiers, punctuation, string
//!    and template literals) with line numbers. Comments and whitespace are
//!    dropped, so braces inside strings or comments can never break parsing.
//! 2. **Scan** the token stream for
//!    - props blocks: `interface XProps { ... }`, `type XProps = { ... }`
//!      (generic parameters and `extends` clauses are skipped correctly),
//!    - component declarations: `export const X = ... => ...`,
//!      `const X: React.FC<P> = ...`, `export function X(...)`,
//!      `export default function X`, `class X extends React.Component<P>`.
//! 3. **Associate** each component with its props block by naming
//!    convention: `Button` prefers the block named exactly `ButtonProps`,
//!    then any block starting with `Button` and ending with `Props`, then —
//!    when exactly one block is pending — that block. Association is
//!    deterministic and independent of declaration order beyond "declared
//!    before use".
//!
//! Known limitations (documented, stable): props blocks must appear before
//! the component that consumes them; quoted/computed member names other than
//! plain identifiers are extracted verbatim; index signatures are skipped.

use ucp_core::Result;

/// Raw TSX component found by the tokenizer-based extractor.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RawTsxExtraction {
    /// Component display name.
    pub name: String,
    /// 1-based line where the component declaration starts.
    pub line_start: usize,
    /// Props declared on the component.
    pub props: Vec<RawTsxPropExtraction>,
    /// Context type this component provides, if any.
    pub provided_context: Option<String>,
    /// Context types this component consumes.
    pub consumed_contexts: Vec<String>,
}

/// Raw prop of a TSX component before type unification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RawTsxPropExtraction {
    /// Prop name as declared in the interface/type literal.
    pub name: String,
    /// Raw TypeScript type annotation text.
    pub raw_type: String,
    /// True when the prop is not required.
    pub is_optional: bool,
    /// True for React `...rest` spread-attribute props.
    pub is_spread_attributes: bool,
}

/// A lexical token: kind, source text, and 1-based line.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    text: String,
    line: usize,
}

/// Token categories relevant to structural scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    /// Identifier or keyword.
    Ident,
    /// Punctuation, including multi-character operators like `=>`.
    Punct,
    /// String, template, or quoted literal (text includes the quotes).
    Str,
}

/// Extract components from TSX/TS source code.
///
/// Supports:
/// - `export interface FooProps { ... }` and `export type FooProps = { ... }`
/// - Inline callbacks (`=>`), string/template literals, and comments inside
///   interface bodies — none of them corrupt member boundaries
/// - Newline-separated members without semicolons (ASI)
/// - Method-signature members (`onPress(e: Event): void`), normalized to
///   arrow-callback type text
/// - Generic interfaces (`interface Props<T>`) and intersections
///   (`type X = React.ComponentProps<"button"> & { ... }`)
/// - `export const X = ...`, `export function X(`, `export default function X(`,
///   `const X: React.FC<P> =`, and `class X extends React.Component<P>` forms
///
/// # Errors
///
/// The scanner is total over well-formed token streams; this never fails
/// today, the signature is kept for forward compatibility.
pub fn extract_tsx_components(code: &str) -> Result<Vec<RawTsxExtraction>> {
    let usage = super::context::detect_react_context(code);
    let tokens = tokenize(code);

    // Pending props blocks, in declaration order.
    let mut pending: Vec<PropsBlock> = Vec::new();
    let mut components: Vec<RawTsxExtraction> = Vec::new();

    let mut i = 0usize;
    while i < tokens.len() {
        if let Some(props) = try_scan_props_block(&tokens, &mut i) {
            pending.push(props);
            continue;
        }
        if let Some((name, line)) = try_scan_component_decl(&tokens, &mut i) {
            let block = take_block_for(&mut pending, &name);
            components.push(RawTsxExtraction {
                name,
                line_start: line,
                props: block.map(|b| b.props).unwrap_or_default(),
                provided_context: usage.provided.clone(),
                consumed_contexts: usage.consumed.clone(),
            });
            continue;
        }
        i += 1;
    }

    Ok(components)
}

/// A parsed props block awaiting a component to bind to.
struct PropsBlock {
    /// Interface/type-alias name, e.g. `ButtonProps`.
    type_name: String,
    props: Vec<RawTsxPropExtraction>,
}

/// Pick the props block that belongs to component `name`.
///
/// Priority: exact `<Name>Props`, then `<Name>…Props` prefix match, then the
/// only pending block when exactly one is waiting. Consumed blocks are
/// removed; everything else stays queued for later components.
fn take_block_for(pending: &mut Vec<PropsBlock>, component: &str) -> Option<PropsBlock> {
    let exact = format!("{component}Props");
    if let Some(pos) = pending.iter().position(|b| b.type_name == exact) {
        return Some(pending.remove(pos));
    }
    let lower = component.to_lowercase();
    if let Some(pos) = pending.iter().position(|b| {
        b.type_name.to_lowercase().starts_with(&lower) && b.type_name.ends_with("Props")
    }) {
        return Some(pending.remove(pos));
    }
    if pending.len() == 1 {
        return Some(pending.remove(0));
    }
    None
}

/// Try to scan a props block starting at `tokens[i]`.
///
/// On success returns the block and advances `i` past the block's closing
/// brace. On failure leaves `i` unchanged and returns `None`.
fn try_scan_props_block(tokens: &[Token], i: &mut usize) -> Option<PropsBlock> {
    let start = *i;
    // `interface XProps ... {` or `type XProps ... = ... {`
    if tokens.get(*i)?.kind != TokenKind::Ident {
        return None;
    }
    let keyword = tokens[*i].text.as_str();
    if keyword != "interface" && keyword != "type" {
        return None;
    }
    let name_tok = tokens.get(*i + 1)?;
    if name_tok.kind != TokenKind::Ident {
        return None;
    }
    let type_name = name_tok.text.clone();
    if !type_name.contains("Props") {
        return None;
    }
    *i += 2;

    // Skip generic parameters and `extends ...` clauses up to the body brace
    // (for `type` aliases, also skip everything up to the `=`).
    if keyword == "type" {
        while let Some(tok) = tokens.get(*i) {
            if tok.kind == TokenKind::Punct && tok.text == "=" {
                *i += 1;
                break;
            }
            *i += 1;
        }
    } else {
        skip_to_body_brace(tokens, i);
    }

    // Find the first top-level `{ ... }` group after the declaration head.
    let Some(body) = capture_balanced_braces(tokens, i) else {
        // Not a braced props block (e.g. `type X = string`); restore position.
        *i = start;
        return None;
    };
    let props = parse_members(&body);
    if props.is_empty() {
        // An empty or unparseable block is still a valid association target,
        // but contributes nothing; keep it so it is not double-consumed by a
        // later component.
    }
    Some(PropsBlock { type_name, props })
}

/// Skip tokens until the opening `{` of an interface body, crossing generic
/// parameter lists and `extends` clauses with balanced angle/paren tracking.
fn skip_to_body_brace(tokens: &[Token], i: &mut usize) {
    let mut angle = 0i32;
    let mut paren = 0i32;
    while let Some(tok) = tokens.get(*i) {
        if tok.kind == TokenKind::Punct {
            match tok.text.as_str() {
                "<" => angle += 1,
                ">" => angle -= 1,
                "(" => paren += 1,
                ")" => paren -= 1,
                "{" if angle <= 0 && paren <= 0 => return,
                _ => {}
            }
        }
        *i += 1;
    }
}

/// Consume the balanced `{ ... }` group starting at `tokens[i]` (which must
/// be a `{`), returning the inner tokens (without the braces) and advancing
/// `i` past the closing `}`. Returns `None` without advancing when the brace
/// is unterminated.
fn capture_balanced_braces(tokens: &[Token], i: &mut usize) -> Option<Vec<Token>> {
    if tokens.get(*i).map(|t| (t.kind, t.text.as_str())) != Some((TokenKind::Punct, "{")) {
        return None;
    }
    let mut depth = 0usize;
    let start = *i;
    let mut inner = Vec::new();
    while let Some(tok) = tokens.get(*i) {
        if tok.kind == TokenKind::Punct {
            match tok.text.as_str() {
                "{" => depth += 1,
                "}" => {
                    depth -= 1;
                    if depth == 0 {
                        *i += 1;
                        return Some(std::mem::take(&mut inner));
                    }
                }
                _ => {}
            }
        }
        if *i > start {
            inner.push(tok.clone());
        }
        *i += 1;
    }
    *i = start;
    None
}

/// Try to scan a component declaration starting at `tokens[i]`.
///
/// On success returns `(name, line)` and advances `i` past the declaration
/// head; the caller continues scanning from there (body tokens are of no
/// interest except that they may contain the *next* declaration).
fn try_scan_component_decl(tokens: &[Token], i: &mut usize) -> Option<(String, usize)> {
    let start = *i;
    let mut j = *i;

    // Optional `export` / `export default` prefixes.
    if tokens.get(j).map(|t| t.text.as_str()) == Some("export") {
        j += 1;
        if tokens.get(j).map(|t| t.text.as_str()) == Some("default") {
            j += 1;
            match tokens.get(j) {
                Some(tok) if tok.kind == TokenKind::Ident && tok.text == "function" => {
                    if let Some(name) = ident_at(tokens, j + 1) {
                        let line = tokens[start].line;
                        *i = j + 2;
                        return Some((name, line));
                    }
                }
                Some(tok) if tok.kind == TokenKind::Ident && tok.text == "class" => {
                    if let Some(name) = ident_at(tokens, j + 1) {
                        let line = tokens[start].line;
                        *i = j + 2;
                        return Some((name, line));
                    }
                }
                _ => {
                    // Anonymous default export: no extractable name.
                    *i = j;
                    return None;
                }
            }
            *i = j;
            return None;
        }
    }

    match tokens.get(j).map(|t| (t.kind, t.text.as_str())) {
        Some((TokenKind::Ident, "const")) => {
            let name = ident_at(tokens, j + 1)?;
            let line = tokens[start].line;
            if const_is_component(tokens, j + 2) {
                *i = j + 2;
                return Some((name, line));
            }
            None
        }
        Some((TokenKind::Ident, "function")) => {
            let name = ident_at(tokens, j + 1)?;
            let line = tokens[start].line;
            *i = j + 2;
            Some((name, line))
        }
        Some((TokenKind::Ident, "class")) => {
            let name = ident_at(tokens, j + 1)?;
            if class_is_react_component(tokens, j + 2) {
                let line = tokens[start].line;
                *i = j + 2;
                return Some((name, line));
            }
            None
        }
        _ => None,
    }
}

/// The identifier text at `tokens[i]` when the token is an identifier.
fn ident_at(tokens: &[Token], i: usize) -> Option<String> {
    match tokens.get(i) {
        Some(tok) if tok.kind == TokenKind::Ident && !is_reserved(&tok.text) => {
            Some(tok.text.clone())
        }
        _ => None,
    }
}

/// Keywords that can never be component names.
fn is_reserved(word: &str) -> bool {
    matches!(
        word,
        "function"
            | "class"
            | "const"
            | "let"
            | "var"
            | "return"
            | "if"
            | "else"
            | "for"
            | "while"
            | "new"
            | "typeof"
            | "interface"
            | "type"
            | "extends"
            | "export"
            | "default"
            | "import"
            | "from"
            | "as"
            | "await"
            | "async"
    )
}

/// Decide whether a `const` declaration is a component declaration.
///
/// A const is treated as a component when either
/// - its type annotation matches `React.FC<...>` / `FC<...>` /
///   `(React.)PureComponent<...>`, or
/// - its initializer contains an arrow function (`=>` token) before the
///   statement-terminating `;` at depth zero.
fn const_is_component(tokens: &[Token], mut j: usize) -> bool {
    // Type annotation form: `: ... =` — look for FC patterns.
    if tokens.get(j).map(|t| t.text.as_str()) == Some(":") {
        let mut k = j + 1;
        let mut angle = 0i32;
        while let Some(tok) = tokens.get(k) {
            match tok.kind {
                TokenKind::Punct if tok.text == "=" && angle <= 0 => break,
                TokenKind::Punct if tok.text == "<" => angle += 1,
                TokenKind::Punct if tok.text == ">" => angle -= 1,
                _ => {}
            }
            k += 1;
        }
        let type_toks = &tokens[j + 1..k.min(tokens.len())];
        if fc_pattern_in(type_toks) {
            return true;
        }
        j = k;
    }
    // Initializer scan: an arrow token before the depth-zero `;`.
    let mut depth = 0i32;
    while let Some(tok) = tokens.get(j) {
        if tok.kind == TokenKind::Punct {
            match tok.text.as_str() {
                "(" | "[" | "{" => depth += 1,
                ")" | "]" | "}" => depth -= 1,
                ";" if depth <= 0 => return false,
                "=>" => return true,
                _ => {}
            }
        }
        j += 1;
    }
    false
}

/// `React.FC<` / `FC<` / `React.PureComponent<` / `PureComponent<` shapes.
fn fc_pattern_in(type_toks: &[Token]) -> bool {
    let texts: Vec<&str> = type_toks.iter().map(|t| t.text.as_str()).collect();
    for w in texts.windows(3) {
        if w[1] == "." {
            let base = w[2];
            if matches!(base, "FC" | "PureComponent") {
                return true;
            }
        }
    }
    texts
        .windows(2)
        .any(|w| matches!(w[0], "FC" | "PureComponent") && w[1] == "<")
}

/// Decide whether a `class X` declaration extends a React component.
fn class_is_react_component(tokens: &[Token], mut j: usize) -> bool {
    if tokens.get(j).map(|t| t.text.as_str()) != Some("extends") {
        return false;
    }
    j += 1;
    // `React.Component<` / `React.PureComponent<` / `Component<` / `PureComponent<`
    if tokens.get(j).map(|t| t.text.as_str()) == Some("React")
        && tokens.get(j + 1).map(|t| t.text.as_str()) == Some(".")
    {
        j += 2;
    }
    matches!(
        tokens.get(j).map(|t| t.text.as_str()),
        Some("Component" | "PureComponent")
    )
}

/// Parse interface/type-literal members out of a braced body.
fn parse_members(body: &[Token]) -> Vec<RawTsxPropExtraction> {
    let mut props = Vec::new();
    let mut i = 0usize;
    while i < body.len() {
        // Skip separators between members.
        if body[i].kind == TokenKind::Punct && matches!(body[i].text.as_str(), ";" | ",") {
            i += 1;
            continue;
        }
        let Some(end) = member_end(body, i) else {
            break;
        };
        if let Some(prop) = parse_single_member(&body[i..end]) {
            props.push(prop);
        }
        i = end;
    }
    props
}

/// Find the exclusive end index of the member starting at `start`.
///
/// A member ends at a depth-zero `;` or `,`, or (ASI) immediately before a
/// depth-zero token that starts a new member (`name:`, `name?:`, or a method
/// signature `name(`).
fn member_end(body: &[Token], start: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut i = start;
    while i < body.len() {
        let tok = &body[i];
        if tok.kind == TokenKind::Punct {
            match tok.text.as_str() {
                "(" | "[" | "{" | "<" => depth += 1,
                ")" | "]" | "}" | ">" => depth -= 1,
                // Exclusive end *before* the separator; the caller skips it.
                ";" | "," if depth <= 0 => return Some(i),
                _ => {}
            }
            i += 1;
            continue;
        }
        if depth <= 0 && i > start && starts_new_member(body, i) {
            return Some(i);
        }
        if depth <= 0
            && i > start
            && tok.kind == TokenKind::Ident
            && tok.text == "readonly"
            && member_name_at(body, i + 1).is_some()
        {
            return Some(i);
        }
        i += 1;
    }
    // A member that runs to the end of the body.
    if i > start {
        Some(i)
    } else {
        None
    }
}

/// True when the token at `i` begins a new interface member.
fn starts_new_member(body: &[Token], i: usize) -> bool {
    member_name_at(body, i).is_some()
}

/// If a member name starts at `i`, return the index just past the name
/// (including an optional `?`), else `None`.
fn member_name_at(body: &[Token], i: usize) -> Option<usize> {
    let tok = body.get(i)?;
    match tok.kind {
        TokenKind::Ident if !is_reserved(&tok.text) => {}
        TokenKind::Str => {}
        _ => return None,
    }
    // Quoted names must be followed by `:` or `?:`; identifiers may also be
    // method signatures (`name(`).
    let next = body.get(i + 1)?.kind;
    match next {
        _ if body[i + 1].kind == TokenKind::Punct && body[i + 1].text == ":" => Some(i + 2),
        _ if body[i + 1].kind == TokenKind::Punct && body[i + 1].text == "?" => {
            let after = body.get(i + 2)?;
            if after.kind == TokenKind::Punct && after.text == ":" {
                Some(i + 3)
            } else {
                None
            }
        }
        _ if body[i + 1].kind == TokenKind::Punct && body[i + 1].text == "(" => Some(i + 2),
        _ => None,
    }
}

/// Parse one member's tokens into a raw prop.
///
/// Handles plain members (`name?: type`), method signatures
/// (`name(args): ret` — normalized to `(args) => ret`), the `readonly`
/// modifier, and index signatures (skipped: they carry no single prop
/// identity).
fn parse_single_member(tokens: &[Token]) -> Option<RawTsxPropExtraction> {
    let mut i = 0usize;
    if tokens.first().map(|t| t.text.as_str()) == Some("readonly") {
        i = 1;
    }
    let name_tok = tokens.get(i)?;
    let name = match name_tok.kind {
        TokenKind::Ident => name_tok.text.clone(),
        TokenKind::Str => name_tok
            .text
            .trim_matches(|c| c == '"' || c == '\'')
            .to_string(),
        TokenKind::Punct => return None,
    };
    if is_reserved(&name) {
        return None;
    }
    i += 1;

    let mut is_optional = false;
    if tokens.get(i).map(|t| (t.kind, t.text.as_str())) == Some((TokenKind::Punct, "?")) {
        is_optional = true;
        i += 1;
    }

    // Index signature: `[key: string]: T` — no single prop identity.
    if tokens.get(i).map(|t| (t.kind, t.text.as_str())) == Some((TokenKind::Punct, "[")) {
        return None;
    }

    // Method signature: `name(args): ret` → `(args) => ret`.
    if tokens.get(i).map(|t| (t.kind, t.text.as_str())) == Some((TokenKind::Punct, "(")) {
        let mut depth = 0i32;
        let args_start = i;
        while let Some(tok) = tokens.get(i) {
            if tok.kind == TokenKind::Punct {
                match tok.text.as_str() {
                    "(" => depth += 1,
                    ")" => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        let args: Vec<&str> = tokens[args_start..=i.min(tokens.len().saturating_sub(1))]
            .iter()
            .map(|t| t.text.as_str())
            .collect();
        i += 1; // past `)`
                // Expect `: ret`.
        let ret =
            if tokens.get(i).map(|t| (t.kind, t.text.as_str())) == Some((TokenKind::Punct, ":")) {
                join_type_text(&tokens[i + 1..])
            } else {
                String::new()
            };
        return Some(RawTsxPropExtraction {
            name,
            raw_type: format!("({}) => {}", args.join(""), ret),
            is_optional,
            is_spread_attributes: false,
        });
    }

    if tokens.get(i).map(|t| (t.kind, t.text.as_str())) != Some((TokenKind::Punct, ":")) {
        return None;
    }
    let raw_type = join_type_text(&tokens[i + 1..]);
    if raw_type.is_empty() {
        return None;
    }
    Some(RawTsxPropExtraction {
        name,
        raw_type,
        is_optional,
        is_spread_attributes: false,
    })
}

/// Concatenate type tokens back into source-like text.
///
/// String and template literals keep their interior spacing verbatim; all
/// other whitespace was already dropped by the lexer, matching the
/// space-stripped concrete-type convention used across UCP.
fn join_type_text(tokens: &[Token]) -> String {
    tokens.iter().map(|t| t.text.as_str()).collect()
}

/// Lex `code` into tokens, dropping whitespace and comments.
///
/// Handles `//` and `/* */` comments, single- and double-quoted strings with
/// escape sequences, and template literals (escapes honored; `${...}`
/// interpolation is kept as raw text inside the token, which is sufficient
/// for type annotations).
fn tokenize(code: &str) -> Vec<Token> {
    let chars: Vec<char> = code.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut line = 1usize;

    while i < chars.len() {
        let c = chars[i];
        match c {
            '\n' => {
                line += 1;
                i += 1;
            }
            ' ' | '\t' | '\r' => i += 1,
            '/' if chars.get(i + 1) == Some(&'/') => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            '/' if chars.get(i + 1) == Some(&'*') => {
                i += 2;
                while i < chars.len() {
                    if chars[i] == '\n' {
                        line += 1;
                    }
                    if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            }
            '"' | '\'' | '`' => {
                let quote = c;
                let start_line = line;
                let mut text = String::new();
                text.push(quote);
                i += 1;
                while i < chars.len() {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        text.push(chars[i]);
                        text.push(chars[i + 1]);
                        i += 2;
                        continue;
                    }
                    if chars[i] == '\n' {
                        line += 1;
                    }
                    if chars[i] == quote {
                        i += 1;
                        break;
                    }
                    text.push(chars[i]);
                    i += 1;
                }
                text.push(quote);
                tokens.push(Token {
                    kind: TokenKind::Str,
                    text,
                    line: start_line,
                });
            }
            c if c.is_alphabetic() || c == '_' || c == '$' => {
                let start = i;
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '$')
                {
                    i += 1;
                }
                tokens.push(Token {
                    kind: TokenKind::Ident,
                    text: chars[start..i].iter().collect(),
                    line,
                });
            }
            '=' if chars.get(i + 1) == Some(&'>') => {
                tokens.push(Token {
                    kind: TokenKind::Punct,
                    text: "=>".to_string(),
                    line,
                });
                i += 2;
            }
            _ => {
                tokens.push(Token {
                    kind: TokenKind::Punct,
                    text: c.to_string(),
                    line,
                });
                i += 1;
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_keeps_arrow_as_single_token() {
        let tokens = tokenize("(e: T) => void");
        assert!(tokens.iter().any(|t| t.text == "=>"));
    }

    #[test]
    fn lexer_drops_comments_and_keeps_string_interior() {
        let tokens = tokenize("// gone\n\"keep; me {ok}\" /* also gone */ x");
        let strs: Vec<_> = tokens.iter().filter(|t| t.kind == TokenKind::Str).collect();
        assert_eq!(strs.len(), 1);
        assert_eq!(strs[0].text, "\"keep; me {ok}\"");
        assert!(tokens.iter().any(|t| t.text == "x"));
    }

    #[test]
    fn lexer_tracks_lines() {
        let tokens = tokenize("a\nb\nc");
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[2].line, 3);
    }

    #[test]
    fn unterminated_brace_does_not_loop() {
        let components = extract_tsx_components("export interface XProps { a: string").unwrap();
        assert!(components.is_empty());
    }

    #[test]
    fn unterminated_string_does_not_loop() {
        let components =
            extract_tsx_components("export interface XProps { a: \"unterminated").unwrap();
        assert!(components.is_empty());
    }
}
