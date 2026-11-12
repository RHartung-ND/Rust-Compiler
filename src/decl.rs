use crate::print::print_indent;
use crate::data_type::type_print;
use crate::expr::expr_print;
use crate::stmt::stmt_print;
use crate::ast::{Decl, ExprKind};


pub fn decl_print( d: &Box<Decl>, num_spaces: i8 ){
	print_indent(num_spaces);

    print!("{}: ", d.name);
    
    type_print(&Box::new(d.decl_type.clone()).clone());
	
    if d.value.is_some() {
        print!(" = ");
        if d.value.as_ref().unwrap().kind == ExprKind::ExprList {
            print!("{{");
            expr_print(&d.value.as_ref().expect("Must have Expr"));
            print!("}};");
        } else {
            expr_print(&d.value.as_ref().expect("Must have Expr"));
            print!(";");
        }
    } else if d.code.is_some() {
        print!(" = ");
        stmt_print(&d.code.as_ref().expect("Must have Stmt"), num_spaces);
    } else {
        print!(";");
    }
    print!("\n");

    if let Some(next) = &d.next {
        decl_print(next, num_spaces);
    }
}
