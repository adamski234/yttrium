#![allow(clippy::needless_return)]

use yttrium_key_base as key_base;
use serenity::async_trait;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_db_read {
		info: create_key_info(),
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
}

unsafe impl Send for std_db_read {}
unsafe impl Sync for std_db_read {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_db_read {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}
	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		let db = environment.database_manager.get_database(&parameter[0]);
		match db.get_key(&parameter[1]) {
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
}