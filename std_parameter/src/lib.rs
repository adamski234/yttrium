#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	/*
	Parameters:
	Optional, the string to split on. If empty returns the entire parameter string
	Required, the index of the split string to return
	*/
	let key_info = key_base::KeyInfo {
		name: String::from("parameter"),
		parameters_required: vec![0, 2],
	};
	return Box::into_raw(Box::new(std_parameter {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_parameter {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_parameter {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	if parameter.is_empty() {
		return environment.parameter.clone();
	} else {
		let index;
		match parameter[1].parse::<usize>() {
			Ok(value) => {
				index = value;
			}
			Err(_) => {
				return String::new();
			}
		}
		if !environment.split_parameters.contains_key(&parameter[0]) {
			//This is hacky and I don't like this
			let split = environment.parameter.split(&parameter[0]).map(String::from).collect();
			environment.split_parameters.insert(parameter[0].clone(), split);
		}
		let split = environment.split_parameters.get(&parameter[0]).unwrap();
		if split.len() >= index + 1 {
			return split[index].clone();
		} else {
			return String::new();
		}
	}
}