
use crate::utf8;


pub fn single_utf8_str_find(target: String, sub: String) -> Option<(usize, usize)> {
    let target_converted = utf8::convert_utf8_from(target.clone());
    let sub_converted = utf8::convert_utf8_from(sub.clone());
    
    let mut indices = vec![0; target_converted.len()];
    
    None
}