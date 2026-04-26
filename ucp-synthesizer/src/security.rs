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
pub fn check_spdx_compliance(license_str: &str) -> Result<()> {
    let _expr = spdx::Expression::parse(license_str)
        .map_err(|e| UcpError::License(format!("Invalid SPDX: {}", e)))?;

    let re = Regex::new(r"[A-Za-z0-9.+-]+").unwrap();
    for cap in re.captures_iter(license_str) {
        let id = cap.get(0).unwrap().as_str();
        match id {
            "AND" | "OR" | "WITH" => continue,
            _ if id.starts_with(char::is_uppercase)
                || id.starts_with(|c: char| c.is_ascii_digit()) =>
            {
                if !ALLOWED_LICENSES.contains(&id) {
                    return Err(UcpError::License(format!(
                        "Rejected non-permissive license: {}. Allowed: {:?}",
                        id, ALLOWED_LICENSES
                    )));
                }
            }
            _ => continue,
        }
    }
    Ok(())
}

/// Check if a file path is safe to parse (not credentials, secrets, etc.).
///
/// Uses `std::path::Path` component iteration instead of string splitting,
/// so it works correctly on both Unix and Windows paths regardless of separator.
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
