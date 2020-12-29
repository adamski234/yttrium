#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn safe_create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_text {
		info: create_key_info(),
		function: key_function,
	});
}

/*
Parameters:
Required, operation to perform. Possible values: equals, contains, starts_with, ends_with, regex, glob, extract
Required, text to check
Required, text to check against. Regex if parameter 0 is `regex`, glob if `glob`, etc.
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("text"),
		parameters_required: vec![3],
	};
}

#[allow(non_camel_case_types)]
struct std_text<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_text<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_text<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_text<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], _environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	let result;
	match parameter[0].as_str() {
		"equals" => {
			result = parameter[1] == parameter[2];
		}
		"contains" => {
			result = parameter[1].contains(&parameter[2]);
		}
		"starts_with" => {
			result = parameter[1].starts_with(&parameter[2]);
		}
		"ends_with" => {
			result = parameter[1].ends_with(&parameter[2]);
		}
		"regex" => {
			let matcher;
			match regex::Regex::new(&parameter[2]) {
				Ok(value) => {
					matcher = value;
				}
				Err(error) => {
					return Err(format!("Invalid regex in `text`: `{}`", error));
				}
			}
			result = matcher.is_match(&parameter[1]);
		}
		"extract" => {
			let matcher;
			match regex::Regex::new(&parameter[2]) {
				Ok(value) => {
					matcher = value;
				}
				Err(error) => {
					return Err(format!("Invalid regex in `text`: `{}`", error));
				}
			}
			match matcher.captures(&parameter[1]) {
				Some(result) => {
					return Ok(String::from(&result[1]));
				}
				None => {
					return Ok(String::new());
				}
			}
		}
		"glob" => {
			let matcher;
			match glob::Pattern::new(&parameter[2]) {
				Ok(value) => {
					matcher = value;
				}
				Err(error) => {
					return Err(format!("Invalid glob in `text`: `{}`", error));
				}
			}
			result = matcher.matches(&parameter[1]);
		}
		_ => {
			result = false;
		}
	}
	return Ok(String::from(if result { "1" } else { "0" }));
}