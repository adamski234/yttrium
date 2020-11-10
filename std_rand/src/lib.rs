#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use rand::Rng;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	/*
	Parameters:
	Optional, lowest value, default 0
	Optional, highest value, default 10
	*/
	let key_info = key_base::KeyInfo {
		name: String::from("rand"),
		parameters_required: vec![0, 1, 2],
	};
	return Box::into_raw(Box::new(std_mention {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_mention {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_mention {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], _environment: &mut key_base::environment::Environment) -> String {
	let lower = if !parameter.is_empty() { parameter[0].parse().unwrap() } else { 0 };
	let upper = if parameter.len() == 2 { parameter[1].parse().unwrap() } else { 10 };
	let result = rand::thread_rng().gen_range(lower, upper);
	return result.to_string();
}