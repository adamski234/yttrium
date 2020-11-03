#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("redirect"),
		parameters_required: vec![1],
	};
	return Box::into_raw(Box::new(std_redirect {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_redirect {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_redirect {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	let possibly_id = &parameter[0];
	let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
	if matcher.is_match(possibly_id) {
		environment.target = possibly_id.clone();
	};
	return String::new();
}