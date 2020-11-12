#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	/*
	Parameters:
	Required: chooses the pattern format, either glob or regex
	Required: Pattern
	Required: String to match against
	*/
	let key_info = key_base::KeyInfo {
		name: String::from("match"),
		parameters_required: vec![3],
	};
	return Box::into_raw(Box::new(std_match {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_match {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_match {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], _environment: &mut key_base::environment::Environment) -> String {
	match parameter[0].as_str() {
		"regex" => {
			let matcher;
			match regex::Regex::new(&parameter[1]) {
				Ok(value) => {
					matcher = value;
				}
				Err(_) => {
					return String::from("0");
				}
			}
			if matcher.is_match(&parameter[2]) {
				return String::from("1");
			} else {
				return String::from("0");
			}
		}
		"glob" => {
			let matcher;
			match glob::Pattern::new(&parameter[1]) {
				Ok(value) => {
					matcher = value;
				}
				Err(_) => {
					return String::from("0");
				}
			}
			if matcher.matches(&parameter[2]) {
				return String::from("1");
			} else {
				return String::from("0");
			}
		}
		_ => {
			return String::new();
		}
	}
	return String::from("0");
}