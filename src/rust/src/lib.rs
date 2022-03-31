pub mod utf8;
pub mod find;

use extendr_api::prelude::*;

extendr_module! {
    mod stringer;

    fn str_length;
}

#[extendr]
fn str_length(str: String) -> i64 {
    utf8::length_of(str) as i64
}
