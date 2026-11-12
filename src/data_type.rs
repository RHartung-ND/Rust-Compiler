use crate::ast::{Type, TypeKind};
use crate::expr::expr_print;
use crate::param_list::param_list_print;

pub fn type_print( t: &Box<Type> ) {
    match t.kind {
		TypeKind::TypeString => {
			print!("string");
        },
		TypeKind::TypeBoolean => {
            print!("boolean");
        },
        TypeKind::TypeAuto => {
            print!("auto");
        },
        TypeKind::TypeInteger => {
            print!("integer");
        },
		TypeKind::TypeDouble => {
            print!("double");
        },
        TypeKind::TypeChar => {
            print!("char");
        },
		TypeKind::TypeVoid => {
            print!("void");
        },
        TypeKind::TypeArray => {
            print!("array [");
            expr_print(&t.size.as_ref().expect("Must have Expr"));
            print!("] ");
            type_print(&t.subtype.as_ref().expect("Function type must have a subtype"));
        },
        TypeKind::TypeCArray => {
            print!("carray [");
            expr_print(&t.size.as_ref().expect("Must have Expr"));
            print!("] ");
            type_print(&t.subtype.as_ref().expect("Function type must have a subtype"));
        },
        TypeKind::TypeFunction => {
            print!("function ");
            type_print(&t.subtype.as_ref().expect("Function type must have a subtype"));
            if t.params.is_some() {
                print!(" ( ");
                param_list_print(&t.params.as_ref().expect("Function type must have a params"));
                print!(" )");
            } else {
                print!(" ()");
            }
        }
    }
}
