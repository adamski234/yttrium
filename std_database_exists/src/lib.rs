#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_database_exists {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_database_exists {
		info: create_key_info(),
		function: key_function,
	});
}

/*
Parameters:
Required, what to check for, valid parameter: db, key
Required, database name
Required if the first parameter is `key`, the key to check for in database
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("database_exists"),
		parameters_required: vec![2, 3],
	};
}
#[allow(non_camel_case_types)]
struct std_database_exists {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_database_exists {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	let db = environment.database_manager.get_database(&parameter[1]);
	match db {
		Some(db) => {
			match parameter[0].as_str() {
				"db" => {
					return String::from("1");
				}
				"key" => {
					return String::from(if db.key_exists(&parameter[2]) { "1" } else { "0" });
				}
				_ => {
					return String::from("0");
				}
			}
		}
		None => {
			return String::from("0");
		}
	}
}