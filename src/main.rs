mod decode;
mod encode;
use std::process::ExitCode;

fn main() -> ExitCode{
    let args: Vec<String> = std::env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    if query.to_string() == "--encode" {
        let contents = std::fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        let encoded_str = "";
        let decoded_str = "";
        if decode::decode(contents, encoded_str) != 0 {
            return ExitCode::from(1);
        }
        println!("{encoded_str}");
        // encode::encode(encoded_str, decoded_str);
    } else{
        println!("invalid query");
    }
    ExitCode::SUCCESS
}
