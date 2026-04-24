use ucp_synthesizer::security::{check_spdx_compliance, is_path_safe_to_parse};

#[test]
fn reject_gpl_license() {
    assert!(check_spdx_compliance("GPL-3.0-only").is_err());
}

#[test]
fn accept_mit_license() {
    assert!(check_spdx_compliance("MIT").is_ok());
}

#[test]
fn accept_apache_license() {
    assert!(check_spdx_compliance("Apache-2.0").is_ok());
}

#[test]
fn accept_bsd_licenses() {
    assert!(check_spdx_compliance("BSD-2-Clause").is_ok());
    assert!(check_spdx_compliance("BSD-3-Clause").is_ok());
    assert!(check_spdx_compliance("ISC").is_ok());
}

#[test]
fn accept_complex_spdx_expression() {
    assert!(check_spdx_compliance("MIT OR Apache-2.0").is_ok());
}

#[test]
fn reject_agpl() {
    assert!(check_spdx_compliance("AGPL-3.0-only").is_err());
}

#[test]
fn reject_mpl() {
    assert!(check_spdx_compliance("MPL-2.0").is_err());
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
fn reject_key_file() {
    assert!(!is_path_safe_to_parse("secret.key"));
}

#[test]
fn allow_src_directory() {
    assert!(is_path_safe_to_parse("src/components/button.rs"));
}

#[test]
fn allow_components_directory() {
    assert!(is_path_safe_to_parse("components/Button.tsx"));
}

#[test]
fn reject_tests_directory() {
    assert!(!is_path_safe_to_parse("tests/mock_extraction.rs"));
}

#[test]
fn reject_target_directory() {
    assert!(!is_path_safe_to_parse("target/debug/build.rs"));
}

#[test]
fn reject_node_modules() {
    assert!(!is_path_safe_to_parse("node_modules/pkg/index.js"));
}

#[test]
fn reject_dist_directory() {
    assert!(!is_path_safe_to_parse("dist/bundle.js"));
}

#[test]
fn reject_hidden_file_in_src() {
    assert!(!is_path_safe_to_parse("src/.env.local"));
}

#[test]
fn allow_nested_src() {
    assert!(is_path_safe_to_parse("packages/ui/src/Button.tsx"));
}

#[test]
fn allow_absolute_path_with_src() {
    assert!(is_path_safe_to_parse("/home/user/project/src/lib.rs"));
}

#[test]
fn reject_absolute_path_without_src() {
    assert!(!is_path_safe_to_parse("/home/user/project/lib.rs"));
}

#[test]
fn reject_dotenv_at_root() {
    assert!(!is_path_safe_to_parse(".env"));
}

#[test]
fn reject_secret_file() {
    assert!(!is_path_safe_to_parse("secret.key"));
}

#[test]
fn allow_rs_file_in_src() {
    assert!(is_path_safe_to_parse("src/main.rs"));
}

#[test]
fn allow_tsx_file_in_components() {
    assert!(is_path_safe_to_parse("components/Dialog.tsx"));
}

#[test]
fn allow_ts_file_in_src() {
    assert!(is_path_safe_to_parse("src/utils.ts"));
}

#[test]
fn reject_build_directory() {
    assert!(!is_path_safe_to_parse("build/output.js"));
}
