#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_everyone {
		info: create_key_info(),
		function: key_function,
	});
}

/*
Parameters:
Optional, set to "here" to send a @here instead of an @everyone
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("everyone"),
		parameters_required: vec![0, 1],
	};
}

#[allow(non_camel_case_types)]
struct std_everyone<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_everyone<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_everyone<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_everyone<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], _environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	if !parameter.is_empty() && parameter[0] == "here" {
		return Ok(String::from("@here"));
	} else {
		return Ok(String::from("@everyone"));
	}
}