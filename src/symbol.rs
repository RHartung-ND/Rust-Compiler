use crate::Data_type::Data_type;
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

pub enum symbol_t {
	SYMBOL_LOCAL,
	SYMBOL_PARAM,
	SYMBOL_GLOBAL
}

pub struct Symbol {
	kind: symbol_t,
	Data_type: Data_type,
	name: String,
}

pub fn symbol_create( kind: symbol_t, Data_type: Data_type, name: String ) -> Symbol{
	let d = Symbol {
		kind: kind,
		Data_type: Data_type,
		name: name,
	};

	return d;
}