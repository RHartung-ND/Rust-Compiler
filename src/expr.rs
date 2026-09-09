use crate::helper_functions::unicode_to_dec;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum expr_t {
    EXPR_PLUS,
	EXPR_MINUS,
	EXPR_NEGATE,
	EXPR_TIMES,
	EXPR_DIVIDE,
	EXPR_MOD,
	EXPR_EXP,
	EXPR_NAME,
	EXPR_INT_LITERAL,
	EXPR_BOOL_LITERAL,
	EXPR_CHAR_LITERAL,
	EXPR_STRING_LITERAL,
	EXPR_DOUBLE_LITERAL,
	EXPR_FUNC,
	EXPR_INC,
	EXPR_DEC,
	EXPR_IDX,
	EXPR_ASSIGN,
	EXPR_OR,
	EXPR_AND,
	EXPR_EQUIV,
	EXPR_NEQ,
	EXPR_GT,
	EXPR_GEQ,
	EXPR_LT,
	EXPR_LEQ,
	EXPR_UNARY,
	EXPR_NOT,
	EXPR_BRACE,
	EXPR_LIST
}

pub struct Expr {
    pub kind:  expr_t,
    pub value: i32,
    pub left:  Option<Box<Expr>>,
    pub right: Option<Box<Expr>>
}

pub fn expr_create(kind:expr_t, left:Option<Box<Expr>>, right:Option<Box<Expr>>) -> Expr {
	let e = Expr {
		kind:kind,
		left:left,
		value:0,
		right:right,
	};

	return e;
}

pub fn expr_create_integer_literal( c: String ) -> Expr {
    let mut e = expr_create(expr_t::EXPR_INT_LITERAL,None,None);
    e.value = unicode_to_dec(&c);
    return e;
}
