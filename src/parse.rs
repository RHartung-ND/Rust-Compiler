use crate::scan::scan;

pub fn parse(contents: &String, verbose: bool) -> i32 {
    if scan(contents, false) == 1{
        return 1;
    }
    if verbose == true {
        // let grammar = grammar();
        // let parse_trees = santiago::parser::parse(&grammar, &("INT", "10", (1,1))).unwrap();
    }
    return 0;
}