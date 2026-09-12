use crate::scan::scan;

use lrlex::lrlex_mod;
use lrpar::{Lexer, Lexeme};
lrlex_mod!("scanner.l");
lrlex_mod!("parser.y");

use crate::ast::*;

pub fn parse(contents: &String, verbose: bool) -> i32 {
    if scan(contents, false) == 1 {
        return 1;
    }

    let input = std::fs::read_to_string(contents).expect("Should have been able to read the file");
    
    let lexerdef = scanner_l::lexerdef();
    let lexer = lexerdef.lexer(&input);

    let (res, errs) = parser_y::parse(&lexer);
    for e in errs {
        if verbose {
            println!("Parse Failed!");
            println!("{:?}", e);
            return 1;
        }
        // println!("{}", e.pp(&lexer, &parser_y::token_epp));
    }

    // println!("{:?}", res);
    if verbose {
        println!("Parse successful!");
    }

    // match parse_trees {
    //     Err(_) => {
    //         if verbose {println!("Parse Failed!")}
    //         return 1;
    //     },

    //     _ => {
    //         if verbose {println!("Parse successful!")}
    //     }
    // }
    return 0;
}