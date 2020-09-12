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

fn cond(parameter: &Vec<String>, environment: &key_base::environment::Environment) -> String {
	println!("placeholder");
	return String::from("test");
}

struct Key {
	key_info: key_base::KeyInfo,
	function: fn(&Vec<String>, &key_base::environment::Environment) -> String
}

impl key_base::Key for Key {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.key_info;
	}

	fn get_key_function(&self) -> fn(&Vec<String>, &key_base::environment::Environment) -> String {
		return self.function;
	}
}