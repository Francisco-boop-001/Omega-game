#![no_main]

use libfuzzer_sys::fuzz_target;
use omega_save::{decode_json, decode_state_json};

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let _ = decode_json(input);
        let _ = decode_state_json(input);
    }
});
