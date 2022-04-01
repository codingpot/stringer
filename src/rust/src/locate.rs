
use regex::Regex;

use crate::utf8;

pub fn single_utf8_str_locate(
    target: String,
    sub: String,
    folding: bool,
) -> Option<(usize, usize)> {
    if !folding {
        let index = match target.find(&sub) {
            Some(index) => index,
            None => return None,
        };

        let start = index;
        let end = start + sub.len();

        Some((start, end))
    } else {
        let re = match Regex::new(&utf8::add_i_flag_to(utf8::disable_regex(sub.clone()))) {
            Ok(re) => re,
            Err(_) => return None,
        };

        re.find(&target).map(|m| (m.start(), m.end()))
    }
}

pub fn multi_utf8_str_locate(target: String, sub: String, folding: bool) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    if !folding {
        let mut prev_length = 0;
        let mut remain_str = target.clone();

        loop {
            match remain_str.find(&sub) {
                Some(index) => {
                    let sub_target = &remain_str.as_str()[index..index + sub.len()];
                    let start = utf8::length_of((&remain_str.as_str()[..index]).to_string());
                    let end = start + utf8::length_of(sub_target.to_string());

                    result.push((prev_length + start, prev_length + end));
                    prev_length += end;
                    remain_str = remain_str.as_str()[index + sub.len()..].to_string();
                }
                None => break,
            }
        }
    } else {
        let re = match Regex::new(&utf8::add_i_flag_to(utf8::disable_regex(sub.clone()))) {
            Ok(re) => re,
            Err(_) => return result,
        };

        re.find_iter(&target).for_each(|m| {
            result.push((m.start(), m.end()));
        });
    }

    result
}

pub fn single_regex_str_locate(string: String, pat: String) -> Result<(usize, usize), String> {
    let re = match Regex::new(pat.as_str()) {
        Ok(re) => re,
        Err(_) => return Err(format!("Invalid regex pattern: {}", pat)),
    };
    let result = re.find(string.as_str());

    match result {
        Some(index) => {
            Ok((index.start(), index.end()))
        }
        None => Err(String::from("No match")),
    }
}

pub fn multi_regex_str_locate(string: String, pat: String) -> Result<Vec<(usize, usize)>, String> {
    let re = match Regex::new(pat.as_str()) {
        Ok(re) => re,
        Err(_) => return Err(format!("Invalid regex pattern: {}", pat)),
    };
    let mut result = Vec::new();

    re.find_iter(string.as_str()).for_each(|m| {
        result.push((m.start(), m.end()));
    });

    Ok(result)
}

pub fn single_bytes_str_locate(string: String, sub: String) -> Option<(usize, usize)> {
    let index = match string.find(sub.as_str()) {
        Some(index) => index,
        None => return None,
    };

    let start = index;
    let end = index + sub.len();

    Some((start, end))
}

pub fn multi_bytes_str_locate(string: String, sub: String) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let mut prev_length = 0;
    let mut remain_str = string.clone();

    loop {
        match remain_str.find(sub.as_str()) {
            Some(index) => {
                let start = prev_length + index;
                let end = start + sub.len();

                result.push((start, end));
                prev_length += end;
                remain_str = remain_str.as_str()[index + sub.len()..].to_string();
            }
            None => break,
        }
    }

    result
}
