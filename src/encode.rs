pub fn encode(s: String, es: &mut String){
                           // a,  b,  e,   f,   n,   r,   t,   v,   \,  ',  "
    let reserved: [u8; 11] = [97, 98, 101, 102, 110, 114, 116, 118, 92, 39, 34];
    let ascii:    [u8; 11] = [7,  8,  27,  12,  10,  13,  9,   11,  92, 39, 34];

    let mut idx: usize = 0;
    let len: usize  = s.len();
    let mut result: String = String::from("");

    result.push('"');
    while idx < len {
        let val = s.as_bytes()[idx];
        if val < 32 || val > 127 {
            let unicode_char = s.chars().nth(idx);
            let unicode_string = unicode_char.unwrap().escape_unicode().to_string();
            let mut combined_val = String::from("");
            combined_val.push(unicode_string.chars().nth(3).unwrap());
            combined_val.push(unicode_string.chars().nth(4).unwrap());

            println!("ASCII val: {:?}", combined_val);
            let mut found: bool = false;
            for j in 1..11 {
                if val == ascii[j] {
                    result.push('\\');
                    result.push(reserved[j] as char);
                    found = true;
                    idx += 2;
                    break;
                }
            }

            if found == false {
                result.push_str("\\0x");
                result = format!("{}{:X}", result, val);
                idx += 1;
            }
            continue;
        }
        
        if val == 34 {
            result.push_str("\\\"");
            idx += 1;
            continue;    
        }

        result.push(val as char);
        idx += 1;
    }

    result.push('"');

    es.push_str(&result);
    print!("{result}");
//     printf("%s", es);
}