use regex::Regex;
use ucp_core::{Result, UcpError};

const ALLOWED_LICENSES: &[&str] = &[
    "MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause",
    "ISC", "0BSD", "Unicode-DFS-2016"
];

pub fn check_spdx_compliance(license_str: &str) -> Result<()> {
    // Validate that the expression is valid SPDX syntax
    let _expr = spdx::Expression::parse(license_str)
        .map_err(|e| UcpError::License(format!("Invalid SPDX: {}", e)))?;

    // Extract license identifiers from the raw string using regex.
    // SPDX IDs are alphanumeric plus dots, pluses, and dashes.
    // We skip operator keywords (AND, OR, WITH).
    let re = Regex::new(r"[A-Za-z0-9.+-]+").unwrap();
    for cap in re.captures_iter(license_str) {
        let id = cap.get(0).unwrap().as_str();
        match id {
            "AND" | "OR" | "WITH" => continue,
            _ if id.starts_with(char::is_uppercase) || id.starts_with(|c: char| c.is_ascii_digit()) => {
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

pub fn is_path_safe_to_parse(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let path_str = path.to_string_lossy().replace("\\", "/");

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if ["env", "pem", "key"].contains(&ext.to_lowercase().as_str()) { return false; }
    }
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        if ["credentials", "secret"].contains(&stem.to_lowercase().as_str()) { return false; }
    }

    path_str.starts_with("src/") || path_str.starts_with("components/")
}
