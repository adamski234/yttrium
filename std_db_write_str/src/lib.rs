#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use key_base;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("db_write_str"),
		parameters_required: vec![3],
	};
	return Box::into_raw(Box::new(std_db_read {
		info: key_info,
		function: key_function,
	}));
}

struct std_db_read {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_db_read {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
	if let Some(result) = environment.database_manager.get_database(&parameter[0]) {
        result.write_key(parameter[1].clone(), key_base::databases::StringOrArray::String(parameter[2].clone()));
	}
	return String::new();
}