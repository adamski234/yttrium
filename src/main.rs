//This is a binary made specifically for testing and should be removed at some point

#![allow(clippy::needless_return)] //I'm gonna use returns whether clippy likes it or not
use std::collections::HashMap;
use ars::tree_creator;
use ars::key_loader;

use std::io::stdin;

fn main() {
	loop {
		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("An error has happened while reading from the console");
		//println!("{:#?}", ars::run_ars_string(input.trim().into(), &load_keys_test(), key_base::environment::events::EventType::Default, String::new(), )); //This will crash when I'm done
		println!("{:#?}", key_base::databases::Database::new_from_value(serde_json::from_str(&input).unwrap()));
	}
}

//This is for manual testing only
fn load_keys_test() -> HashMap<String, Box<dyn key_base::Key>> {
	let mut keys = HashMap::<String, Box<dyn key_base::Key>>::new();
	keys.insert(String::from("abc"),
		Box::new(Key1 {
			function: |_param, _env| {
				return String::from("abc");
			},
			info: key_base::KeyInfo {
				parameters_required: vec![0],
				name: String::from("abc"),
			}
		})
	);
	keys.insert(String::from("def"),
		Box::new(Key1 {
			function: |param, _env| {
				let mut returned = String::from("def:");
				returned.push_str(&param.join("|"));
				return returned;
			},
			info: key_base::KeyInfo {
				parameters_required: vec![1],
				name: String::from("def"),
			}
		})
	);
	keys.insert(String::from("ghi"),
		Box::new(Key1 {
			function: |param, _env| {
				let mut returned = String::from("ghi:");
				returned.push_str(&param.join("|"));
				return returned;
			},
			info: key_base::KeyInfo {
				parameters_required: vec![0, 1, 3],
				name: String::from("ghi"),
			}
		})
	);
	keys.insert(String::from("jkm"),
		Box::new(Key1 {
			function: |_param, _env| {
				return String::from("jkm");
			},
			info: key_base::KeyInfo {
				parameters_required: vec![0],
				name: String::from("jkm"),
			}
		})
	);
	keys.insert(String::from("ab"),
		Box::new(Key1 {
			function: |param, _env| {
				let mut returned = String::from("ab:");
				returned.push_str(&param.join("|"));
				return returned;
			},
			info: key_base::KeyInfo {
				parameters_required: vec![2],
				name: String::from("ab"),
			}
		})
	);
	keys.insert(String::from("bc"),
		Box::new(Key1 {
			function: |param, _env| {
				let mut returned = String::from("bc:");
				returned.push_str(&param.join("|"));
				return returned;
			},
			info: key_base::KeyInfo {
				parameters_required: vec![1, 2],
				name: String::from("bc"),
			}
		})
	);
	return keys;
}
struct Key1 {
	function: fn(&Vec<String>, &key_base::environment::Environment) -> String,
	info: key_base::KeyInfo,
}
impl key_base::Key for Key1 {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}
	fn get_key_function(&self) -> fn(&Vec<String>, &key_base::environment::Environment) -> String {
		return self.function;
	}
}