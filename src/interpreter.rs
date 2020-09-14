use std::collections::HashMap;
use key_base::environment::Environment;
use crate::tree_creator;

pub fn interpret_tree(tree: Vec<tree_creator::TreeNode>, key_list: &HashMap<String, Box<dyn key_base::Key>>, environment: &mut Environment) -> InterpretationResult {
	let mut current_index = 0;
	let mut interpretable_tree = Vec::with_capacity(tree.len());
	let mut next_rule = None;
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
		//FIXME: Shit ain't working
		//Iterate over parameters adding them to the result
		let mut current_node = &mut interpretable_tree[current_index];
		if current_node.interpreted_param == current_node.inner_node.parameters.len() {
			if current_node.inner_node.parent.is_none() {
				#[cfg(debug_assertions)]
				println!("breaking because no parent");
				return InterpretationResult {
					message: current_node.returned_values.join(""),
					embed: None,
					next_rule: next_rule,
				}
			}
			let result;
			if current_node.inner_node.key == "literal" {
				match &current_node.inner_node.parameters[0] {
				    tree_creator::Parameter::Nodes(_) => {
						panic!("`literal` node had nodes as parameters")
					}
				    tree_creator::Parameter::String(text) => {
						result = text.clone();
					}
				}
			} else if current_node.inner_node.key == "cond" {
				result = current_node.returned_values[2].clone();
			} else if current_node.inner_node.key == "ars" {
				next_rule = Some(current_node.returned_values.join(""));
				result = String::new();
			} else {
				result = key_list.get(&current_node.inner_node.key).unwrap().get_key_function()(&current_node.returned_values, environment);
			}
			current_index = current_node.inner_node.parent.unwrap();
			current_node = &mut interpretable_tree[current_index];
			current_node.returned_values.push(result);
			current_node.interpreted_param += 1;
		} else if current_node.inner_node.key == "cond" && current_node.interpreted_param == current_node.inner_node.parameters.len() - 1 {
			if current_node.returned_values[0] == "1" {
				let result = current_node.returned_values[1].clone(); //FIXME: crashes
				current_index = current_node.inner_node.parent.unwrap();
				current_node = &mut interpretable_tree[current_index];
				current_node.returned_values.push(result);
			}
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

#[derive(Debug)]
pub struct InterpretationResult {
	pub message: String,
	pub embed: Option<Embed>,
	pub next_rule: Option<String>,
}

#[derive(Debug)]
pub struct Embed;

struct InterpretableNode {
	pub inner_node: tree_creator::TreeNode,
	pub interpreted_param: usize,
	pub returned_values: Vec<String>
}