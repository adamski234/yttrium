#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_db_write_str {
		info: create_key_info(),
		function: key_function,
	});
}


fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("db_write_str"),
		parameters_required: vec![3],
	};
}
#[allow(non_camel_case_types)]
struct std_db_write_str {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_db_write_str {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
	if let Some(result) = environment.database_manager.get_database(&parameter[0]) {
        result.write_key(parameter[1].clone(), key_base::databases::StringOrArray::String(parameter[2].clone()));
	}
	return Ok(String::new());
}