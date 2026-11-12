// ============================================================================
// Expression Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
	ExprPlus,
	ExprMinus,
	ExprNegate,
	ExprTimes,
	ExprDivide,
	ExprMod,
	ExprExp,
	ExprName(String),
	ExprIntLiteral(String),
	ExprBoolLiteral(bool),
	ExprCharLiteral(char),
	ExprStringLiteral(String),
	ExprDoubleLiteral(f64),
	ExprFunc,
	ExprInc,
	ExprDec,
	ExprIdx,
	ExprAssign,
	ExprOr,
	ExprAnd,
	ExprEquiv,
	ExprNeq,
	ExprGt,
	ExprGeq,
	ExprLt,
	ExprLeq,
	ExprUnary,
	ExprNot,
	ExprBrace,
	ExprList
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub left: Option<Box<Expr>>,
    pub right: Option<Box<Expr>>,
}

// ============================================================================
// Type System
// ============================================================================

#[derive(Debug, Clone)]
pub enum TypeKind {
	TypeString,
	TypeBoolean,
	TypeAuto,
	TypeInteger,
	TypeDouble,
	TypeChar,
	TypeVoid,
	TypeArray,
	TypeCArray,
	TypeFunction
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
    pub value: Option<Box<Expr>>,
    pub code: Option<Box<Stmt>>,
    pub next: Option<Box<Decl>>,
}


// ============================================================================
// Statement Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
	StmtDecl,
	StmtExpr,
	StmtIfElse,
	StmtFor,
	StmtPrint,
	StmtReturn,
	StmtBlock
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub decl: Option<Box<Decl>>,
    pub init: Option<Box<Expr>>,      // init_expr
    pub condition: Option<Box<Expr>>, // expr
    pub update: Option<Box<Expr>>,    // next_expr
    pub body: Option<Box<Stmt>>,
    pub else_body: Option<Box<Stmt>>,
    pub next: Option<Box<Stmt>>,
}

// ============================================================================
// Helper Functions
// ============================================================================

pub fn expr_create(
    kind: ExprKind,
    left: Box<Expr>,
    right: impl Into<Option<Box<Expr>>>,
) -> Expr {
    Expr {
        kind,
        left: Some(left),
        right: right.into(),
    }
}

pub fn expr_create_integer_literal(value: &str) -> Expr {
    Expr {
        kind: ExprKind::ExprIntLiteral(value.parse().unwrap()),
        left: None,
        right: None,
    }
}

pub fn expr_create_double_literal(value: f64) -> Expr {
    Expr {
        kind: ExprKind::ExprDoubleLiteral(value),
        left: None,
        right: None,
    }
}

pub fn expr_create_string_literal(value: &str) -> Expr {
    Expr {
        kind: ExprKind::ExprStringLiteral(value.to_string()),
        left: None,
        right: None,
    }
}

pub fn expr_create_char_literal(text: &str) -> Expr {
    let b = text.as_bytes();
    let c: u8 = if b[1] == b'\\' {
        if b[2] == b'0' {
            if b.get(3) == Some(&b'x') {
                u8::from_str_radix(&text[4..6], 16).unwrap_or(0)
            } else {
                0
            }
        } else {
            b[2]
        }
    } else {
        b[1]
    };

    Expr {
        kind: ExprKind::ExprCharLiteral(c as char),
        left: None,
        right: None,
    }
}

pub fn expr_create_boolean_literal(value: bool) -> Expr {
    Expr {
        kind: ExprKind::ExprBoolLiteral(value),
        left: None,
        right: None,
    }
}

pub fn expr_create_name(name: &str) -> Expr {
    Expr {
        kind: ExprKind::ExprName(name.to_string()),
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
    value: Option<Box<Expr>>,
    code: Option<Box<Stmt>>,
    _reserved: Option<()>,
) -> Decl {
    Decl {
        name: name.to_string(),
        decl_type,
        value,
        code,
        next: None,
    }
}

pub fn stmt_create(
    kind: StmtKind,
    decl: Option<Box<Decl>>,
    init: Option<Box<Expr>>,
    condition: Option<Box<Expr>>,
    update: Option<Box<Expr>>,
    body: Option<Box<Stmt>>,
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
        next: None,
    }
}