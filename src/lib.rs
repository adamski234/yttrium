#![allow(clippy::needless_return)]
#![feature(is_sorted)]
use std::collections::HashMap;

#[path = "./errors_and_warns.rs"] pub mod errors_and_warns;
#[path = "./tokenizer.rs"] mod tokenizer;
#[path = "./key_loader.rs"] pub mod key_loader;
#[path = "./tree_creator.rs"] pub mod tree_creator; //#[path] allows to load a module from an arbitrary part
#[path ="./interpreter.rs"] pub mod interpreter;


pub fn run_ars_string(ars_string: String, key_list: &HashMap<String, Box<dyn key_base::Key>>, guild_id: String, trigger_channel: String, message_id: String, user_id: String, trigger: String) -> Result<ResultAndWarnings, errors_and_warns::Error> {
	match tree_creator::create_ars_tree(ars_string, key_list) {
	    Ok(tree) => {
			return Ok(ResultAndWarnings {
				result: run_ars_tree(tree.tree, key_list, message_id, trigger_channel, guild_id, user_id, trigger),
				warnings: tree.warnings,
			});
		}
	    Err(error) => {
			return Err(error);
		}
	}
}

pub fn run_ars_tree(tree: Vec<tree_creator::TreeNode>, key_list: &HashMap<String, Box<dyn key_base::Key>>, guild_id: String, trigger_channel: String, message_id: String, user_id: String, trigger: String) -> interpreter::InterpretationResult {
	return interpreter::interpret_tree(tree, key_list, key_base::environment::Environment::new(message_id, trigger_channel, guild_id, user_id, trigger));
}

#[derive(Debug)]
pub struct ResultAndWarnings {
	pub result: interpreter::InterpretationResult,
	pub warnings: Option<Vec<errors_and_warns::Warning>>,
}