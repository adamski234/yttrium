#![allow(clippy::needless_return)] //I'm gonna use returns whether clippy likes it or not
use std::collections::HashMap;
use ars::tree_creator;
use ars::key_loader;

use std::io::stdin;

fn main() {
	let key_list = key_loader::load_keys("keys");
	loop {
		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("An error has happened while reading from the console");
		println!("{:#?}", ars::run_ars_string(input.trim().into(), &load_keys_test()));
	}
}

//This is for manual testing only
fn load_keys_test() -> HashMap<String, Box<dyn key_base::Key>> {
	let mut keys = HashMap::<String, Box<dyn key_base::Key>>::new();
	keys.insert(String::from("abc"),
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![0],
				name: String::from("abc"),
				opcode: 0,
				allowed_key_names: vec![String::from("*")],
			}
		})
	);
	keys.insert(String::from("def"),
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![1],
				name: String::from("def"),
				opcode: 0,
				allowed_key_names: vec![String::from("*")],
			}
		})
	);
	keys.insert(String::from("ghi"),
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![0, 1, 3],
				name: String::from("ghi"),
				opcode: 0,
				allowed_key_names: vec![String::from("*")],
			}
		})
	);
	keys.insert(String::from("jkm"),
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![0],
				name: String::from("jkm"),
				opcode: 0,
				allowed_key_names: vec![String::from("*")],
			}
		})
	);
	keys.insert(String::from("ab"),
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![2],
				name: String::from("ab"),
				opcode: 0,
				allowed_key_names: vec![String::from("*")],
			}
		})
	);
	keys.insert(String::from("bc"),
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![1, 2],
				name: String::from("bc"),
				opcode: 0,
				allowed_key_names: vec![String::from("*")],
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
#[allow(dead_code)]
fn placeholder_fn(_param: &Vec<String>, _env: &key_base::environment::Environment) -> String {
	return String::from("abc");
}