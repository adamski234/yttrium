use std::collections::HashMap;
use crate::tree_creator;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Error {
	WrongAmountOfParameters,
	ParameterDelimAfterCondFalse,
	EmptyParameter,
	NonexistentKey,
	InterpretationError(String),
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
	#[test]
	fn no_parameter() {
		use crate::tree_creator::{TreeNode, Parameter};
		let keys = crate::key_loader::load_keys("");
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
		let output = check_for_errors(&input, &keys.keys);
		assert_eq!(output, Some(Error::NonexistentKey));
	}
}