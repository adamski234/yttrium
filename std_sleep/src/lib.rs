#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("sleep"),
		parameters_required: vec![1],
	};
	return Box::into_raw(Box::new(std_sleep {
		info: key_info,
		function: key_function,
	}));
}

struct std_sleep {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_sleep {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &Vec<String>, _environment: &mut key_base::environment::Environment) -> String {
	match humantime::parse_duration(&parameter[0]) {
		Ok(result) => {
			std::thread::sleep(result);
		}
		Err(_) => {}
	}
	return String::new();
}