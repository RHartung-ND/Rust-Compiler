use crate::data_type::Data_type;

#[warn(non_camel_case_types)]
pub struct Param_list {
	name: String,
	Data_type: Data_type,
	next: Option<Box<Param_list>>
}

pub fn param_list_create( name: String, Data_type: Data_type, next: Option<Box<Param_list>> ) -> Param_list {
	let a = Param_list {
		name: name,
		Data_type: Data_type,
		next: next,
	};

	return a;
}

// void param_list_print( struct param_list *a ){
//     if(!a){
//         return;
//     }

// 	printf("%s: ", a->name);
// 	type_print(a->type);
// 	if (a->next){
// 		printf(", ");
// 		param_list_print(a->next);
// 	}
// }