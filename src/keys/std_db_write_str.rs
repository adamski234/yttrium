#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_db_write_str {
		info: create_key_info(),
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
}

unsafe impl Send for std_db_write_str {}
unsafe impl Sync for std_db_write_str {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_db_write_str {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn run_key(&self, parameter: &[String], environment: &mut Environment<Manager, DB>) -> Result<String, String> {
		let mut db = environment.database_manager.get_database(&parameter[0]);
 	   db.write_key(parameter[1].clone(), key_base::databases::StringOrArray::String(parameter[2].clone()));
		return Ok(String::new());
	}
}