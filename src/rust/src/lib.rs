pub mod utf8;
pub mod locate;

use extendr_api::prelude::*;

extendr_module! {
    mod stringer;

    fn add_s_flag_to_dot;
    fn add_m_flag_to;
    fn add_x_flag_to;
    fn add_i_flag_to;

    fn convert_to_uppercase;

    fn str_length;

    fn str_utf8_locate;
    fn str_utf8_locates;
    fn str_regex_locate;
    fn str_regex_locates;
    fn str_bytes_locate;
    fn str_bytes_locates;
}

#[extendr]
pub fn add_s_flag_to_dot(string: String) -> String {
    string.replace(".", "(?s:.)")
}

#[extendr]
pub fn add_m_flag_to(string: String) -> String {
    string.replace("^", "(?m)^").replace("$", "(?m)$")
}

#[extendr]
pub fn add_x_flag_to(string: String) -> String {
    "(?x)".to_string() + &string
}

#[extendr]
pub fn add_i_flag_to(string: String) -> String {
    "(?i)".to_string() + &string
}

#[extendr]
pub fn convert_to_uppercase(string: String) -> String {
    string.to_uppercase()
}

#[extendr]
pub fn str_length(str: String) -> i64 {
    utf8::length_of(str) as i64
}

#[extendr]
pub fn str_utf8_locate(str: String, sub: String, folding: bool) -> Vec<i64> {
    let index = locate::single_utf8_str_locate(str, sub, folding);
    match index {
        Some((start, end)) => vec![start as i64, end as i64],
        None => vec![],
    }
}

#[extendr]
pub fn str_utf8_locates(str: String, sub: String, folding: bool) -> Vec<i64> {
    let mut result = vec![];
    let indexs = locate::multi_utf8_str_locate(str, sub, folding);
    for (_, v) in indexs.iter().enumerate() {
        result.push(v.0 as i64);
        result.push(v.1 as i64);
    }
    result
}

#[extendr]
pub fn str_regex_locate(str: String, pat: String) -> Vec<i64> {
    let index = locate::single_regex_str_locate(str, pat);
    match index {
        Ok((start, end)) => vec![start as i64, end as i64],
        Err(err) => panic!("{}", err),
    }
}

#[extendr]
pub fn str_regex_locates(str: String, pat: String) -> Vec<i64> {
    let mut result = vec![];
    let indexs = match locate::multi_regex_str_locate(str, pat) {
        Ok(indexs) => indexs,
        Err(err) => panic!("{}", err),
    };
    for (_, v) in indexs.iter().enumerate() {
        result.push(v.0 as i64);
        result.push(v.1 as i64);
    }
    result
}

#[extendr]
pub fn str_bytes_locate(str: String, sub: String) -> Vec<i64> {
    let index = locate::single_bytes_str_locate(str, sub);
    match index {
        Some((start, end)) => vec![start as i64, end as i64],
        None => vec![],
    }
}

#[extendr]
pub fn str_bytes_locates(str: String, sub: String) -> Vec<i64> {
    let mut result = vec![];
    let indexs = locate::multi_bytes_str_locate(str, sub);
    for (_, v) in indexs.iter().enumerate() {
        result.push(v.0 as i64);
        result.push(v.1 as i64);
    }
    result
}
