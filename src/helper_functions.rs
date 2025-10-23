pub fn unicode_to_dec(unicode: &str) -> u8 {
    if unicode.len() == 5 {
        return u8::from_str_radix(&unicode.chars().nth(3).unwrap().to_string(), 16).unwrap();
    }
    let char1 = unicode.chars().nth(3).unwrap();
    let char2 = unicode.chars().nth(4).unwrap();
    return u8::from_str_radix(&format!("{char1}{char2}"), 16).unwrap();
}