mod decode;
mod encode;
mod helper_functions;
use std::process::ExitCode;

fn main() -> ExitCode{
    let args: Vec<String> = std::env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    if query.to_string() == "--encode" {
        let contents = std::fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        let mut decoded_str = String::from("");
        let mut encoded_str = String::from("");
        if decode::decode(contents, &mut decoded_str) != 0 {
            return ExitCode::from(1);
        }
        println!("{decoded_str}");
        encode::encode(decoded_str, &mut encoded_str);
    } else{
        println!("invalid query");
    }
    ExitCode::SUCCESS
}
