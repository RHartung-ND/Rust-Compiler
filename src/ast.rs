use std::fmt;

// ============================================================================
// Expression Types
// ============================================================================

#[derive(Debug, Clone)]
pub enum ExprKind {
    IntegerLiteral(i64),
    DoubleLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    BooleanLiteral(bool),
    Name(String),
    Assign,
    Or,
    And,
    Equiv,
    Neq,
    Gt,
    Geq,
    Lt,
    Leq,
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Exp,
    Unary,
    Not,
    Negate,
    Inc,
    Dec,
    Func,
    Index,
    List,
    Brace,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub left: Option<Box<Expr>>,
    pub right: Option<Box<Expr>>,
}

impl Expr {
    pub fn empty() -> Self {
        Expr {
            kind: ExprKind::IntegerLiteral(0),
            left: None,
            right: None,
        }
    }
}

impl Default for Expr {
    fn default() -> Self {
        Self::empty()
    }
}

// ============================================================================
// Type System
// ============================================================================

#[derive(Debug, Clone)]
pub enum TypeKind {
    String,
    Boolean,
    Auto,
    Integer,
    Double,
    Char,
    Void,
    Array,
    CArray,
    Function,
}

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub subtype: Option<Box<Type>>,
    pub params: Option<Box<ParamList>>,
    pub size: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct ParamList {
    pub name: String,
    pub param_type: Type,
    pub next: Option<Box<ParamList>>,
}

// ============================================================================
// Declarations
// ============================================================================

#[derive(Debug, Clone)]
pub struct Decl {
    pub name: String,
    pub decl_type: Type,
    pub init_value: Option<Box<Expr>>,
    pub init_code: Option<Box<Stmt>>,
    pub next: Box<Decl>,
}

impl Default for Decl {
    fn default() -> Self {
        Decl {
            name: String::new(),
            decl_type: Type {
                kind: TypeKind::Void,
                subtype: None,
                params: None,
                size: None,
            },
            init_value: None,
            init_code: None,
            next: Box::new(Decl::default()),
        }
    }
}

// ============================================================================
// Statement Types
// ============================================================================

#[derive(Debug, Clone)]
pub enum StmtKind {
    Block,
    IfElse,
    For,
    Expr,
    Return,
    Print,
    Decl,
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub decl: Option<Decl>,
    pub init: Option<Box<Expr>>,
    pub condition: Option<Box<Expr>>,
    pub update: Option<Box<Expr>>,
    pub body: Box<Stmt>,
    pub else_body: Option<Box<Stmt>>,
    pub next: Box<Stmt>,
}

impl Stmt {
    pub fn empty() -> Self {
        Stmt {
            kind: StmtKind::Block,
            decl: None,
            init: None,
            condition: None,
            update: None,
            body: Box::new(Stmt::empty_inner()),
            else_body: None,
            next: Box::new(Stmt::empty_inner()),
        }
    }

    fn empty_inner() -> Self {
        Stmt {
            kind: StmtKind::Block,
            decl: None,
            init: None,
            condition: None,
            update: None,
            body: Box::new(Stmt {
                kind: StmtKind::Block,
                decl: None,
                init: None,
                condition: None,
                update: None,
                body: Box::new(Stmt::empty_inner()),
                else_body: None,
                next: Box::new(Stmt::empty_inner()),
            }),
            else_body: None,
            next: Box::new(Stmt::empty_inner()),
        }
    }
}

impl Default for Stmt {
    fn default() -> Self {
        Stmt {
            kind: StmtKind::Block,
            decl: None,
            init: None,
            condition: None,
            update: None,
            body: Box::new(Stmt::empty()),
            else_body: None,
            next: Box::new(Stmt::empty()),
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

pub fn expr_create(kind: ExprKind, left: Box<Expr>, right: Box<Expr>) -> Expr {
    Expr {
        kind,
        left: Some(left),
        right: Some(right),
    }
}

pub fn expr_create_integer_literal(value: &str) -> Expr {
    Expr {
        kind: ExprKind::IntegerLiteral(value.parse().unwrap_or(0)),
        left: None,
        right: None,
    }
}

pub fn expr_create_double_literal(value: f64) -> Expr {
    Expr {
        kind: ExprKind::DoubleLiteral(value),
        left: None,
        right: None,
    }
}

pub fn expr_create_string_literal(value: &str) -> Expr {
    Expr {
        kind: ExprKind::StringLiteral(value.to_string()),
        left: None,
        right: None,
    }
}

pub fn expr_create_char_literal(value: &str) -> Expr {
    Expr {
        kind: ExprKind::CharLiteral(value.chars().next().unwrap_or('\0')),
        left: None,
        right: None,
    }
}

pub fn expr_create_boolean_literal(value: bool) -> Expr {
    Expr {
        kind: ExprKind::BooleanLiteral(value),
        left: None,
        right: None,
    }
}

pub fn expr_create_name(name: &str) -> Expr {
    Expr {
        kind: ExprKind::Name(name.to_string()),
        left: None,
        right: None,
    }
}

pub fn type_create(
    kind: TypeKind,
    subtype: Option<Box<Type>>,
    params: Option<Box<ParamList>>,
    size: Option<Box<Expr>>,
) -> Type {
    Type {
        kind,
        subtype,
        params,
        size,
    }
}

pub fn param_list_create(
    name: &str,
    param_type: Type,
    next: Option<Box<ParamList>>,
) -> ParamList {
    ParamList {
        name: name.to_string(),
        param_type,
        next,
    }
}

pub fn decl_create(
    name: &str,
    decl_type: Type,
    init_value: Option<Box<Expr>>,
    init_code: Option<Box<Stmt>>,
    _reserved: Option<()>,
) -> Decl {
    Decl {
        name: name.to_string(),
        decl_type,
        init_value,
        init_code,
        next: Box::new(Decl::default()),
    }
}

pub fn stmt_create(
    kind: StmtKind,
    decl: Option<Decl>,
    init: Option<Box<Expr>>,
    condition: Option<Box<Expr>>,
    update: Option<Box<Expr>>,
    body: Box<Stmt>,
    else_body: Option<Box<Stmt>>,
    _reserved: Option<()>,
) -> Stmt {
    Stmt {
        kind,
        decl,
        init,
        condition,
        update,
        body,
        else_body,
        next: Box::new(Stmt::default()),
    }
}