use crate::parse::parse;
use crate::decl::decl_print;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

pub const DEFAULT_INDENT: i8 = 4;

pub fn print(path: &String) -> i32 {
    if parse(path, false) == 1 {
        return 1;
    }

    let input = std::fs::read_to_string(path).expect("Should have been able to read the file");
    
    let ast = grammar::ProgramParser::new().parse(&input.to_string(), crate::scan::Lexer::new(&input)).unwrap();

    // println!("AST: {:?}", ast);

    decl_print(&ast.expect("There should be an AST"), 0);

    return 0;
}

pub fn print_indent(num_spaces: i8){
    for _ in 0..num_spaces * DEFAULT_INDENT {
        print!(" ");
    }
}
