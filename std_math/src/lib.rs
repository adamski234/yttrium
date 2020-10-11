#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("math"),
		parameters_required: vec![1],
	};
	return Box::into_raw(Box::new(std_math {
		info: key_info,
		function: key_function,
	}));
}

struct std_math {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_math {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &Vec<String>, _environment: &mut key_base::environment::Environment) -> String {
	//
	return String::new();
}

#[cxx::bridge]
mod ffi {
	extern "C" {
		include!("/usr/include/libqalculate/Calculator.h");
		fn calculate(cxx::CxxString) -> cxx::CxxString;
	}
}