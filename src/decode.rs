use regex::Regex;

pub fn decode(es: String, s: &str) -> i32 {
                            // a,  b,  e,   f,   n,   r,   t,   v,   \,  ',  "
    let reserved: [i32; 11] = [97, 98, 101, 102, 110, 114, 116, 118, 92, 39, 34];
    let ascii:    [i32; 11] = [7,  8,  27,  12,  10,  13,  9,   11,  92, 39, 34];

    let mut idx: usize = 1;
    let mut insert: i32 = 0;
    let len: usize  = es.len();

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
        let val = es.as_bytes()[idx];
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
                let hay = format!("{char1}{char2}");
                if re.is_match(&hay) {
                    let dec = i32::from_str_radix(&hay, 16).unwrap();
                    // s[insert] = (char)dec;
                    // insert += 1;
                    // idx += 5;
                    // continue;
                }
            }
            
//             // Escape sequence handler
//             for (int j = 0; j < 11; j++){
//                 if ((int)es[idx + 1] == reserved[j]){
//                     s[insert] = (char)ascii[j];
//                     found = true;
//                     idx += 2;
//                     insert++;
//                     break;
//                 }
//             }

//             // Escape sequence handler (non-reserved characters)
//             if (found == false){
//                 s[insert] = (char)es[idx + 1];
//                 idx += 2;
//                 insert++;
//             }
//             continue;
//         } else if ((val == 34)){
//             if ((int)es[idx - 1] != 92){
//                 printf("Double quotation marks must be escaped.");
//                 return 1;
//             }
//             idx++;
//             continue;
        }
        
//         s[insert] = (char)es[idx];
        idx += 1;
//         insert++;
    }

// if (strlen(s) > 255){
//         printf("Strings may contain up to 255 characters.\n");
//         return 1;
//     }
    return 0;
}




//     return 0;
// }