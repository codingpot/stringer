
use regex::Regex;

use crate::utf8;

pub fn single_utf8_str_find(target: String, sub: String) -> Option<(usize, usize)> {
    let index = match target.find(&sub) {
        Some(index) => index,
        None => return None,
    };

    let sub_target = &target.as_str()[index..index+sub.len()];
    let start = utf8::length_of((&target.as_str()[..index]).to_string());
    let end = start + utf8::length_of(sub_target.to_string());

    Some((start, end))
}

pub fn multi_utf8_str_find(target: String, sub: String) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    
    let mut prev_length = 0;
    let mut remain_str = target.clone();

    loop {
        match remain_str.find(&sub) {
            Some(index) => {
                let sub_target = &remain_str.as_str()[index..index+sub.len()];
                let start = utf8::length_of((&remain_str.as_str()[..index]).to_string());
                let end = start + utf8::length_of(sub_target.to_string());

                result.push((prev_length+start, prev_length+end));
                prev_length += end;
                remain_str = remain_str.as_str()[index+sub.len()..].to_string();
            },
            None => break,
        }
    }

    result
}

pub fn single_regex_str_find(string: String, pat: String) -> Result<(usize, usize), String> {
    let re = match Regex::new(pat.as_str()) {
        Ok(re) => re,
        Err(_) => return Err(format!("Invalid regex pattern: {}", pat)),
    };
    let result = re.find(string.as_str());
    
    match result {
        Some(index) => {
            let start = utf8::length_of((&string.as_str()[..index.start()]).to_string());
            let end = start + utf8::length_of((&string.as_str()[index.start()..index.end()]).to_string());

            Ok((start, end))
        },
        None => Err(String::from("No match")),
    }
}

pub fn multi_regex_str_find(string: String, pat: String) -> Result<Vec<(usize, usize)>, String> {
    let re = match Regex::new(pat.as_str()) {
        Ok(re) => re,
        Err(_) => return Err(format!("Invalid regex pattern: {}", pat)),
    };
    let mut result = Vec::new();
    
    let mut prev_length = 0;
    let mut remain_str = string.clone();

    loop {
        match re.find(remain_str.as_str()) {
            Some(index) => {
                let start = utf8::length_of((&remain_str.as_str()[..index.start()]).to_string());
                let end = start + utf8::length_of((&remain_str.as_str()[index.start()..index.end()]).to_string());

                result.push((prev_length+start, prev_length+end));
                prev_length += end;
                remain_str = remain_str.as_str()[index.end()..].to_string();
            },
            None => break,
        }
    }

    Ok(result)
}

#[test]
fn test() {
    let opt = multi_regex_str_find(String::from("가 나 다 라 마 바 사"), String::from("[가-사]"));
    println!("{:?}", opt);
}