use extendr_api::prelude::*;
use regex::Regex;

extendr_module! {
    mod stringer;
    fn str_regex_count;
    fn str_text_count;
}

#[extendr]
fn str_regex_count(strings: Vec<String>, pattern: String) -> Vec<i64> {
    let re = match Regex::new(pattern.as_str()) {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    strings.iter().map(|x| {
        let c = re.split(x.as_str()).count();
        if c > 0 {
            (c - 1) as i64
        } else {
            0 as i64
        }
    }).collect()
}

#[extendr]
fn str_text_count(strings: Vec<String>, sub: String) -> Vec<i64> {
    strings.iter().map(|x| {
        let c = x.split(&sub).count();
        if c > 0 {
            (c - 1) as i64
        } else {
            0 as i64
        }
    }).collect()
}