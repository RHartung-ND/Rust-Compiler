use crate::helper_functions::unicode_to_dec;

pub fn encode(s: String, es: &mut String){
                           // a,  b,  e,   f,   n,   r,   t,   v,   \,  ',  "
    let reserved: [u8; 11] = [97, 98, 101, 102, 110, 114, 116, 118, 92, 39, 34];
    let ascii:    [u8; 11] = [7,  8,  27,  12,  10,  13,  9,   11,  92, 39, 34];

    let mut idx: usize = 0;
    let len: usize  = s.chars().count();
    let mut result: String = String::from("");


    result.push('"');
    while idx < len {
        let character = s.chars().nth(idx).unwrap();
        let unicode = character.escape_unicode().to_string();
        let val = unicode_to_dec(&unicode);
        if val < 32 || val > 127 {
            let mut found: bool = false;
            for j in 0..11 {
                if val == ascii[j] {
                    result.push('\\');
                    result.push(reserved[j] as char);
                    found = true;
                    idx += 1;
                    break;
                }
            }

            if found == false {
                result.push('\\');
                result.push('0');
                result.push('x');
                result.push_str(&format!("{:X}", val));
                idx += 1;
            }
            continue;

        }
        
        if val == 34 {
            result.push('\\');
            result.push('"');
            idx += 1;
            continue;    
        }

        result.push(character);
        idx += 1;
    }

    result.push('"');
    es.push_str(&result);
    print!("{}", es);
}