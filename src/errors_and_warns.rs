use std::collections::HashMap;
use crate::tree_creator;
use yttrium_key_base::databases::{
	Database,
	DatabaseManager,
};

/// Enum containing different error types
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Error {
	/// Error used when the key receives the wrong amount of parameters
	WrongAmountOfParameters,
	/// Error used when a parameter is empty, usually happens by accident
	EmptyParameter,
	/// Error used when a key does not exist in `keys`
	NonexistentKey,
	/// Error returned from [crate::interpreter::interpret_tree]
	InterpretationError(String),
}

/// Enum containing warnings that aren't explicitly errors, but are likely undesirable
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Warning {
	/// Warning used when there is an unmatched opening bracket
	/// Like `{key`
	UnclosedKeys,
}

/// Checks the ARS tree for syntax errors
/// # Arguments:
/// * `nodes` - An array of [TreeNodes](tree_creator::TreeNode), probably created by [crate::tree_creator::create_ars_tree]
/// * `keys` - The HashMap of keys
pub fn check_for_errors<Manager: DatabaseManager<DB>, DB: Database>(nodes: &[tree_creator::TreeNode], keys: &HashMap<String, Box<dyn yttrium_key_base::Key<Manager, DB> + Send + Sync>>) -> Option<Error> {
	for node in nodes {
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
	use yttrium_key_base::databases::{
		JSONDatabaseManager,
		JSONDatabase,
	};
    use super::*;
	#[test]
	fn no_parameter() {
		use crate::tree_creator::{TreeNode, Parameter};
		let keys = crate::key_loader::load_keys::<JSONDatabaseManager, JSONDatabase>();
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
		let output = check_for_errors(&input, &keys);
		assert_eq!(output, Some(Error::NonexistentKey));
	}
}