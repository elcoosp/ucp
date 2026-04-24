use ucp_synthesizer::security::{check_spdx_compliance, is_path_safe_to_parse};

#[test]
fn reject_gpl_license() {
    let result = check_spdx_compliance("GPL-3.0-only");
    assert!(result.is_err());
}

#[test]
fn accept_mit_license() {
    let result = check_spdx_compliance("MIT");
    assert!(result.is_ok());
}

#[test]
fn reject_credentials_file() {
    assert!(!is_path_safe_to_parse(".env"));
}

#[test]
fn reject_pem_file() {
    assert!(!is_path_safe_to_parse("credentials.pem"));
}

#[test]
fn allow_src_directory() {
    assert!(is_path_safe_to_parse("src/components/button.rs"));
}

#[test]
fn reject_tests_directory() {
    assert!(!is_path_safe_to_parse("tests/mock_extraction.rs"));
}
