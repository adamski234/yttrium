#![allow(clippy::needless_return, clippy::redundant_field_names)]
#![deny(clippy::implicit_return)]
#![feature(is_sorted)]
use std::collections::HashMap;

#[path = "./errors_and_warns.rs"] pub mod errors_and_warns;
#[path = "./key_loader.rs"] pub mod key_loader;
#[path = "./tree_creator.rs"] pub mod tree_creator; //#[path] allows to load a module from an arbitrary part
#[path ="./interpreter.rs"] pub mod interpreter;


pub fn run_ars_string(ars_string: String, key_list: &HashMap<String, Box<dyn key_base::Key>>, environment: key_base::environment::Environment) -> Result<ResultAndWarnings, errors_and_warns::Error> {
	match tree_creator::create_ars_tree(ars_string, key_list) {
		Ok(tree) => {
			match run_ars_tree(tree.tree, key_list, environment) {
				Ok(result) => {
					return Ok(ResultAndWarnings {
						result: result,
						warnings: tree.warnings,
					});
				}
				Err(reason) => {
					return Err(errors_and_warns::Error::InterpretationError(reason));
				}
			}
		}
		Err(error) => {
			return Err(error);
		}
	}
}

pub fn run_ars_tree(tree: Vec<tree_creator::TreeNode>, key_list: &HashMap<String, Box<dyn key_base::Key>>, environment: key_base::environment::Environment) -> Result<interpreter::InterpretationResult, String> {
	return interpreter::interpret_tree(tree, key_list, environment);
}

#[derive(Debug)]
pub struct ResultAndWarnings {
	pub result: interpreter::InterpretationResult,
	pub warnings: Option<Vec<errors_and_warns::Warning>>,
}