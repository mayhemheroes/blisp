#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(code) = std::str::from_utf8(data) {
        // Parse and type-check the input.
        if let Ok(exprs) = blisp::init(code, vec![]) {
            if let Ok(ctx) = blisp::typing(exprs) {
                // Evaluate the same source against the typed context.
                let _ = blisp::eval(code, &ctx);
            }
        }
    }
});
