use crate::decode::decode;
use lrlex::lrlex_mod;
use lrpar::{Lexer, Lexeme};
lrlex_mod!("scanner.l");


fn token_id_to_name(token_id: usize) -> &'static str {
    // Old names.
    // When I added the grammar, it reordered the lexemes to be in order of when they were defined. It used
    //     to be in the order defined by the scanner.l file. I don't know why they decided this. The new order
    //     can be found at target/release/build/bminor-<some numbers>/out/scanner.l.rs starting on line 299.
    
    // let names: [&'static str; 57] = ["None", "None", "TOKEN_UNMATCHED_COMMENT","TOKEN_ARRAY","TOKEN_AUTO","TOKEN_BOOLEAN","TOKEN_CARRAY","TOKEN_CHAR","TOKEN_DOUBLE","TOKEN_ELSE","TOKEN_FALSE","TOKEN_FLOAT","TOKEN_FOR","TOKEN_FUNCTION","TOKEN_IF","TOKEN_INTEGER","TOKEN_PRINT","TOKEN_RETURN","TOKEN_STRING","TOKEN_TRUE","TOKEN_VOID","TOKEN_WHILE","TOKEN_LPAREN","TOKEN_RPAREN","TOKEN_LBRACKET","TOKEN_RBRACKET","TOKEN_INCREMENT","TOKEN_DECREMENT","TOKEN_UNARY","TOKEN_PLUS","TOKEN_MINUS","TOKEN_DIVIDE","TOKEN_TIMES","TOKEN_MOD","TOKEN_SEMICOLON","TOKEN_COLON","TOKEN_COMMA","TOKEN_NOT","TOKEN_LBRACE","TOKEN_RBRACE","TOKEN_CARET","TOKEN_LESSTHAN","TOKEN_GREATERTHAN","TOKEN_EQUALS","TOKEN_LEQ","TOKEN_GEQ","TOKEN_NEQ","TOKEN_EQUIVALENT","TOKEN_OR","TOKEN_AND","TOKEN_IDENTIFIER","TOKEN_INTEGER_LITERAL","TOKEN_DOUBLE_LITERAL","TOKEN_STRING_LITERAL","TOKEN_CHAR_LITERAL", "None", "UNMATCHED"];

    let names: [&'static str; 55] = ["TOKEN_COLON", "TOKEN_SEMICOLON", "TOKEN_EQUALS", "TOKEN_LBRACE", "TOKEN_RBRACE", "TOKEN_INTEGER_LITERAL", "TOKEN_DOUBLE_LITERAL", "TOKEN_STRING_LITERAL", "TOKEN_CHAR_LITERAL", "TOKEN_FALSE", "TOKEN_TRUE", "TOKEN_STRING", "TOKEN_BOOLEAN", "TOKEN_AUTO", "TOKEN_INTEGER", "TOKEN_DOUBLE", "TOKEN_CHAR", "TOKEN_VOID", "TOKEN_ARRAY", "TOKEN_LBRACKET", "TOKEN_RBRACKET", "TOKEN_CARRAY", "TOKEN_FUNCTION", "TOKEN_LPAREN", "TOKEN_RPAREN", "TOKEN_COMMA", "TOKEN_IDENTIFIER", "TOKEN_OR", "TOKEN_AND", "TOKEN_EQUIVALENT", "TOKEN_NEQ", "TOKEN_GREATERTHAN", "TOKEN_GEQ", "TOKEN_LESSTHAN", "TOKEN_LEQ", "TOKEN_PLUS", "TOKEN_MINUS", "TOKEN_TIMES", "TOKEN_DIVIDE", "TOKEN_MOD", "TOKEN_CARET", "TOKEN_UNARY", "TOKEN_NOT", "TOKEN_INCREMENT", "TOKEN_DECREMENT", "TOKEN_IF", "TOKEN_ELSE", "TOKEN_FOR", "TOKEN_RETURN", "TOKEN_PRINT", "TOKEN_WHILE", "TOKEN_ERROR", "TOKEN_UNMATCHED_COMMENT", "TOKEN_C_COMMENT", "TOKEN_CPP_COMMENT"];

    return names[token_id];
}

pub fn scan(path: &String, verbose: bool) -> i32 {
    let input = std::fs::read_to_string(path).expect("Should have been able to read the file");
    

    let lexerdef = scanner_l::lexerdef();
    let lexer = lexerdef.lexer(&input);

    for lexeme_result in lexer.iter() {
        match lexeme_result {
            Ok(lexeme) => {
                let token_id = lexeme.tok_id() as usize;
                let token_name = token_id_to_name(token_id);
                let token_text = &input[lexeme.span().start()..lexeme.span().end()];
                match token_name {
                    "TOKEN_UNMATCHED_COMMENT" => {
                        println!("scan error: Unterminated comment");
                        return 1;
                    },

                    "TOKEN_INTEGER_LITERAL" | "TOKEN_DOUBLE_LITERAL" => {
                        if verbose {println!("{} {}", token_name, token_text)};
                    },
                        
                    "TOKEN_STRING_LITERAL" => {
                        if decode(token_text.to_owned(), &mut String::from("")) == 1 {
                            println!("scan error: {} is not a valid string", token_text);
                            return 1;
                        }
                        if verbose {println!("{} {}", token_name, token_text)}
                    },

                    "TOKEN_CHAR_LITERAL" => {
                        if token_text.contains("\\x") && token_text.len() == 6 {
                            // Hex escape: '\xHH'
                            let temp_string = format!("{}{}",
                                token_text.chars().nth(4).unwrap(),
                                token_text.chars().nth(5).unwrap());
                            let decimal = u8::from_str_radix(&temp_string, 16).unwrap();
                            if verbose { println!("{} '{}'", token_name, decimal as char) }
                        } else {
                            // Regular escape or single char
                            if verbose { println!("{} {}", token_name, token_text) }
                        }
                    },

                    "TOKEN_IDENTIFIER" => {
                        if token_text.len() > 255 {
                            println!("scan error: identifier must be less than 255 characters long");
                            return 1;
                        }
                        if verbose {println!("{} {}", token_name, token_text)}
                    },

                    "TOKEN_ERROR" => {
                        println!("scan error: {} is not a valid character", token_text);
                        println!("Error occurred at position {}", lexeme.span().start());
                        return 1;
                    },

                    _ => if verbose {println!("{}", token_name)}
                }
            }
            Err(e) => eprintln!("Lex error: {:?}", e),
        }
    }
    return 0;
}
