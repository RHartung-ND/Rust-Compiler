use crate::scan::{scan, Lexer};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

pub fn parse(path: &String, verbose: bool) -> i32 {
    if scan(path, false) == 1 {
        return 1;
    }

    let input = std::fs::read_to_string(path).expect("Should have been able to read the file");

    match grammar::ProgramParser::new().parse(Lexer::new(&input)) {
        Ok(()) => {
            if verbose {
                println!("Parse successful!");
            }
            return 0;
        }
        Err(e) => {
            println!("Parse failed: {:?}", e);
            return 1;
        }
    }
}