use crate::decode::decode;

pub fn encode(s: String, es: String){
    println!("");
}
// void string_encode(const char *s, char *es){
//     es[0] = (char)34;
//                      // a,  b,  e,   f,   n,   r,   t,   v,   \,  ',  "
//     int reserved[11] = {97, 98, 101, 102, 110, 114, 116, 118, 92, 39, 34};
//     int ascii[11]    = {7,  8,  27,  12,  10,  13,  9,   11,  92, 39, 34};

//     int idx = 0;
//     int insert = 1;
//     int len = strlen(s);

//     while (idx < len){
//         int val = (int)s[idx];
//         if ((val < 32) | (val > 127)){
//             if (val < 0){
//                 val += 256;
//             }
            
//             bool found = false;

//             for (int j = 0; j < 11; j++){
//                 if (val == ascii[j]){
//                     es[insert] = (char)92;
//                     es[insert + 1] = (char)reserved[j];
//                     found = true;
//                     idx++;
//                     insert += 2;
//                     break;
//                 }
//             }

//             if (found == false){
//                 char hex[5];
//                 sprintf(hex, "\\0x%X", val);
//                 strcat(es, hex);
//                 idx++;
//                 insert += 5;
//             }
//             continue;
//         }
        
//         if (val == 34){
//             es[insert] = (char)92;
//             es[insert + 1] = (char)val;
//             idx++;
//             insert += 2;
//             continue;    
//         }
//         es[insert] = (char)val;
//         idx++;
//         insert++;
//     }

//     es[insert] = (char)34;
//     printf("%s", es);
// }