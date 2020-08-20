use crate::tree_creator;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Error {
	NotEnoughParameters,
	TooManyParameters,
	BracketsInCond,
	ParameterDelimAfterCondFalse,
	EmptyParameter,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Warning {
	UnclosedKeys,
}

pub fn check_for_errors(nodes: &Vec<tree_creator::TreeNode>) -> Option<Error> {
	for node in nodes {
		//TODO implement the rest of errors and add tests
		if node.is_editing_parameter {
			for param in &node.parameters {
				if let tree_creator::Parameter::String(string) = param {
					if string.is_empty() {
						return Some(Error::EmptyParameter);
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
		let output = check_for_errors(&input);
		assert_eq!(output, Some(Error::EmptyParameter));
	}
}