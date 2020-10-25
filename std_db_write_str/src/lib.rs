#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("db_write_str"),
		parameters_required: vec![3],
	};
	return Box::into_raw(Box::new(std_db_write_str {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_db_write_str {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_db_write_str {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	if let Some(result) = environment.database_manager.get_database(&parameter[0]) {
        result.write_key(parameter[1].clone(), key_base::databases::StringOrArray::String(parameter[2].clone()));
	}
	return String::new();
}