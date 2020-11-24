#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_everyone {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
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
struct std_everyone {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_everyone {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(parameter: &[String], _environment: &mut key_base::environment::Environment) -> Result<String, String> {
	if !parameter.is_empty() && parameter[0] == "here" {
		return Ok(String::from("@here"));
	} else {
		return Ok(String::from("@everyone"));
	}
}