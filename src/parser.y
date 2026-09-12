%grmtools{
    yacckind: Original(GenericParseTree),
}
%start Program

%left TOKEN_OR
%left TOKEN_AND
%left TOKEN_EQUIVALENT TOKEN_NEQ
%left TOKEN_LESSTHAN TOKEN_GREATERTHAN TOKEN_LEQ TOKEN_GEQ
%left TOKEN_PLUS TOKEN_MINUS
%left TOKEN_TIMES TOKEN_DIVIDE TOKEN_MOD
%right TOKEN_CARET
%right TOKEN_NOT TOKEN_UNARY

%expect-unused Unused "TOKEN_WHILE"
%expect-unused Unused "TOKEN_ERROR"
%expect-unused Unused "TOKEN_UNMATCHED_COMMENT"
%expect-unused Unused "TOKEN_C_COMMENT"
%expect-unused Unused "TOKEN_CPP_COMMENT"
%%

Program: DeclList { Some($1) }
        ;

// Declarations
DeclList: Decl DeclList { let mut d = $1; d.next = Box::new($2); d }
         |                 { Default::default() }
         ;

Decl: Ident "TOKEN_COLON" Type "TOKEN_SEMICOLON" { decl_create(&$1, $3, None, None, None) }
     | Ident "TOKEN_COLON" Type "TOKEN_EQUALS" "TOKEN_LBRACE" ExprBraced "TOKEN_RBRACE" "TOKEN_SEMICOLON" { decl_create(&$1, $3, Some($6), None, None) }
     | Ident "TOKEN_COLON" Type "TOKEN_EQUALS" Expr "TOKEN_SEMICOLON" { decl_create(&$1, $3, Some($5), None, None) } 
     | Ident "TOKEN_COLON" Type "TOKEN_EQUALS" StmtBrace { decl_create(&$1, $3, None, Some($5), None) }
     ;

Literals: "TOKEN_INTEGER_LITERAL" { expr_create_integer_literal(&$1) }
    | "TOKEN_DOUBLE_LITERAL" { expr_create_double_literal($1.parse::<f64>().unwrap()) }
    | "TOKEN_STRING_LITERAL" { expr_create_string_literal(&$1) }
    | "TOKEN_CHAR_LITERAL" { expr_create_char_literal(&$1) }
    | "TOKEN_FALSE" { expr_create_boolean_literal(false) }
    | "TOKEN_TRUE" { expr_create_boolean_literal(true) }
    ;

Type: "TOKEN_STRING" { type_create(TypeKind::String, None, None, None) }
    | "TOKEN_BOOLEAN" { type_create(TypeKind::Boolean, None, None, None) }
    | "TOKEN_AUTO" { type_create(TypeKind::Auto, None, None, None) }
    | "TOKEN_INTEGER" { type_create(TypeKind::Integer, None, None, None) }
    | "TOKEN_DOUBLE" { type_create(TypeKind::Double, None, None, None) }
    | "TOKEN_CHAR" { type_create(TypeKind::Char, None, None, None) }
    | "TOKEN_VOID" { type_create(TypeKind::Void, None, None, None) }
    | "TOKEN_ARRAY" "TOKEN_LBRACKET" Expr "TOKEN_RBRACKET" Type { type_create(TypeKind::Array, Some(Box::new($5)), None, Some(Box::new($3))) }
    | "TOKEN_CARRAY" "TOKEN_LBRACKET" Expr "TOKEN_RBRACKET" Type { type_create(TypeKind::CArray, Some(Box::new($5)), None, Some(Box::new($3))) }
    | "TOKEN_ARRAY" "TOKEN_LBRACKET" "TOKEN_RBRACKET" Type { type_create(TypeKind::Array, Some(Box::new($4)), None, None) }
    | "TOKEN_CARRAY" "TOKEN_LBRACKET" "TOKEN_RBRACKET" Type { type_create(TypeKind::CArray, Some(Box::new($4)), None, None) }
    | "TOKEN_FUNCTION" Type "TOKEN_LPAREN" ParamOpt "TOKEN_RPAREN" { type_create(TypeKind::Function, Some(Box::new($2)), $4, None) }
    ;

ParamOpt: ParamList { $1 }
        | { None }
        ;

ParamList: Ident "TOKEN_COLON" Type "TOKEN_COMMA" ParamList { Some(Box::new(param_list_create(&$1, $3, $5))) } 
        | Ident "TOKEN_COLON" Type { Some(Box::new(param_list_create(&$1, $3, None))) }
        ;

// Expressions
Ident: "TOKEN_IDENTIFIER" { expr_create_name(&$1) }
      ;

Expr: ExprOr { $1 }
    | ExprOr "TOKEN_EQUALS" Expr { expr_create(ExprKind::Assign, Box::new($1), Box::new($3)) }
    ;

ExprOr: ExprAnd { $1 }
    | ExprOr "TOKEN_OR" ExprAnd { expr_create(ExprKind::Or, Box::new($1), Box::new($3)) }
    ;

ExprAnd: ExprComparison { $1 }
        | ExprAnd "TOKEN_AND" ExprComparison { expr_create(ExprKind::And, Box::new($1), Box::new($3)) }
        ;

ExprComparison: ExprPm { $1 }
        | ExprComparison "TOKEN_EQUIVALENT" ExprPm { expr_create(ExprKind::Equiv, Box::new($1), Box::new($3)) }
        | ExprComparison "TOKEN_NEQ" ExprPm { expr_create(ExprKind::Neq, Box::new($1), Box::new($3)) }
        | ExprComparison "TOKEN_GREATERTHAN" ExprPm { expr_create(ExprKind::Gt, Box::new($1), Box::new($3)) }
        | ExprComparison "TOKEN_GEQ" ExprPm { expr_create(ExprKind::Geq, Box::new($1), Box::new($3)) }
        | ExprComparison "TOKEN_LESSTHAN" ExprPm { expr_create(ExprKind::Lt, Box::new($1), Box::new($3)) }
        | ExprComparison "TOKEN_LEQ" ExprPm { expr_create(ExprKind::Leq, Box::new($1), Box::new($3)) }
        ;

ExprPm: ExprTd { $1 }
        | ExprPm "TOKEN_PLUS" ExprTd { expr_create(ExprKind::Plus, Box::new($1), Box::new($3)) }
        | ExprPm "TOKEN_MINUS" ExprTd { expr_create(ExprKind::Minus, Box::new($1), Box::new($3)) }
        ;

ExprTd: ExprExp { $1 }
        | ExprTd "TOKEN_TIMES" ExprExp { expr_create(ExprKind::Times, Box::new($1), Box::new($3)) }
        | ExprTd "TOKEN_DIVIDE" ExprExp { expr_create(ExprKind::Divide, Box::new($1), Box::new($3)) }
        | ExprTd "TOKEN_MOD" ExprExp { expr_create(ExprKind::Mod, Box::new($1), Box::new($3)) }
        ;

ExprExp: ExprNegate { $1 }
        | ExprNegate "TOKEN_CARET" ExprExp { expr_create(ExprKind::Exp, Box::new($1), Box::new($3)) }
        ;

ExprNegate: ExprEnd { $1 }
            | "TOKEN_UNARY" ExprEnd { expr_create(ExprKind::Unary, Box::new($2), Box::new(Expr::empty())) }
            | "TOKEN_NOT" ExprEnd { expr_create(ExprKind::Not, Box::new($2), Box::new(Expr::empty())) }
            | "TOKEN_MINUS" ExprEnd { expr_create(ExprKind::Negate, Box::new($2), Box::new(Expr::empty())) }
            ;

ExprEnd: ExprEnd "TOKEN_INCREMENT" { expr_create(ExprKind::Inc, Box::new($1), Box::new(Expr::empty())) }
        | ExprEnd "TOKEN_DECREMENT" { expr_create(ExprKind::Dec, Box::new($1), Box::new(Expr::empty())) }
        | Literals { $1 }
        | Ident { $1 }
        | Ident "TOKEN_LPAREN" ExprListOpt "TOKEN_RPAREN" { expr_create(ExprKind::Func, Box::new($1), Box::new($3.unwrap_or_default())) }
        | "TOKEN_LPAREN" Expr "TOKEN_RPAREN" { $2 }
        | ExprEnd "TOKEN_LBRACKET" Expr "TOKEN_RBRACKET" { expr_create(ExprKind::Index, Box::new($1), Box::new($3)) }
        ;

ExprList: Expr "TOKEN_COMMA" ExprList { expr_create(ExprKind::List, Box::new($1), Box::new($3)) }
        | Expr { expr_create(ExprKind::List, Box::new($1), Box::new(Expr::empty())) }
        ;

ExprListOpt: ExprList { Some($1) }
            | { None }
            ;

ExprOpt: Expr { Some($1) }
        | { None }
        ;

ExprBraced: "TOKEN_LBRACE" ExprBraced "TOKEN_RBRACE" "TOKEN_COMMA" ExprBraced { expr_create(ExprKind::Brace, Box::new($2), Box::new($5)) }
            | "TOKEN_LBRACE" ExprBraced "TOKEN_RBRACE" { expr_create(ExprKind::Brace, Box::new($2), Box::new(Expr::empty())) }
            | ExprList { $1 }
            ;

// Statements
Stmt: UnmatchedStmt { $1 }
    | MatchedStmt { $1 }
    ;

StmtBrace: "TOKEN_LBRACE" StmtCont "TOKEN_RBRACE" { stmt_create(StmtKind::Block, None, None, None, None, $2, None, None) }
        ;

StmtCont: Stmt StmtCont { let mut s = $1; s.next = Box::new($2); s }
    | { Default::default() }
    ;

UnmatchedStmt: "TOKEN_IF" "TOKEN_LPAREN" Expr "TOKEN_RPAREN" Stmt { stmt_create(StmtKind::IfElse, None, None, Some(Box::new($3)), None, Box::new($5), None, None) }
    | "TOKEN_IF" "TOKEN_LPAREN" Expr "TOKEN_RPAREN" MatchedStmt "TOKEN_ELSE" UnmatchedStmt { stmt_create(StmtKind::IfElse, None, None, Some(Box::new($3)), None, Box::new($5), Some(Box::new($7)), None) }
    | "TOKEN_FOR" "TOKEN_LPAREN" ExprOpt "TOKEN_SEMICOLON" ExprOpt "TOKEN_SEMICOLON" ExprOpt "TOKEN_RPAREN" UnmatchedStmt {stmt_create(StmtKind::For, None, $3.map(Box::new), $5.map(Box::new), $7.map(Box::new), Box::new($9), None, None) }
    ;

MatchedStmt: "TOKEN_FOR" "TOKEN_LPAREN" ExprOpt "TOKEN_SEMICOLON" ExprOpt "TOKEN_SEMICOLON" ExprOpt "TOKEN_RPAREN" MatchedStmt { stmt_create(StmtKind::For, None, $3.map(Box::new), $5.map(Box::new), $7.map(Box::new), Box::new($9), None, None) }
        | "TOKEN_IF" "TOKEN_LPAREN" Expr "TOKEN_RPAREN" MatchedStmt "TOKEN_ELSE" MatchedStmt { stmt_create(StmtKind::IfElse, None, None, Some(Box::new($3)), None, Box::new($5), Some(Box::new($7)), None) }
        | StmtOther { $1 }
        ;

StmtOther: Expr "TOKEN_SEMICOLON" { stmt_create(StmtKind::Expr, None, None, Some(Box::new($1)), None, Box::new(Stmt::empty()), None, None) }
        | "TOKEN_RETURN" ExprOpt "TOKEN_SEMICOLON" { stmt_create(StmtKind::Return, None, None, $2.map(Box::new), None, Box::new(Stmt::empty()), None, None) }
        | "TOKEN_PRINT" ExprListOpt "TOKEN_SEMICOLON" { stmt_create(StmtKind::Print, None, None, $2.map(Box::new), None, Box::new(Stmt::empty()), None, None) }
        | Decl { stmt_create(StmtKind::Decl, Some($1), None, None, None, Box::new(Stmt::empty()), None, None) }
        | StmtBrace { $1 }
        ;

Unused: "TOKEN_WHILE" { }
     | "TOKEN_ERROR" { }
     | "TOKEN_UNMATCHED_COMMENT" { }
     | "TOKEN_C_COMMENT" { }
     | "TOKEN_CPP_COMMENT" { }
     ;
%%
