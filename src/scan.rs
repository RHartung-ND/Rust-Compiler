use logos::Logos;
use crate::decode::decode;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = String)]

// Skip the following
#[logos(skip r"[ \n\t\r]+")]
#[logos(skip r"//(.*)")]
#[logos(skip "[/][*][^*]*[*]+([^*/][^*]*[*]+)*[/]")]

// Allow for all uppercase variables
#[allow(non_camel_case_types)]
enum Token {

    #[regex("[/][*]")] TOKEN_UNMATCHED_COMMENT,


    #[token("array")]     TOKEN_ARRAY,
    #[token("auto")]      TOKEN_AUTO,
    #[token("boolean")]   TOKEN_BOOLEAN,
    #[token("carray")]    TOKEN_CARRAY,
    #[token("char")]      TOKEN_CHAR,
    #[token("double")]    TOKEN_DOUBLE,
    #[token("else")]      TOKEN_ELSE,
    #[token("false")]     TOKEN_FALSE,
    #[token("float")]     TOKEN_FLOAT,
    #[token("for")]       TOKEN_FOR,
    #[token("function")]  TOKEN_FUNCTION,
    #[token("if")]        TOKEN_IF,
    #[token("integer")]   TOKEN_INTEGER,
    #[token("print")]     TOKEN_PRINT,
    #[token("return")]    TOKEN_RETURN,
    #[token("string")]    TOKEN_STRING,
    #[token("true")]      TOKEN_TRUE,
    #[token("void")]      TOKEN_VOID,
    #[token("while")]     TOKEN_WHILE,

    #[token("(")]         TOKEN_LPAREN,
    #[token(")")]         TOKEN_RPAREN,
    #[token("[")]         TOKEN_LBRACKET,
    #[token("]")]         TOKEN_RBRACKET,
    #[token("++")]        TOKEN_INCREMENT,
    #[token("--")]        TOKEN_DECREMENT,
    #[token("#")]         TOKEN_UNARY,
    #[token("+")]         TOKEN_PLUS,
    #[token("-")]         TOKEN_MINUS,
    #[token("/")]         TOKEN_DIVIDE,
    #[token("*")]         TOKEN_TIMES,
    #[token("%")]         TOKEN_MOD,
    #[token(";")]         TOKEN_SEMICOLON,
    #[token(":")]         TOKEN_COLON,
    #[token(",")]         TOKEN_COMMA,
    #[token("!")]         TOKEN_NOT,
    #[token("{")]         TOKEN_LBRACE,
    #[token("}")]         TOKEN_RBRACE,
    #[token("^")]         TOKEN_CARET,
    #[token("<")]         TOKEN_LESSTHAN,
    #[token(">")]         TOKEN_GREATERTHAN,
    #[token("=")]         TOKEN_EQUALS,
    #[token("<=")]        TOKEN_LEQ,
    #[token(">=")]        TOKEN_GEQ,
    #[token("!=")]        TOKEN_NEQ,
    #[token("==")]        TOKEN_EQUIVALENT,
    #[token("||")]        TOKEN_OR,
    #[token("&&")]        TOKEN_AND,
    
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*")]               TOKEN_IDENTIFIER,
    #[regex(r"([0-9]+)|(0x[0-9a-zA-Z]+)|(0b[01]+)")]  TOKEN_INTEGER_LITERAL,
    #[regex(r"[0-9]*[.][0-9]+")]                      TOKEN_DOUBLE_LITERAL,
    #[regex(r#"\"(\\.|[^\\\"\n])*\""#)]               TOKEN_STRING_LITERAL,
    #[regex(r"'(.|\\0x([a-fA-F0-9]){2})'")]           TOKEN_CHAR_LITERAL,
}

pub fn scan(file_path: &String, verbose: bool) -> i32 {
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lexer = Token::lexer(&contents);
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => {
                let yytext = lexer.slice().to_string();                    
                if verbose == true {
                    match token {
                        Token::TOKEN_UNMATCHED_COMMENT => {
                            println!("scan error: Unterminated comment");
                            return 1;
                        },
                        
                        Token::TOKEN_INTEGER_LITERAL => println!("{:?} {}", token, yytext),
                        Token::TOKEN_DOUBLE_LITERAL  => println!("{:?} {}", token, yytext),
                        
                        Token::TOKEN_STRING_LITERAL => {
                            let mut temp_string = String::from("");
                            if decode(yytext.to_owned(), &mut temp_string) == 1 {
                                println!("scan error: {} is not a valid string", yytext);
                                return 1;
                            }
                            println!("{:?} {}", token, yytext);
                        },

                        Token::TOKEN_CHAR_LITERAL => {
                            if yytext.len() > 3 {
                                let temp_string = format!("{}{}", yytext.chars().nth(4).unwrap(),
                                    yytext.chars().nth(5).unwrap());
                                let decimal = u8::from_str_radix(&temp_string, 16).unwrap();
                                println!("{:?} '{}'", token, decimal as char);
                            } else {
                                println!("{:?} {}", token, yytext);
                            }
                        },

                        Token::TOKEN_IDENTIFIER => {
                            if yytext.len() > 255 {
                                println!("scan error: identifier must be less than 255 characters long");
                                return 1;
                            }
                            println!("{:?} {}", token, yytext);

                        },

                        _ => println!("{:?}", token)
                    }
                } else {
                    match token {
                        Token::TOKEN_UNMATCHED_COMMENT => {
                            println!("scan error: Unterminated comment");
                            return 1;
                        },

                        Token::TOKEN_IDENTIFIER => {
                            if yytext.len() > 255 {
                                println!("scan error: identifier must be less than 255 characters long");
                                return 1;
                            }
                        },
                                                    
                        Token::TOKEN_STRING_LITERAL => {
                            let mut temp_string = String::from("");
                            if decode(yytext.to_owned(), &mut temp_string) == 1 {
                                println!("scan error: {} is not a valid string", yytext);
                                return 1;
                            }
                        },
                        _ => println!("{:?}", token)
                    }
                }
            },
            Err(_) => {
                println!("scan error: {} is not a valid character", lexer.slice().to_string());
                return 1;
            }
        }
    }
    return 0;
}