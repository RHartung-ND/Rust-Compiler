use crate::data_type::Data_type;

#[warn(non_camel_case_types)]
pub struct Param_list {
	name: String,
	Data_type: Option<Box<Data_type>>,
	next: Option<Box<Param_list>>
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