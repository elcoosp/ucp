#![no_main]
use libfuzzer_sys::fuzz_target;
use ucp_synthesizer::import::design_md::parse_design_md;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_design_md(s);
    }
});
