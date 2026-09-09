use crate::expr::Expr;
use crate::param_list::Param_list;#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum data_type_t {
	TYPE_STRING,
	TYPE_BOOLEAN,
	TYPE_AUTO,
	TYPE_INTEGER,
	TYPE_DOUBLE,
	TYPE_CHAR,
	TYPE_VOID,
	TYPE_ARRAY,
	TYPE_CARRAY,
	TYPE_FUNCTION
}

#[allow(non_camel_case_types)]
pub struct Data_type {
    kind: data_type_t,
    subtype: Option<Box<Data_type>>,
    params: Option<Box<Param_list>>,
    arr_type: Expr
}

pub fn type_create( kind: data_type_t, subtype: Option<Box<Data_type>>, params: Option<Box<Param_list>>, arr_type: Expr ) -> Data_type {
	let t = Data_type {
        kind: kind,
        subtype: subtype,
        params: params,
        arr_type: arr_type
    };
	return t;
}

// pub fn type_print( t: data_type ) {
//     if t {
//         return;
//     }
//     match t.kind {
// 		TYPE_STRING => {
// 			print!("string");
//         },
// 		TYPE_BOOLEAN => {
//             print!("boolean");
//         },
//         TYPE_AUTO => {
//             print!("auto");
//         },
//         TYPE_INTEGER => {
//             print!("integer");
//         },
// 		TYPE_DOUBLE => {
//             print!("double");
//         },
//         TYPE_CHAR => {
//             print!("char");
//         },
// 		TYPE_VOID => {
//             print!("void");
//         },
//         TYPE_ARRAY => {
//             print!("array [");
//             expr_print(t.arr_type);
//             print!("] ");
//             type_print(t.subtype);
//         },
//         TYPE_CARRAY => {
//             print!("carray [");
//             expr_print(t.arr_type);
//             print!("] ");
//             type_print(t.subtype);
//         },
//         TYPE_FUNCTION => {
//             print!("function ");
//             type_print(t.subtype);
//             print!(" (");
//             param_list_print(t.params);
//             print!(")");
//         },
//         _ => {
//             println!("print error: Invalid type found.");
//             exit(1);
//         }
//     }
// }
