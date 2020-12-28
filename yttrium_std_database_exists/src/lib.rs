#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn safe_create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
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
struct std_database_exists<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_database_exists<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_database_exists<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_database_exists<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	let db = environment.database_manager.get_database(&parameter[1]);
	match db {
		Some(db) => {
			match parameter[0].as_str() {
				"db" => {
					return Ok(String::from("1"));
				}
				"key" => {
					return Ok(String::from(if db.key_exists(&parameter[2]) { "1" } else { "0" }));
				}
				_ => {
					return Ok(String::from("0"));
				}
			}
		}
		None => {
			return Ok(String::from("0"));
		}
	}
}