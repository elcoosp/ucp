use ucp_core::{Result, UcpError};

const ALLOWED_LICENSES: &[&str] = &[
    "MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause",
    "ISC", "0BSD", "Unicode-DFS-2016"
];

pub fn check_spdx_compliance(license_str: &str) -> Result<()> {
    let expr = spdx::Expression::parse(license_str)
        .map_err(|e| UcpError::License(format!("Invalid SPDX: {}", e)))?;

    let mut iter = expr.iter();
    while let Some(req) = iter.next() {
        let req_str = req.req.to_string();
        if !ALLOWED_LICENSES.contains(&req_str.as_str()) {
            return Err(UcpError::License(format!(
                "Rejected non-permissive license: {}. Allowed: {:?}",
                req_str, ALLOWED_LICENSES
            )));
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
