use crate::decl::decl_create;
use crate::parse::parse;


// use crate::decl::decl_print;
const DEFAULT_INDENT: i8 = 4;

pub fn print(contents: &String, verbose: bool) -> i32 {
    if parse(contents, false) == 1 {
        return 1;
    }

    // match parse_trees {
    //     Err(_) => {
    //         if verbose {println!("Parse Failed!")}
    //         return 1;
    //     },

    //     _ => {
    //         println!("{:?}", parse_trees);
    //     }
    // }

    // decl_print(ast, 0);
    return 0;
}

pub fn print_indent(num_spaces: i8){
    for _ in 0..num_spaces * DEFAULT_INDENT {
        print!(" ");
    }
}