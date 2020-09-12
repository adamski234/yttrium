use std::collections::HashMap;
use key_base::environment::Environment;
use crate::tree_creator;

pub fn interpret_tree(tree: Vec<tree_creator::TreeNode>, key_list: &HashMap<String, Box<dyn key_base::Key>>, environment: &mut Environment) -> InterpretationResult {
	let mut current_index = 0;
	let mut interpretable_tree = Vec::with_capacity(tree.len());
	for node in tree {
		interpretable_tree.push(InterpretableNode {
			inner_node: node,
			interpreted_param: 0,
			returned_values: Vec::new(),
		});
	}
	//Remember: no recursion
	loop {
		//Check if key is finished and execute with params
		if interpretable_tree[current_index].interpreted_param == interpretable_tree[current_index].inner_node.parameters.len() - 1 {
			let result = key_list.get(&interpretable_tree[current_index].inner_node.key).unwrap().get_key_function()(&interpretable_tree[current_index].returned_values, environment);
			if interpretable_tree[current_index].inner_node.parent.is_none() {
				#[cfg(debug_assertions)]
				println!("breaking because no parent");
				break;
			}
			current_index = interpretable_tree[current_index].inner_node.parent.unwrap();
			interpretable_tree[current_index].returned_values.push(result);
		}
		break;
	}
	return InterpretationResult {
		message: String::new(),
		embed: None,
	}
}

pub struct InterpretationResult {
	pub message: String,
	pub embed: Option<Embed>,
}

pub struct Embed;

struct InterpretableNode {
	pub inner_node: tree_creator::TreeNode,
	pub interpreted_param: usize,
	pub returned_values: Vec<String>
}