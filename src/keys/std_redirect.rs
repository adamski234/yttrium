#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_redirect {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("redirect"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_redirect<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_redirect<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_redirect<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_redirect<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	let possibly_id = &parameter[0];
	let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
	if matcher.is_match(possibly_id) {
		environment.target = possibly_id.clone();
		return Ok(String::new());
	} else {
		return Err(String::from("Invalid ID passed to `target`"));
	};
}