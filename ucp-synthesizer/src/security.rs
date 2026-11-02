use regex::Regex;
use std::path::Path;
use ucp_core::{Result, UcpError};

const ALLOWED_LICENSES: &[&str] = &[
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "0BSD",
    "Unicode-DFS-2016",
];

#[cfg(feature = "license-check")]
/// Compile the identifier regex once; the pattern is a compile-time
/// constant and cannot fail.
fn ident_regex() -> &'static Regex {
    static IDENT_RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    IDENT_RE.get_or_init(|| Regex::new("[A-Za-z0-9.+-]+").expect("valid static regex"))
}

/// Validate that a license string is an accepted SPDX identifier.
///
/// # Errors
///
/// Errors when the string is not a valid SPDX expression or is not on
/// the allow-list (MIT, Apache-2.0, BSD variants, ISC).
///
/// # Panics
///
/// Panics only if the static identifier regex fails to compile, which
/// cannot happen for the fixed pattern.
pub fn check_spdx_compliance(license_str: &str) -> Result<()> {
    let _expr = spdx::Expression::parse(license_str)
        .map_err(|e| UcpError::License(format!("Invalid SPDX: {e}")))?;

    for cap in ident_regex().captures_iter(license_str) {
        let Some(id) = cap.get(0).map(|m| m.as_str()) else {
            continue;
        };
        // SPDX composition keywords are not license ids.
        if matches!(id, "AND" | "OR" | "WITH") {
            continue;
        }
        let is_license_id =
            id.starts_with(char::is_uppercase) || id.starts_with(|c: char| c.is_ascii_digit());
        if is_license_id && !ALLOWED_LICENSES.contains(&id) {
            return Err(UcpError::License(format!(
                "Rejected non-permissive license: {id}. Allowed: {ALLOWED_LICENSES:?}"
            )));
        }
    }
    Ok(())
}

/// Check if a file path is safe to parse (not credentials, secrets, etc.).
///
/// Uses `std::path::Path` component iteration instead of string splitting,
/// so it works correctly on both Unix and Windows paths regardless of separator.
#[must_use]
pub fn is_path_safe_to_parse(path: &str) -> bool {
    let path = Path::new(path);

    // Reject dangerous extensions
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if ["env", "pem", "key"].contains(&ext.to_lowercase().as_str()) {
            return false;
        }
    }

    // Reject dangerous file stems
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        if ["credentials", "secret"].contains(&stem.to_lowercase().as_str()) {
            return false;
        }
    }

    // Reject hidden files (filenames starting with '.')
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if file_name.starts_with('.') {
            return false;
        }
    }

    // Require "src" or "components" as a path component anywhere in the path.
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|s| s == "src" || s == "components")
    })
}
