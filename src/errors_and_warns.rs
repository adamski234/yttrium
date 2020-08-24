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

pub fn check_for_errors(nodes: &Vec<tree_creator::TreeNode>, keys: &Vec<Box<dyn key_base::Key>>) -> Option<Error> {
	for node in nodes {
		//TODO implement the rest of errors and add tests
		if node.is_editing_parameter {
			for key in keys {
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
			for key in keys {
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
	#[test]
	fn no_parameter() {
		use crate::tree_creator::{TreeNode, Parameter};
		let input = vec![TreeNode {
			key: String::from("abc"),
			parameters: vec![
				Parameter::String(
					String::new(),
				),
			],
			is_editing_parameter: true,
			edited_parameter: 0,
			parent: None
		}];
		let output = check_for_errors(&input, &vec![]);
		assert_eq!(output, Some(Error::EmptyParameter));
	}
}