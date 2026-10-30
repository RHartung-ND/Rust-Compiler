mod decode;
mod encode;
mod scan;
mod parse;
mod print;
mod param_list;

mod decl;
mod data_type;
mod stmt;
mod expr;
mod ast;

mod helper_functions;
use std::process::ExitCode;

fn main() -> ExitCode{
    let args: Vec<String> = std::env::args().collect();

    let query = &args[1].to_string();
    let file_path = &args[2].to_string();

    match query.as_str() {
        "--encode" => {
            let contents = std::fs::read_to_string(file_path).expect("Should have been able to read the file");
            let mut decoded_str = String::from("");
            let mut encoded_str = String::from("");
            if decode::decode(contents, &mut decoded_str) != 0 {
                return ExitCode::from(1);
            }
            encode::encode(decoded_str, &mut encoded_str);
        },
        "--scan" => {
            if scan::scan(&file_path, true) != 0 {
                return ExitCode::from(1);
            }
        },
        "--parse" => {
            if parse::parse(&file_path, true) != 0 {
                return ExitCode::from(1);
            }
        },
        "--print" => {
            if print::print(&file_path, true) != 0 {
                return ExitCode::from(1);
            }
        },
        _ => {
            println!("invalid query");
            return ExitCode::from(1);

        }
    }
    return ExitCode::from(0);
}
