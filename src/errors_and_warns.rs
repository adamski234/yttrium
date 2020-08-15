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