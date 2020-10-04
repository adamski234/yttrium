#![allow(non_camel_case_types)]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("db_read"),
		parameters_required: vec![2],
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
	match environment.database_manager.get_database(&parameter[0]) {
		Some(result) => {
			match result.get_key(&parameter[1]) {
				Some(value) => {
					match value {
						key_base::databases::StringOrArray::String(string) => {
							return string;
						}
						key_base::databases::StringOrArray::Array(array) => {
							return array.join("");
						}
					}
				}
				None => {
					return String::new();
				}
			}
		}
		None => {
			return String::new();
		}
	}
}