#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_db_read {
		info: create_key_info(),
		function: key_function,
	});
}


fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("db_read"),
		parameters_required: vec![2],
	};
}

#[allow(non_camel_case_types)]
struct std_db_read {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_db_read {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
	match environment.database_manager.get_database(&parameter[0]) {
		Some(result) => {
			match result.get_key(&parameter[1]) {
				Some(value) => {
					match value {
						key_base::databases::StringOrArray::String(string) => {
							return Ok(string);
						}
						key_base::databases::StringOrArray::Array(array) => {
							return Ok(array.join(""));
						}
					}
				}
				None => {
					return Ok(String::new());
				}
			}
		}
		None => {
			return Ok(String::new());
		}
	}
}