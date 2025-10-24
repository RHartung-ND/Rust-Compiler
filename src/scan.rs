use logos::Logos;
use crate::decode::decode;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \n\t\r]+")]
#[logos(error = String)]
enum Token {
    #[token("+")]
    TokenPlus,

    #[token("-")]
    TokenMinus,

    #[token("*")]
    TokenTimes,

    #[token("/")]
    TokenDivide,

    #[token("(")]
    TokenLParen,

    #[token(")")]
    TokenRParen,

    #[regex("([0-9]+)|(0x[0-9a-zA-Z]+)|(0b[01]+)")]
    TokenIntegerLiteral ,

    #[regex("\"(\\.|[^\\\"\n])*\"")]
    TokenStringLiteral,
}

pub fn scan(file_path: &String, verbose: bool) -> i32 {
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lexer = Token::lexer(&contents);
    if verbose == true {
        while let Some(token) = lexer.next() {
            match token {
                Ok(token) => {
                    let yytext = lexer.slice().to_string();
                    match token {
                        Token::TokenIntegerLiteral => println!("{:?} {}", token, yytext),
                        Token::TokenStringLiteral => {
                            let mut temp_string = String::from("");
                            if decode(yytext.to_owned(), &mut temp_string) == 1 {
                                println!("scan error: {} is not a valid string", yytext);
                                return 1;
                            }
                            println!("{:?} {}", token, yytext);
                        },
                        _ => println!("{:?}", token)
                    }
                },
                Err(_) => {
                    println!("lexer error");
                    return 1;
                }
            }
        }
    }
    return 0;
}