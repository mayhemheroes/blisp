#![no_main]

use libfuzzer_sys::fuzz_target;
use blisp;

fuzz_target!(|data: (&str, &str)| {
    let (code, e) = data;
    match blisp::init(code, vec![]) {
        Ok(exprs) => {
            match blisp::typing(exprs) {
                Ok(ctx) => {
                    blisp::eval(e, &ctx);
                },
                _ => ()
            }
        },
        _ => ()
    }

});
