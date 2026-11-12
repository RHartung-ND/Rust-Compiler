use crate::data_type::type_print;
use crate::ast::ParamList;

pub fn param_list_print(a: &Box<ParamList>) {
    print!("{}: ", a.name);
    type_print(&Box::new(a.param_type.clone()).clone());
    if a.next.is_some() {
        print!(", ");
        param_list_print(&a.next.as_ref().expect("Must have ParamList"));
    }
}