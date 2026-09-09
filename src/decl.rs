use crate::expr::Expr;
// use crate::expr::expr_print;
// use crate::expr::expr_t;
use crate::stmt::Stmt;
use crate::data_type::Data_type;
// use crate::print::print_indent;

#[allow(non_snake_case)]
pub struct Decl {
    pub name:  String,
    pub Data_type:  Data_type,
    pub value: Expr,
    pub code:  Stmt,
    pub next:  Option<Box<Decl>>
}

pub fn decl_create( name: String, Data_type: Data_type, value: Expr, code: Stmt, next: Option<Box<Decl>> ) -> Decl {
    let d = Decl {
        name:name,
        Data_type:Data_type,
        value:value,
        code:code,
        next:next
    };

	return d;
}

// pub fn decl_print( d: Decl, num_spaces: i8 ){
//     if d == 0 {
//         return;
//     }

// 	print_indent(num_spaces);

//     print!("%s: ", d.name);
    
//     type_print(d.Type);
	
//     if (d.value) {
//         print!(" = ");
//         if d.value.kind == expr_t::EXPR_LIST {
//             print!("{{");
//             expr_print(d.value);
//             print!("}");
//         } else {
//             expr_print(d.value);
//             print!(";");
//         }
//     } else if (d.code) {
//         print!(" = ");
//         stmt_print(d.code, num_spaces);
//     } else {
//         print!(";");
//     }
//     print!("\n");

//     decl_print(d.next, num_spaces);
// }