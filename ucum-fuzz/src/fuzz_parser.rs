#![no_main]

use libfuzzer_sys::fuzz_target;
use octofhir_ucum_core::parse_expression;

fuzz_target!(|data: &[u8]| {
    // Convert the byte array to a string if possible
    if let Ok(s) = std::str::from_utf8(data) {
        // Only fuzz if the string is not too long to avoid excessive resource usage
        if s.len() < 100 {
            // Try to parse the expression
            let _ = parse_expression(s);
        }
    }
});
