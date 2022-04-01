
pub fn length_of(string: String) -> usize {
    let mut count = 0;
    let mut cur = 0;

    for c in string.as_bytes() {
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

pub fn validate(string: String) -> bool {
    let mut cur = 0;
    let mut count = 0;

    for c in string.as_bytes() {
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
        if (c & 0b11000000) != 0b10000000 {
            return false;
        }
        if count == 0 {
            return false; 
        }
        cur -= 1;
    }

    true
}

pub fn convert_utf8_from(string: String) -> Vec<u32> {
    let mut count = 0;
    let mut step = 0;
    let mut result = vec![];

    if !validate(string.clone()) {
        return result;
    }

    let mut temp = 0;
    for c in string.as_bytes() {
        if count == 0 {
            if (c & 0b10000000) == 0 {
                result.push(temp);
                temp = (*c as u32) & 0b01111111;
                count = 0;
                step = 2;
                continue;
            }
            if (c & 0b11100000) == 0b11000000 {
                result.push(temp);
                temp = (*c as u32) & 0b00011111;
                temp = temp << 6;
                count = 1;
                step = 2;
                continue;
            }
            if (c & 0b11110000) == 0b11100000 {
                result.push(temp);
                temp = (*c as u32) & 0b00001111;
                temp = temp << 12;
                count = 2;
                step = 2;
                continue;
            }
            if (c & 0b11111000) == 0b11110000 {
                result.push(temp);
                temp = (*c as u32) & 0b00000111;
                temp = temp << 18;
                count = 3;
                step = 2;
                continue;
            }
        }
        count -= 1;
        step -= 1;
        temp |= ((*c as u32) & 0b00111111) << (step * 6);
    }
    result.push(temp);

    if let Some(x) = result.first() {
        if *x == 0 {
            result.remove(0);
        }
    }
    if let Some(x) = result.last() {
        if *x == 0 {
            result.pop();
        }
    }

    result
}

pub fn convert_utf8_to(vec: Vec<u32>) -> String {
    let mut result = Vec::new();
    
    for v in vec {
        println!("{:?}", v);
        if v <= 0x0000007F {
            result.push(v as u8);
            continue;
        }
        if v <= 0x000007FF {
            result.push((((v >> 6) & 0b00011111) | 0b11000000) as u8);
            result.push(((v & 0b00111111) | 0b10000000) as u8);
            continue;
        }
        if v <= 0x0000FFFF {
            result.push((((v >> 12) & 0b00001111) | 0b11100000) as u8);
            result.push((((v >> 6) & 0b00111111) | 0b10000000) as u8);
            result.push(((v & 0b00111111) | 0b10000000) as u8);
            continue;
        }
        if v <= 0x0010FFFF {
            result.push((((v >> 18) & 0b00000111) | 0b11110000) as u8);
            result.push((((v >> 12) & 0b00111111) | 0b10000000) as u8);
            result.push((((v >> 6) & 0b00111111) | 0b10000000) as u8);
            result.push(((v & 0b00111111) | 0b10000000) as u8);
            continue;
        }
    }

    match String::from_utf8(result) {
        Ok(x) => x,
        Err(_) => String::from(""),
    }
}

pub fn disable_regex(string: String) -> String {
    let mut result = String::new();

    for c in string.chars() {
        if c == '[' || c == ']' || c == '(' || c == ')' || c == '{' || c == '}' || c == '.' || c == '*' || c == '+' || c == '?' || c == '|' || c == '^' || c == '$' || c == '\\' {
            result.push('\\');
        }
        result.push(c);
    }

    result
}

pub fn add_i_flag_to(string: String) -> String {
    "(?i)".to_string() + &string
}