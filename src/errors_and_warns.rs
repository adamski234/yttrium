use crate::tree_creator;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Error {
	NoParameterSupplied,
	EmptyCondition,
	EmptyIfConditionTrue,
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
		//TODO check if key requires a parameter for `Error::NoParameterSupplied`
		match &node.inner_node {
			tree_creator::NodeEntryType::Unconditional(inner) => {
				if inner.is_editing_parameter {
					if let Some(param) = &inner.parameter {
						if let tree_creator::Parameter::String(text) = param {
							if text.is_empty() {
								return Some(Error::EmptyParameter);
							}
						}
					}
				}
			}
			tree_creator::NodeEntryType::Conditional(inner) => {
				if let tree_creator::Parameter::String(text) = &inner.condition {
					if text.is_empty() {
						return Some(Error::EmptyCondition);
					}
				}
				if let tree_creator::Parameter::String(text) = &inner.if_condition_true {
					if text.is_empty() {
						return Some(Error::EmptyIfConditionTrue);
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
		use crate::tree_creator::{TreeNode, NodeEntryType::Unconditional, Parameter, UnconditionalNodeEntry};
		let input = vec![TreeNode {
            inner_node: Unconditional(
                UnconditionalNodeEntry {
                    key: String::from("abc"),
                    parameter: Some(
                        Parameter::String(
                            String::new(),
                        ),
                    ),
                    is_editing_parameter: true,
                },
            ),
            parent: None
		}];
		let output = check_for_errors(&input);
		assert_eq!(output, Some(Error::EmptyParameter));
	}
	#[test]
	fn no_condition() {
		use crate::tree_creator::{TreeNode, NodeEntryType::Conditional, Parameter, ConditionalNodeEntry, CurrentlyEditedPartOfConditional::Condition};
		let input = vec![TreeNode {
			inner_node: Conditional(ConditionalNodeEntry {
				condition: Parameter::String(
					String::new(),
				),
				if_condition_true: Parameter::String(
					String::new(),
				),
				if_condition_false: None,
				currently_edited_part: Condition,
			}),
			parent: None,
		}];
		let output = check_for_errors(&input);
		assert_eq!(output, Some(Error::EmptyCondition));
	}
	#[test]
	fn no_condition_true() {
		use crate::tree_creator::{TreeNode, NodeEntryType::Conditional, Parameter, ConditionalNodeEntry, CurrentlyEditedPartOfConditional::Condition};
		let input = vec![TreeNode {
			inner_node: Conditional(ConditionalNodeEntry {
				condition: Parameter::String(
					String::from("abc"),
				),
				if_condition_true: Parameter::String(
					String::new(),
				),
				if_condition_false: None,
				currently_edited_part: Condition,
			}),
			parent: None,
		}];
		let output = check_for_errors(&input);
		assert_eq!(output, Some(Error::EmptyIfConditionTrue));
	}
}