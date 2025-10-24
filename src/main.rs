mod decode;
mod encode;
mod scan;
mod helper_functions;
use std::process::ExitCode;

fn main() -> ExitCode{
    let args: Vec<String> = std::env::args().collect();

    let query = &args[1].to_string();
    let file_path = &args[2].to_string();

    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    if query == "--encode" {
        let mut decoded_str = String::from("");
        let mut encoded_str = String::from("");
        if decode::decode(contents, &mut decoded_str) != 0 {
            return ExitCode::from(1);
        }
        encode::encode(decoded_str, &mut encoded_str);
    } else if query == "--scan" {
        if scan::scan(&contents, true) != 0 {
            return ExitCode::from(1);
        }
    } else {
        println!("invalid query");
    }
    ExitCode::SUCCESS
}
