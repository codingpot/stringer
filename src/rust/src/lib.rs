use extendr_api::prelude::*;
use regex::Regex;

extendr_module! {
    mod stringer;
    fn str_regex_count;
    fn str_text_count;
    fn str_regex_detect;
    fn str_text_detect;
    fn str_regex_extract;
    fn str_text_extract;
    fn single_str_regex_extract_all;
    fn single_str_text_extract_all;

    fn str_length;
}

#[extendr]
fn str_regex_count(strings: Vec<String>, pattern: String) -> Vec<i64> {
    let re = match Regex::new(pattern.as_str()) {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    strings
        .iter()
        .map(|x| {
            let c = re.split(x.as_str()).count();
            if c > 0 {
                (c - 1) as i64
            } else {
                0 as i64
            }
        })
        .collect()
}

#[extendr]
fn str_text_count(strings: Vec<String>, sub: String) -> Vec<i64> {
    strings
        .iter()
        .map(|x| {
            let c = x.split(&sub).count();
            if c > 0 {
                (c - 1) as i64
            } else {
                0 as i64
            }
        })
        .collect()
}

#[extendr]
fn str_regex_detect(strings: Vec<String>, pattern: String) -> Vec<bool> {
    let re = match Regex::new(pattern.as_str()) {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    strings.iter().map(|x| re.is_match(x.as_str())).collect()
}

#[extendr]
fn str_text_detect(strings: Vec<String>, sub: String) -> Vec<bool> {
    strings.iter().map(|x| x.contains(sub.as_str())).collect()
}

#[extendr]
fn str_regex_extract(strings: Vec<String>, pattern: String) -> Vec<String> {
    let re = match Regex::new(pattern.as_str()) {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    strings
        .iter()
        .map(|x| {
            re.find(x.as_str())
                .map(|m| {
                    x.chars()
                        .skip(m.start())
                        .take(m.end() - m.start())
                        .collect()
                })
                .unwrap_or_else(|| "".to_string())
        })
        .collect()
}

#[extendr]
fn str_text_extract(strings: Vec<String>, sub: String) -> Vec<String> {
    strings
        .iter()
        .map(|x| {
            if x.contains(sub.as_str()) {
                sub.clone()
            } else {
                "".to_string()
            }
        })
        .collect()
}

#[extendr]
fn single_str_regex_extract_all(string: String, pattern: String) -> Vec<String> {
    let re = match Regex::new(pattern.as_str()) {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    let mut rs = Vec::new();
    let mut index = 0;
    loop {
        match re.find_at(string.as_str(), index) {
            Some(m) => {
                rs.push(
                    string.chars()
                        .skip(m.start())
                        .take(m.end() - m.start())
                        .collect(),
                );
                index = m.end();
            }
            None => break,
        }
    }
    if rs.len() == 0 {
        rs.push("".to_string());
    }
    rs
}

#[extendr]
fn single_str_text_extract_all(string: String, sub: String) -> Vec<String> {
    let mut rs = Vec::new();
    let mut index = 0;
    loop {
        let (_, target) = string.split_at(index);
        match target.find(sub.as_str()) {
            Some(m) => {
                rs.push(
                    string.chars()
                        .skip(m)
                        .take(sub.len())
                        .collect(),
                );
                index = m + sub.len();
            }
            None => break,
        }
    }
    if rs.len() == 0 {
        rs.push("".to_string());
    }
    rs
}

#[extendr]
fn str_length(str: String) -> i64 {
    let mut count = 0;
    let mut cur = 0;

    for c in str.as_bytes() {
        if cur == 0 {
            if (c & 0b10000000) == 0 {
                count += 1;
                cur = 0;
                continue;
            }
            if (c & 0b11100000) == 0b11000000 {
                count += 1;
                cur = 1;
                continue;
            }
            if (c & 0b11110000) == 0b11100000 {
                count += 1;
                cur = 2;
                continue;
            }
            if (c & 0b11111000) == 0b11110000 {
                count += 1;
                cur = 3;
                continue;
            }
        }
        cur -= 1;
    }

    count
}
