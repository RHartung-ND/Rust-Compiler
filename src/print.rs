use crate::parse::parse;
use crate::decl::decl_print;
const DEFAULT_INDENT: i8 = 4;

pub fn print(contents: &String, verbose: bool) -> i32 {
    if parse(contents, false) == 1 {
        return 1;
    }
    return 0;
}

pub fn print_indent(num_spaces: i8){
    for _ in 0..num_spaces * DEFAULT_INDENT {
        print!(" ");
    }
}