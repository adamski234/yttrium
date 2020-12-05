#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;

pub fn safe_create() -> Box<dyn key_base::Key + Send + Sync> {
	return Box::new(std_selfdelete {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("selfdelete"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_selfdelete {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

unsafe impl Send for std_selfdelete {}
unsafe impl Sync for std_selfdelete {}

impl key_base::Key for std_selfdelete {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
	match humantime::parse_duration(&parameter[0]) {
		Ok(duration) => {
			environment.delete_option = Some(duration);
			return Ok(String::new());
		}
		Err(error) => {
			return Err(format!("Error in time value: `{}`", error));
		}
	}
}