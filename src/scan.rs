use santiago::lexer::LexerRules;
use crate::decode::decode;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // Comments and whitespace
        "DEFAULT" | "WS" = pattern r"[ \n\t\r]+" => |lexer| lexer.skip();
        "DEFAULT" | "C_COMMENT" = pattern r"//(.*)" => |lexer| lexer.skip();
        "DEFAULT" | "CPP_COMMENT" = pattern "[/][*][^*]*[*]+([^*/][^*]*[*]+)*[/]" => |lexer| lexer.skip();
        "DEFAULT" | "TOKEN_UNMATCHED_COMMENT" = pattern r"[/][*]";

        // Types
        "DEFAULT" | "TOKEN_ARRAY" = string "array";
        "DEFAULT" | "TOKEN_AUTO" = string "auto";
        "DEFAULT" | "TOKEN_BOOLEAN" = string "boolean";
        "DEFAULT" | "TOKEN_CARRAY" = string "carray";
        "DEFAULT" | "TOKEN_CHAR" = string "char";
        "DEFAULT" | "TOKEN_DOUBLE" = string "double";
        "DEFAULT" | "TOKEN_ELSE" = string "else";
        "DEFAULT" | "TOKEN_FALSE" = string "false";
        "DEFAULT" | "TOKEN_FLOAT" = string "float";
        "DEFAULT" | "TOKEN_FOR" = string "for";
        "DEFAULT" | "TOKEN_FUNCTION" = string "function";
        "DEFAULT" | "TOKEN_IF" = string "if";
        "DEFAULT" | "TOKEN_INTEGER" = string "integer";
        "DEFAULT" | "TOKEN_PRINT" = string "print";
        "DEFAULT" | "TOKEN_RETURN" = string "return";
        "DEFAULT" | "TOKEN_STRING" = string "string";
        "DEFAULT" | "TOKEN_TRUE" = string "true";
        "DEFAULT" | "TOKEN_VOID" = string "void";
        "DEFAULT" | "TOKEN_WHILE" = string "while";

        // Symbols
        "DEFAULT" | "TOKEN_LPAREN" = string "(";
        "DEFAULT" | "TOKEN_RPAREN" = string ")";
        "DEFAULT" | "TOKEN_LBRACKET" = string "[";
        "DEFAULT" | "TOKEN_RBRACKET" = string "]";
        "DEFAULT" | "TOKEN_INCREMENT" = string "++";
        "DEFAULT" | "TOKEN_DECREMENT" = string "--";
        "DEFAULT" | "TOKEN_UNARY" = string "#";
        "DEFAULT" | "TOKEN_PLUS" = string "+";
        "DEFAULT" | "TOKEN_MINUS" = string "-";
        "DEFAULT" | "TOKEN_DIVIDE" = string "/";
        "DEFAULT" | "TOKEN_TIMES" = string "*";
        "DEFAULT" | "TOKEN_MOD" = string "%";
        "DEFAULT" | "TOKEN_SEMICOLON" = string ";";
        "DEFAULT" | "TOKEN_COLON" = string ":";
        "DEFAULT" | "TOKEN_COMMA" = string ",";
        "DEFAULT" | "TOKEN_NOT" = string "!";
        "DEFAULT" | "TOKEN_LBRACE" = string "{";
        "DEFAULT" | "TOKEN_RBRACE" = string "}";
        "DEFAULT" | "TOKEN_CARET" = string "^";
        "DEFAULT" | "TOKEN_LESSTHAN" = string "<";
        "DEFAULT" | "TOKEN_GREATERTHAN" = string ">";
        "DEFAULT" | "TOKEN_EQUALS" = string "=";
        "DEFAULT" | "TOKEN_LEQ" = string "<=";
        "DEFAULT" | "TOKEN_GEQ" = string ">=";
        "DEFAULT" | "TOKEN_NEQ" = string "!=";
        "DEFAULT" | "TOKEN_EQUIVALENT" = string "==";
        "DEFAULT" | "TOKEN_OR" = string "||";
        "DEFAULT" | "TOKEN_AND" = string "&&";
    
        // Literals
        "DEFAULT" | "TOKEN_IDENTIFIER" = pattern r"[_a-zA-Z][_a-zA-Z0-9]*";
        "DEFAULT" | "TOKEN_INTEGER_LITERAL" = pattern r"([0-9]+)|(0x[0-9a-zA-Z]+)|(0b[01]+)";
        "DEFAULT" | "TOKEN_DOUBLE_LITERAL" = pattern r"[0-9]*[.][0-9]+";
        "DEFAULT" | "TOKEN_STRING_LITERAL" = pattern r#"\"(\\.|[^\\\"\n])*\""#;
        "DEFAULT" | "TOKEN_CHAR_LITERAL" = pattern r"'(.|\\0x([a-fA-F0-9]){2})'";

        // Error
        "DEFAULT" | "TOKEN_ERROR" = pattern r".";
    )
}

pub fn scan(contents: &String, verbose: bool) -> i32 {
    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &contents).unwrap();

    for lex in lexemes.iter() {
        let token: &str = &lex.kind;
        let yytext = lex.raw.to_string();
        match token {
            "TOKEN_UNMATCHED_COMMENT" => {
                println!("scan error: Unterminated comment");
                return 1;
            },

            "TOKEN_INTEGER_LITERAL" | "TOKEN_DOUBLE_LITERAL" => {
                if verbose {println!("{} {}", token, yytext)}
            },
                        
            "TOKEN_STRING_LITERAL" => {
                if decode(yytext.to_owned(), &mut String::from("")) == 1 {
                    println!("scan error: {} is not a valid string", yytext);
                    return 1;
                }
                if verbose {println!("{} {}", token, yytext)}
            },

            "TOKEN_CHAR_LITERAL" => {
                if yytext.len() > 3 {
                    let temp_string = format!("{}{}", yytext.chars().nth(4).unwrap(),
                        yytext.chars().nth(5).unwrap());
                    let decimal = u8::from_str_radix(&temp_string, 16).unwrap();
                    if verbose {println!("{} '{}'", token, decimal as char)}                    
                } else {
                    if verbose {println!("{} {}", token, yytext)}                    
                }
            },

            "TOKEN_IDENTIFIER" => {
                if yytext.len() > 255 {
                    println!("scan error: identifier must be less than 255 characters long");
                    return 1;
                }
                if verbose {println!("{} {}", token, yytext)}
            },

            "TOKEN_ERROR" => {
                println!("scan error: {} is not a valid character", yytext);
                println!("Error occurred at position {}", lex.position);
                return 1;
            },

            _ => if verbose {println!("{}", token)}
        }
    }
    return 0;
}