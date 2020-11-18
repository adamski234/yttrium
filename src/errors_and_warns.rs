use std::collections::HashMap;
use crate::tree_creator;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Error {
	WrongAmountOfParameters,
	ParameterDelimAfterCondFalse,
	EmptyParameter,
	NonexistentKey,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Warning {
	UnclosedKeys,
}

pub fn check_for_errors(nodes: &[tree_creator::TreeNode], keys: &HashMap<String, Box<dyn key_base::Key>>) -> Option<Error> {
	for node in nodes {
		//TODO implement the rest of errors and add tests
		let param_count = node.parameters.len();
		match node.key.as_str() {
			"top" => {
				continue;
			}
			"literal" => {
				if param_count != 1 {
					return Some(Error::WrongAmountOfParameters);
				};
			}
			"cond" => {
				if !(2..=3).contains(&param_count) {
					return Some(Error::WrongAmountOfParameters);
				}
			}
			"exit" => {
				if param_count != 0 {
					return Some(Error::WrongAmountOfParameters);
				}
			}
			_ => {
				match keys.get(&node.key) {
					Some(key) => {
						let key_info = key.get_key_info();
						if !key_info.parameters_required.contains(&param_count) {
							return Some(Error::WrongAmountOfParameters);
						}
					}
					None => {
						return Some(Error::NonexistentKey);
					}
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
	}
	return None;
}

#[cfg(test)]
mod tests {
	use super::*;
	//Some basic definitions for testing
	struct Key1 {
		function: fn(parameter: &[String], env: &mut key_base::environment::Environment) -> String,
		info: key_base::KeyInfo,
	}
	impl key_base::Key for Key1 {
		fn get_key_info(&self) -> &key_base::KeyInfo {
			return &self.info;
		}
		fn get_key_function(&self) -> fn(parameter: &[String], env: &mut key_base::environment::Environment) -> String {
			return self.function;
		}
	}
	#[allow(dead_code)]
	fn placeholder_fn(_param: &[String], _env: &mut key_base::environment::Environment) -> String {
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
				}
			})
		);
		keys.insert(String::from("def"),
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![1],
					name: String::from("def"),
				}
			})
		);
		keys.insert(String::from("ghi"),
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![0, 1, 3],
					name: String::from("ghi"),
				}
			})
		);
		keys.insert(String::from("jkm"),
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![0],
					name: String::from("jkm"),
				}
			})
		);
		keys.insert(String::from("ab"),
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![2],
					name: String::from("ab"),
				}
			})
		);
		keys.insert(String::from("bc"),
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![1, 2],
					name: String::from("bc"),
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