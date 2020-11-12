#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	/*
	Parameters:
	Required, first string
	Required, second string
	*/
	let key_info = key_base::KeyInfo {
		name: String::from("text_equals"),
		parameters_required: vec![2],
	};
	return Box::into_raw(Box::new(std_text_equals {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_text_equals {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_text_equals {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], _environment: &mut key_base::environment::Environment) -> String {
	return String::from(if parameter[0] == parameter[1] { "1" } else { "0" });
}