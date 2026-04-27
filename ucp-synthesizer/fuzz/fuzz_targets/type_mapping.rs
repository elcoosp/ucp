#![no_main]
use libfuzzer_sys::fuzz_target;
use ucp_synthesizer::unify::map_raw_type_with_concrete;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = map_raw_type_with_concrete(s);
    }
});
