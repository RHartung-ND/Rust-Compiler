use crate::ast::{Expr, ExprKind};

pub fn expr_print( e: &Box<Expr> ) {
    match e.kind {
        ExprKind::ExprPlus => {
            expr_check_precedence(e, "+");
        },
        ExprKind::ExprMinus => {
            expr_check_precedence(e, "-");
        },
        ExprKind::ExprNegate => {
            print!("-");
            expr_print(&e.left.as_ref().expect("Must have a number"));
        },
        ExprKind::ExprTimes => {
            expr_check_precedence(e, "*");
        },
        ExprKind::ExprDivide => {
            expr_check_precedence(e, "/");
        },
        ExprKind::ExprMod => {
            expr_check_precedence(e, "%");
        },
        ExprKind::ExprExp => {
            expr_check_precedence(e, "^");
        },
        ExprKind::ExprName(ref name) => {
            print!("{}", name);
        },
        ExprKind::ExprIntLiteral(ref int) => {
            print!("{}", int);
        },
        ExprKind::ExprBoolLiteral(ref boolean) => {
            print!("{}", boolean);
        },
        ExprKind::ExprCharLiteral(ref character) => {
            print!("'{}'", character);
        },
        ExprKind::ExprStringLiteral(ref string) => {
            print!("{}", string);
        },
        ExprKind::ExprDoubleLiteral(ref double) => {
            print!("{}", double);
        },
        ExprKind::ExprFunc => {
            expr_print(&e.left.as_ref().expect("Check left"));
            print!("(");
            if e.right.is_some() {
                expr_print_list(&e.right.as_ref().expect("Check right"));
            }
            print!(")");
        },
        ExprKind::ExprInc => {
            expr_print(&e.left.as_ref().expect("Check left"));
            print!("++");
        },
        ExprKind::ExprDec => {
            expr_print(&e.left.as_ref().expect("Check left"));
            print!("--");
        },
        ExprKind::ExprIdx => {
            expr_print(&e.left.as_ref().expect("Check left"));
            print!("[");
            expr_print(&e.right.as_ref().expect("Check right"));
            print!("]");
        },
        ExprKind::ExprAssign => {
            expr_check_precedence(e, "=");
        },
        ExprKind::ExprOr => {
            expr_check_precedence(e, "||");
        },
        ExprKind::ExprAnd => {
            expr_check_precedence(e, "&&");
        },
        ExprKind::ExprEquiv => {
            expr_check_precedence(e, "==");
        },
        ExprKind::ExprNeq => {
            expr_check_precedence(e, "!=");
        },
        ExprKind::ExprGt => {
            expr_check_precedence(e, ">");
        },
        ExprKind::ExprGeq => {
            expr_check_precedence(e, ">=");
        },
        ExprKind::ExprLt => {
            expr_check_precedence(e, "<");
        },
        ExprKind::ExprLeq => {
            expr_check_precedence(e, "<=");
        },
        ExprKind::ExprUnary => {
            print!("#");
            expr_print(&e.left.as_ref().expect("Check left"));
        },
        ExprKind::ExprNot => {
            print!("!");
            expr_print(&e.left.as_ref().expect("Check left"));
        },
        ExprKind::ExprBrace => {
            print!("{{");
            expr_print(&e.left.as_ref().expect("Check left"));
            print!("}}");
            if e.right.is_some() {
                print!(",");
            }
            expr_print(&e.right.as_ref().expect("Check right"));
        },
        ExprKind::ExprList => {
            expr_print(&e.left.as_ref().expect("Check left"));
            if e.right.is_some() {
                print!(", ");
                expr_print(&e.right.as_ref().expect("Check right"));
            }
        }
    }
}

fn expr_precedence(e: &Box<Expr>) -> i32 {
    match e.kind {
        ExprKind::ExprAssign => {
            return 1;
        }
        ExprKind::ExprOr => {
            return 2;
        }
        ExprKind::ExprAnd => {
            return 3;
        }
        ExprKind::ExprLt => {
            return 4;
        }
        ExprKind::ExprLeq => {
            return 4;
        }
        ExprKind::ExprGt => {
            return 4;
        }
        ExprKind::ExprGeq => {
            return 4;
        }
        ExprKind::ExprEquiv => {
            return 4;
        }
        ExprKind::ExprNeq => {
            return 4;
        }
        ExprKind::ExprPlus => {
            return 5;
        }
        ExprKind::ExprMinus => {
            return 5;
        }
        ExprKind::ExprTimes => {
            return 6;
        }
        ExprKind::ExprDivide => {
            return 6;
        }
        ExprKind::ExprMod => {
            return 6;
        }
        ExprKind::ExprExp => {
            return 7;
        }
        ExprKind::ExprNegate => {
            return 8;
        }
        ExprKind::ExprInc => {
            return 9;
        }
        ExprKind::ExprDec => {
            return 9;
        }
        _ => {
            return 10;
        }
    }
}

fn expr_check_precedence(e: &Box<Expr>, c: &str ) {
    if expr_precedence(e) > expr_precedence(&e.left.as_ref().expect("Check left")) {
        print!("(");
		expr_print(&e.left.as_ref().expect("Check left"));
		print!(")");
    } else {
		expr_print(&e.left.as_ref().expect("Check left"));
    }

	print!(" {} ", c);

    if expr_precedence(e) > expr_precedence(&e.right.as_ref().expect("Check right")){
        print!("(");
        expr_print(&e.right.as_ref().expect("Check right"));
        print!(")");
    } else {
        expr_print(&e.right.as_ref().expect("Check right"));
    }
}

fn expr_print_list(e: &Box<Expr>) {
    expr_print(&e.left.as_ref().expect("Check left"));

    if e.right.is_some() {
        print!(", ");
        expr_print_list(&e.right.as_ref().expect("Check right"));
    }
}
