#![no_main]
use libfuzzer_sys::fuzz_target;
use ucp_synthesizer::extract::tsx_ast::extract_tsx_components;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = extract_tsx_components(s);
    }
});
