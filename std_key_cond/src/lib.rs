use key_base;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(Key {
		key_info: key_base::KeyInfo {
			parameters_required: vec![2, 3],
			name: String::from("cond"),
			opcode: 1,
			allowed_key_names: vec![String::from("*")],
		},
		function: cond,
	}));
}

fn cond(parameter: &String) -> bool {
	println!("cock");
	return false;
}

struct Key {
	key_info: key_base::KeyInfo,
	function: fn(parameter: &String) -> bool
}

impl key_base::Key for Key {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.key_info;
	}

	fn get_key_function(&self) -> fn(parameter: &String) -> bool {
		return self.function;
	}
}