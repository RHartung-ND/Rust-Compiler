use std::fmt;
use std::num::ParseIntError;
use crate::decode::decode;
use logos::{Logos, SpannedIter};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalError::InvalidInteger(e) => write!(f, "invalid integer: {}", e),
            LexicalError::InvalidToken => write!(f, "invalid token"),
        }
    }
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexicalError)]

// Skip the following
#[logos(skip r"[ \n\t\r]+")]
#[logos(skip(r"\/\/(.*)", allow_greedy = true))]
#[logos(skip "[/][*][^*]*[*]+([^*/][^*]*[*]+)*[/]")]

// Allow for all uppercase variables
#[allow(non_camel_case_types)]
pub enum Token {
    #[regex("[/][*]")] TOKEN_UNMATCHED_COMMENT,

    #[token("array")]    TOKEN_ARRAY,
    #[token("auto")]     TOKEN_AUTO,
    #[token("boolean")]  TOKEN_BOOLEAN,
    #[token("carray")]   TOKEN_CARRAY,
    #[token("char")]     TOKEN_CHAR,
    #[token("double")]   TOKEN_DOUBLE,
    #[token("else")]     TOKEN_ELSE,
    #[token("false")]    TOKEN_FALSE,
    #[token("float")]    TOKEN_FLOAT,
    #[token("function")] TOKEN_FUNCTION,
    #[token("if")]       TOKEN_IF,
    #[token("integer")]  TOKEN_INTEGER,
    #[token("print")]    TOKEN_PRINT,
    #[token("return")]   TOKEN_RETURN,
    #[token("string")]   TOKEN_STRING,
    #[token("true")]     TOKEN_TRUE,
    #[token("void")]     TOKEN_VOID,
    #[token("while")]    TOKEN_WHILE,

    #[token("(")]  TOKEN_LPAREN,
    #[token(")")]  TOKEN_RPAREN,
    #[token("[")]  TOKEN_LBRACKET,
    #[token("]")]  TOKEN_RBRACKET,
    #[token("++")] TOKEN_INCREMENT,
    #[token("--")] TOKEN_DECREMENT,
    #[token("#")]  TOKEN_UNARY,
    #[token("+")]  TOKEN_PLUS,
    #[token("-")]  TOKEN_MINUS,
    #[token("/")]  TOKEN_DIVIDE,
    #[token("*")]  TOKEN_TIMES,
    #[token("%")]  TOKEN_MOD,
    #[token(";")]  TOKEN_SEMICOLON,
    #[token(":")]  TOKEN_COLON,
    #[token(",")]  TOKEN_COMMA,
    #[token("!")]  TOKEN_NOT,
    #[token("{")]  TOKEN_LBRACE,
    #[token("}")]  TOKEN_RBRACE,
    #[token("^")]  TOKEN_CARET,
    #[token(">")]  TOKEN_GREATERTHAN,
    #[token("for")]      TOKEN_FOR,
    #[token("=")]  TOKEN_EQUALS,
    #[token("<")]  TOKEN_LESSTHAN,
    #[token("<=")] TOKEN_LEQ,
    #[token(">=")] TOKEN_GEQ,
    #[token("!=")] TOKEN_NEQ,
    #[token("==")] TOKEN_EQUIVALENT,
    #[token("||")] TOKEN_OR,
    #[token("&&")] TOKEN_AND,

    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*")]                TOKEN_IDENTIFIER,
    #[regex(r"([0-9]+)|(0x[0-9a-zA-Z]+)|(0b[01]+)")]   TOKEN_INTEGER_LITERAL,
    #[regex(r"[0-9]*[.][0-9]+")]                       TOKEN_DOUBLE_LITERAL,
    #[regex(r#"\"(\\.|[^\\\"\n])*\""#)]                TOKEN_STRING_LITERAL,
    #[regex(r#"\'(.|\\.|\\|\\0x([a-fA-F0-9]){2})\'"#)] TOKEN_CHAR_LITERAL,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub struct Lexer<'input> {
  // instead of an iterator over characters, we have a token iterator
  token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
  pub fn new(input: &'input str) -> Self {
    // the Token::lexer() method is provided by the Logos trait
    Self { token_stream: Token::lexer(input).spanned() }
  }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}


pub fn scan(path: &String, verbose: bool) -> i32 {
    let input = std::fs::read_to_string(path).expect("Should have been able to read the file");
    let mut lexer = Token::lexer(&input);
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => {
                let yytext = lexer.slice().to_string();
                match token {
                    Token::TOKEN_UNMATCHED_COMMENT => {
                        println!("scan error: Unterminated comment");
                        return 1;
                    }

                    Token::TOKEN_INTEGER_LITERAL | Token::TOKEN_DOUBLE_LITERAL => {
                        if verbose {println!("{:?} {}", token, yytext)};
                    }

                    Token::TOKEN_STRING_LITERAL => {
                        if decode(yytext.to_owned(), &mut String::from("")) == 1 {
                            println!("scan error: {} is not a valid string", yytext);
                            return 1;
                        }
                        if verbose {println!("{:?} {}", token, yytext)}
                    }

                    Token::TOKEN_CHAR_LITERAL => {
                        if yytext.contains("\\x") && yytext.len() == 6 {
                            // Hex escape: '\xHH'
                            let temp_string = format!("{}{}",
                                yytext.chars().nth(4).unwrap(),
                                yytext.chars().nth(5).unwrap());
                            let decimal = u8::from_str_radix(&temp_string, 16).unwrap();
                            if verbose { println!("{:?} '{}'", token, decimal as char) }
                        } else {
                            // Regular escape or single char
                            if verbose { println!("{:?} {}", token, yytext) }
                        }
                    }

                    Token::TOKEN_IDENTIFIER => {
                        if yytext.len() > 255 {
                            println!("scan error: identifier must be less than 255 characters long");
                            return 1;
                        }
                        if verbose {println!("{:?} {}", token, yytext)}
                    }

                    _ => { if verbose { println!("{:?}", token) } }
                }
            }
            Err(_) => {
                println!("scan error: {} is not a valid character", lexer.slice().to_string());
                return 1;
            }
        }
    }
    return 0;
}
