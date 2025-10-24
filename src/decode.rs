use regex::Regex;
use crate::helper_functions::unicode_to_dec;

pub fn decode(es: String, s: &mut String) -> i32 {
                            // a,  b,  e,   f,   n,   r,   t,   v,   \,  ',  "
    let reserved: [u8; 11] = [97, 98, 101, 102, 110, 114, 116, 118, 92, 39, 34];
    let ascii:    [u8; 11] = [7,  8,  27,  12,  10,  13,  9,   11,  92, 39, 34];

    let mut idx: usize = 1;
    let len: usize  = es.len();
    let mut char_arr = vec![];


    if len < 2 {
        println!("String must start and end with a quotation mark.");
        return 1;
    }

    if es.as_bytes()[0] != 34 {
        println!("String must start with a quotation mark.");
        return 1;
    }

    if es.as_bytes()[len - 1] != 34 {
        println!("String must end with a quotation mark.");
        return 1;
    }

    if es.as_bytes()[len - 2] == 92 && es.as_bytes()[len - 3] != 92 {
        println!("Cannot escape closing quotation mark.");
        return 1;
    }

    while idx < len - 1{
        let character = es.chars().nth(idx).unwrap();
        let unicode = character.escape_unicode().to_string();
        let val = unicode_to_dec(&unicode);
        // println!("{character} : {unicode} : {val}");
        if val > 127 {
            println!("String may only contain valid ASCII characters between 0 and 127.");
            return 1;
        }
        if val == 92 { // backslash character handler
            let mut found: bool = false;
            // Hexadecimal handler
            if es.as_bytes()[idx + 1] == 48 && es.as_bytes()[idx + 2] == 120 {
                let re = Regex::new(r"([a-f]|[A-F]|[0-9]){2}").unwrap();
                let char1 = es.chars().nth(idx + 3).unwrap();
                let char2 = es.chars().nth(idx + 4).unwrap();
                let hex_code: String = format!("{char1}{char2}");
                if re.is_match(&hex_code) {
                    char_arr.push(u8::from_str_radix(&hex_code.to_string(), 16).unwrap());
                    idx += 5;
                    continue;
                }
            }

            // Escape sequence handler
            for j in 0..11 {
                if es.as_bytes()[idx + 1] == reserved[j] {
                    char_arr.push(ascii[j]);
                    found = true;
                    idx += 2;
                    break;
                }
            }

            // Escape sequence handler (non-reserved characters)
            if found == false {
                char_arr.push(es.as_bytes()[idx + 1]);
                idx += 2;
            }
            continue;
        } else if val == 34 {
            if es.as_bytes()[idx - 1] != 92 {
                println!("Double quotation marks must be escaped.");
                return 1;
            }
            idx += 1;
            continue;
        }
        
        char_arr.push(val as u8);
        idx += 1;
    }

    if char_arr.len() > 255 {
        println!("Strings may contain up to 255 characters.");
        return 1;
    }

    for d in char_arr.iter(){
        s.push(*d as char);
    }
    return 0;
}