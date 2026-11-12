use crate::ast::{Stmt, StmtKind};
use crate::print::print_indent;
use crate::expr::expr_print;
use crate::decl::decl_print;

/// Prints a statement and everything linked after it via `next`.
pub fn stmt_print(s: &Stmt, indent: i8) {
    let mut cur = Some(s);
    while let Some(stmt) = cur {
        stmt_print_one(stmt, indent, false);
        cur = stmt.next.as_deref();
    }
}

/// Prints a single statement (ignores `next`).
fn stmt_print_one(s: &Stmt, indent: i8, is_else_if: bool) {
    match s.kind {
        StmtKind::StmtDecl => {
            decl_print(s.decl.as_ref().expect("Must have Decl"), indent);
        }
        StmtKind::StmtExpr => {
            print_indent(indent);
            expr_print(s.condition.as_ref().expect("Must have Expr"));
            print!(";\n");
        }
        StmtKind::StmtIfElse => {
            if !is_else_if {
                print_indent(indent);
            }
            print!("if (");
            expr_print(s.condition.as_ref().expect("Must have Expr"));
            print!(") ");
            print_as_block(s.body.as_ref(), indent);

            if let Some(else_body) = s.else_body.as_ref() {
                print!(" else ");
                if else_body.kind == StmtKind::StmtIfElse {
                    stmt_print_one(else_body, indent, true);
                } else {
                    print_as_block(Some(else_body), indent);
                }
            }
            if !is_else_if {
                println!();
            }
        }
        StmtKind::StmtFor => {
            print_indent(indent);
            print!("for (");
            if s.init.is_some() {
                expr_print(s.init.as_ref().expect("Must have Expr"));
            }
            print!(";");
            if s.condition.is_some() {
                print!(" ");
                expr_print(s.condition.as_ref().expect("Must have Expr"));
            }
            print!(";");
            if s.update.is_some() {
                print!(" ");
                expr_print(s.update.as_ref().expect("Must have Expr"));
            }
            print!(") ");
            print_as_block(s.body.as_ref(), indent);
            println!();
        }
        StmtKind::StmtPrint | StmtKind::StmtReturn => {
            print_indent(indent);
            print!("{}", if s.kind == StmtKind::StmtPrint { "print" } else { "return" });
            if let Some(e) = s.condition.as_ref() {
                print!(" ");
                expr_print(e);
            }
            println!(";");
        }
        StmtKind::StmtBlock => {
            // Collapse a block whose only content is another block.
            if let Some(inner) = s.body.as_deref() {
                if inner.kind == StmtKind::StmtBlock {
                    stmt_print_one(inner, indent, false);
                    return;
                }
            }
            println!("{{");
            if let Some(body) = s.body.as_deref() {
                stmt_print(body, indent + 1);
            }
            print_indent(indent);
            print!("}}");
        }
    }
}

/// Prints `s` wrapped in braces unless it's already a block.
/// Replaces `stmt_wrap`, so no cloning or mutation is needed.
fn print_as_block(s: Option<&Box<Stmt>>, indent: i8) {
    match s {
        Some(stmt) if stmt.kind == StmtKind::StmtBlock => stmt_print_one(stmt, indent, false),
        Some(stmt) => {
            println!("{{");
            stmt_print(stmt, indent + 1);
            print_indent(indent);
            print!("}}");
        }
        None => print!("{{}}"),
    }
}
