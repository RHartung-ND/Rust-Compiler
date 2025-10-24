pub fn unicode_to_dec(unicode: &str) -> i32 {
    let mut dec_string: String = String::from("");
    let mut idx = 3;
    let arr = unicode.as_bytes();
    while arr[idx] != 125 {
        dec_string.push(unicode.chars().nth(idx).unwrap());
        idx += 1;
    }
    return i32::from_str_radix(&dec_string, 16).unwrap();
}