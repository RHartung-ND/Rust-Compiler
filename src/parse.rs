use santiago::grammar::Grammar;
use crate::scan::scan;
use crate::scan::lexer_rules;

pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "program" => rules "decl_list";

        "decl_list" => rules "decl" "decl_list";
        "decl_list" => empty;

        "decl" => rules "ident" "TOKEN_COLON" "type" "TOKEN_SEMICOLON";
        "decl" => rules "ident" "TOKEN_COLON" "type" "TOKEN_EQUALS" "TOKEN_LBRACE" "expr_braced" "TOKEN_RBRACE" "TOKEN_SEMICOLON";
        "decl" => rules "ident" "TOKEN_COLON" "type" "TOKEN_EQUALS" "expr" "TOKEN_SEMICOLON";
        "decl" => rules "ident" "TOKEN_COLON" "type" "TOKEN_EQUALS" "stmt_brace";

        "literals" => rules "TOKEN_INTEGER_LITERAL";
        "literals" => rules "TOKEN_DOUBLE_LITERAL";
        "literals" => rules "TOKEN_STRING_LITERAL";
        "literals" => rules "TOKEN_CHAR_LITERAL";
        "literals" => rules "TOKEN_FALSE";
        "literals" => rules "TOKEN_TRUE";

        "type" => rules "TOKEN_STRING";
        "type" => rules "TOKEN_BOOLEAN";
        "type" => rules "TOKEN_AUTO";
        "type" => rules "TOKEN_INTEGER";
        "type" => rules "TOKEN_DOUBLE";
        "type" => rules "TOKEN_CHAR";
        "type" => rules "TOKEN_VOID";
        "type" => rules "TOKEN_ARRAY" "TOKEN_LBRACKET" "expr" "TOKEN_RBRACKET" "type";
        "type" => rules "TOKEN_CARRAY" "TOKEN_LBRACKET" "expr" "TOKEN_RBRACKET" "type";
        "type" => rules "TOKEN_ARRAY" "TOKEN_LBRACKET" "TOKEN_RBRACKET" "type";
        "type" => rules "TOKEN_CARRAY" "TOKEN_LBRACKET" "TOKEN_RBRACKET" "type";
        "type" => rules "TOKEN_FUNCTION" "type" "TOKEN_LPAREN" "param_opt" "TOKEN_RPAREN";

        "param_opt" => rules "param_list";
        "param_opt" => empty;

        "param_list" => rules "ident" "TOKEN_COLON" "type" "TOKEN_COMMA" "param_list";
        "param_list" => rules "ident" "TOKEN_COLON" "type";

        "ident" => rules "TOKEN_IDENTIFIER";

        "expr" => rules "expr_or";
        "expr" => rules "expr_or" "TOKEN_EQUALS" "expr";

        "expr_or" => rules "expr_and";
        "expr_or" => rules "expr_or" "TOKEN_OR" "expr_and";

        "expr_and" => rules "expr_comparison";
        "expr_and" => rules "expr_and" "TOKEN_AND" "expr_comparison";

        "expr_comparison" => rules "expr_pm";
        "expr_comparison" => rules "expr_comparison" "TOKEN_EQUIVALENT" "expr_pm";
        "expr_comparison" => rules "expr_comparison" "TOKEN_NEQ" "expr_pm";
        "expr_comparison" => rules "expr_comparison" "TOKEN_GREATERTHAN" "expr_pm";
        "expr_comparison" => rules "expr_comparison" "TOKEN_GEQ" "expr_pm";
        "expr_comparison" => rules "expr_comparison" "TOKEN_LESSTHAN" "expr_pm";
        "expr_comparison" => rules "expr_comparison" "TOKEN_LEQ" "expr_pm";

        "expr_pm" => rules "expr_td";
        "expr_pm" => rules "expr_pm" "TOKEN_PLUS" "expr_td";
        "expr_pm" => rules "expr_pm" "TOKEN_MINUS" "expr_td";

        "expr_td" => rules "expr_exp";
        "expr_td" => rules "expr_td" "TOKEN_TIMES" "expr_exp";
        "expr_td" => rules "expr_td" "TOKEN_DIVIDE" "expr_exp";
        "expr_td" => rules "expr_td" "TOKEN_MOD" "expr_exp";

        "expr_exp" => rules "expr_negate";
        "expr_exp" => rules "expr_negate" "TOKEN_CARET" "expr_exp";

        "expr_negate" => rules "expr_end";
        "expr_negate" => rules "TOKEN_UNARY" "expr_end";
        "expr_negate" => rules "TOKEN_NOT" "expr_end";
        "expr_negate" => rules "TOKEN_MINUS" "expr_end";

        "expr_end" => rules "expr_end" "TOKEN_INCREMENT";
        "expr_end" => rules "expr_end" "TOKEN_DECREMENT";
        "expr_end" => rules "literals";
        "expr_end" => rules "ident";
        "expr_end" => rules "ident" "TOKEN_LPAREN" "expr_list_opt" "TOKEN_RPAREN";
        "expr_end" => rules "TOKEN_LPAREN" "expr" "TOKEN_RPAREN";
        "expr_end" => rules "expr_end" "TOKEN_LBRACKET" "expr" "TOKEN_RBRACKET";

        "expr_list" => rules "expr" "TOKEN_COMMA" "expr_list";
        "expr_list" => rules "expr";

        "expr_list_opt" => rules "expr_list";
        "expr_list_opt" => empty;

        "expr_opt" => rules "expr";
        "expr_opt" => empty;

        "expr_braced" => rules "TOKEN_LBRACE" "expr_braced" "TOKEN_RBRACE" "TOKEN_COMMA" "expr_braced";
        "expr_braced" => rules "TOKEN_LBRACE" "expr_braced" "TOKEN_RBRACE";
        "expr_braced" => rules "expr_list";

        "stmt" => rules "unmatched_stmt";
        "stmt" => rules "matched_stmt";

        "stmt_brace" => rules "TOKEN_LBRACE" "stmt_cont" "TOKEN_RBRACE";

        "stmt_cont" => rules "stmt" "stmt_cont";
        "stmt_cont" => empty;

        "unmatched_stmt" => rules "TOKEN_IF" "TOKEN_LPAREN" "expr" "TOKEN_RPAREN" "stmt";
        "unmatched_stmt" => rules "TOKEN_IF" "TOKEN_LPAREN" "expr" "TOKEN_RPAREN" "matched_stmt" "TOKEN_ELSE" "unmatched_stmt";
        "unmatched_stmt" => rules "TOKEN_FOR" "TOKEN_LPAREN" "expr_opt" "TOKEN_SEMICOLON" "expr_opt" "TOKEN_SEMICOLON" "expr_opt" "TOKEN_RPAREN" "unmatched_stmt";

        "matched_stmt" => rules "TOKEN_FOR" "TOKEN_LPAREN" "expr_opt" "TOKEN_SEMICOLON" "expr_opt" "TOKEN_SEMICOLON" "expr_opt" "TOKEN_RPAREN" "matched_stmt";
        "matched_stmt" => rules "TOKEN_IF" "TOKEN_LPAREN" "expr" "TOKEN_RPAREN" "matched_stmt" "TOKEN_ELSE" "matched_stmt";
        "matched_stmt" => rules "stmt_other";

        "stmt_other" => rules "expr" "TOKEN_SEMICOLON";
        "stmt_other" => rules "TOKEN_RETURN" "expr_opt" "TOKEN_SEMICOLON";
        "stmt_other" => rules "TOKEN_PRINT" "expr_list_opt" "TOKEN_SEMICOLON";
        "stmt_other" => rules "decl";
        "stmt_other" => rules "stmt_brace";


        "TOKEN_ARRAY" => lexemes "TOKEN_ARRAY";
        "TOKEN_AUTO" => lexemes "TOKEN_AUTO";
        "TOKEN_BOOLEAN" => lexemes "TOKEN_BOOLEAN";
        "TOKEN_CARRAY" => lexemes "TOKEN_CARRAY";
        "TOKEN_CHAR" => lexemes "TOKEN_CHAR";
        "TOKEN_DOUBLE" => lexemes "TOKEN_DOUBLE";
        "TOKEN_ELSE" => lexemes "TOKEN_ELSE";
        "TOKEN_FALSE" => lexemes "TOKEN_FALSE";
        "TOKEN_FLOAT" => lexemes "TOKEN_FLOAT";
        "TOKEN_FOR" => lexemes "TOKEN_FOR";
        "TOKEN_FUNCTION" => lexemes "TOKEN_FUNCTION";
        "TOKEN_IF" => lexemes "TOKEN_IF";
        "TOKEN_INTEGER" => lexemes "TOKEN_INTEGER";
        "TOKEN_PRINT" => lexemes "TOKEN_PRINT";
        "TOKEN_RETURN" => lexemes "TOKEN_RETURN";
        "TOKEN_STRING" => lexemes "TOKEN_STRING";
        "TOKEN_TRUE" => lexemes "TOKEN_TRUE";
        "TOKEN_VOID" => lexemes "TOKEN_VOID";
        "TOKEN_WHILE" => lexemes "TOKEN_WHILE";
        "TOKEN_LPAREN" => lexemes "TOKEN_LPAREN";
        "TOKEN_RPAREN" => lexemes "TOKEN_RPAREN";
        "TOKEN_LBRACKET" => lexemes "TOKEN_LBRACKET";
        "TOKEN_RBRACKET" => lexemes "TOKEN_RBRACKET";
        "TOKEN_INCREMENT" => lexemes "TOKEN_INCREMENT";
        "TOKEN_DECREMENT" => lexemes "TOKEN_DECREMENT";
        "TOKEN_UNARY" => lexemes "TOKEN_UNARY";
        "TOKEN_PLUS" => lexemes "TOKEN_PLUS";
        "TOKEN_MINUS" => lexemes "TOKEN_MINUS";
        "TOKEN_DIVIDE" => lexemes "TOKEN_DIVIDE";
        "TOKEN_TIMES" => lexemes "TOKEN_TIMES";
        "TOKEN_MOD" => lexemes "TOKEN_MOD";
        "TOKEN_SEMICOLON" => lexemes "TOKEN_SEMICOLON";
        "TOKEN_COLON" => lexemes "TOKEN_COLON";
        "TOKEN_COMMA" => lexemes "TOKEN_COMMA";
        "TOKEN_NOT" => lexemes "TOKEN_NOT";
        "TOKEN_LBRACE" => lexemes "TOKEN_LBRACE";
        "TOKEN_RBRACE" => lexemes "TOKEN_RBRACE";
        "TOKEN_CARET" => lexemes "TOKEN_CARET";
        "TOKEN_LESSTHAN" => lexemes "TOKEN_LESSTHAN";
        "TOKEN_GREATERTHAN" => lexemes "TOKEN_GREATERTHAN";
        "TOKEN_EQUALS" => lexemes "TOKEN_EQUALS";
        "TOKEN_LEQ" => lexemes "TOKEN_LEQ";
        "TOKEN_GEQ" => lexemes "TOKEN_GEQ";
        "TOKEN_NEQ" => lexemes "TOKEN_NEQ";
        "TOKEN_EQUIVALENT" => lexemes "TOKEN_EQUIVALENT";
        "TOKEN_OR" => lexemes "TOKEN_OR";
        "TOKEN_AND" => lexemes "TOKEN_AND";
        "TOKEN_IDENTIFIER" => lexemes "TOKEN_IDENTIFIER";
        "TOKEN_INTEGER_LITERAL" => lexemes "TOKEN_INTEGER_LITERAL";
        "TOKEN_DOUBLE_LITERAL" => lexemes "TOKEN_DOUBLE_LITERAL";
        "TOKEN_STRING_LITERAL" => lexemes "TOKEN_STRING_LITERAL";
        "TOKEN_CHAR_LITERAL" => lexemes "TOKEN_CHAR_LITERAL";
        "TOKEN_ERROR" => lexemes "TOKEN_ERROR";
    )
}

pub fn parse(contents: &String, verbose: bool) -> i32 {
    if scan(contents, false) == 1 {
        return 1;
    }

    let lex_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lex_rules, &contents).unwrap();
    
    let grammar = grammar();
    let parse_trees = santiago::parser::parse(&grammar, &lexemes);

    match parse_trees {
        Err(_) => {
            if verbose {println!("Parse Failed!")}
            return 1;
        },

        _ => {
            if verbose {println!("Parse successful!")}
        }
    }
    return 0;
}