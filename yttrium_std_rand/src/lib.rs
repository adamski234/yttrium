#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use rand::Rng;
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_rand {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_rand {
		info: create_key_info(),
		function: key_function,
	});
}

/*
Parameters:
Optional, lowest value, default 0
Optional, highest value, default 10
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("rand"),
		parameters_required: vec![0, 1, 2],
	};
}

#[allow(non_camel_case_types)]
struct std_rand {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_rand {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(parameter: &[String], _environment: &mut key_base::environment::Environment) -> Result<String, String> {
	let lower = if !parameter.is_empty() { parameter[0].parse().unwrap() } else { 0 };
	let upper = if parameter.len() == 2 { parameter[1].parse().unwrap() } else { 10 };
	if lower > upper {
		return Err(String::from("Lower bound was higher than upper bound in `rand`"));
	}
	let result = rand::thread_rng().gen_range(lower, upper);
	return Ok(result.to_string());
}