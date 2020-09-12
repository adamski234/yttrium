use std::collections::HashMap;
use crate::tree_creator;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Error {
	WrongAmountOfParameters,
	BracketsInCond,
	ParameterDelimAfterCondFalse,
	EmptyParameter,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Warning {
	UnclosedKeys,
}

pub fn check_for_errors(nodes: &Vec<tree_creator::TreeNode>, keys: &HashMap<String, Box<dyn key_base::Key>>) -> Option<Error> {
	for node in nodes {
		//TODO implement the rest of errors and add tests
		if node.is_editing_parameter {
			for key in keys.values() {
				let key_info = key.get_key_info();
				if key_info.name == node.key {
					//We have the correct key. Now check the parameter count
					if !key_info.parameters_required.contains(&(node.edited_parameter + 1)) {
						return Some(Error::WrongAmountOfParameters);
					}
				}
			}
			for param in &node.parameters {
				if let tree_creator::Parameter::String(string) = param {
					if string.is_empty() {
						return Some(Error::EmptyParameter);
					}
				}
			}
		} else {
			for key in keys.values() {
				let key_info = key.get_key_info();
				if key_info.name == node.key {
					if key_info.parameters_required[0] != 0 {
						return Some(Error::WrongAmountOfParameters);
					}
				}
			}
		}
	}
	return None;
}

#[cfg(test)]
mod tests {
	use super::*;
	//Some basic definitions for testing
	struct Key1 {
		function: fn(parameter: &Vec<String>, env: &key_base::environment::Environment) -> String,
		info: key_base::KeyInfo,
	}
	impl key_base::Key for Key1 {
		fn get_key_info(&self) -> &key_base::KeyInfo {
			return &self.info;
		}
		fn get_key_function(&self) -> fn(parameter: &Vec<String>, env: &key_base::environment::Environment) -> String {
			return self.function;
		}
	}
	#[allow(dead_code)]
	fn placeholder_fn(_param: &Vec<String>, _env: &key_base::environment::Environment) -> String {
		return String::from("return");
	}
	fn load_keys_test() -> HashMap<String, Box<dyn key_base::Key>> {
		//let keys = Vec::new();
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
	
	#[test]
	fn no_parameter() {
		use crate::tree_creator::{TreeNode, Parameter};
		let input = vec![TreeNode {
			key: String::from("does_not_exist"),
			parameters: vec![
				Parameter::String(
					String::new(),
				),
			],
			is_editing_parameter: true,
			edited_parameter: 0,
			parent: None
		}];
		let output = check_for_errors(&input, &load_keys_test());
		assert_eq!(output, Some(Error::EmptyParameter));
	}
}