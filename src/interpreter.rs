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
		//TODO: keys like `ars`, `cond`
		//Check if key is finished and execute with params
		let mut current_node = &mut interpretable_tree[current_index];
		if current_node.interpreted_param == current_node.inner_node.parameters.len() - 1 {
			let result = key_list.get(&current_node.inner_node.key).unwrap().get_key_function()(&current_node.returned_values, environment);
			if current_node.inner_node.parent.is_none() {
				#[cfg(debug_assertions)]
				println!("breaking because no parent");
				return InterpretationResult {
					message: result,
					embed: None,
				}
			}
			current_index = current_node.inner_node.parent.unwrap();
			current_node = &mut interpretable_tree[current_index];
			current_node.returned_values.push(result);
			current_node.interpreted_param += 1;
		} else {
			match &current_node.inner_node.parameters[current_node.interpreted_param] {
			    tree_creator::Parameter::Nodes(nodes) => {
					current_index = nodes[current_node.interpreted_param];
				}
			    tree_creator::Parameter::String(string) => {
					current_node.interpreted_param += 1;
					current_node.returned_values.push(string.clone());
				}
			}
		}
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