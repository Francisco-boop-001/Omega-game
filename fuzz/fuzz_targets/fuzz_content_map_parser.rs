#![no_main]

use libfuzzer_sys::fuzz_target;
use omega_content::parse_legacy_map_from_str;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let _ = parse_legacy_map_from_str(input, "fuzz.map");
    }
});
