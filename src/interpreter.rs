use std::collections::HashMap;
use key_base::environment::Environment;
use crate::tree_creator;

pub fn interpret_tree(tree: Vec<tree_creator::TreeNode>, key_list: &HashMap<String, Box<dyn key_base::Key>>, mut environment: Environment) -> InterpretationResult {
	let mut current_index = 0; //Pointer to the currently interpreted node
	let mut interpretable_tree = Vec::with_capacity(tree.len());
	let mut next_rule = None;
	for node in tree {
		let cond_if_false = node.key == "cond" && node.parameters.len() == 3;
		let param_count = node.parameters.len();
		let mut subparam_longest = 0;
		for param in &node.parameters {
			match param {
				tree_creator::Parameter::Nodes(nodes) => {
					if nodes.len() > subparam_longest {
						subparam_longest = nodes.len();
					}
				}
				tree_creator::Parameter::String(_) => {
					continue;
				}
			}
		}
		interpretable_tree.push(InterpretableNode {
			inner_node: node,
			interpreted_param: 0, //For `inner_node.parameters`
			interpreted_subparam: 0, //For `inner_node.parameters[interpreted_param]` (Nodes)
			returned_values: Vec::with_capacity(param_count),
			returned_subvalues: Vec::with_capacity(subparam_longest),
			cond_if_false: cond_if_false,
		});
	}
	//Remember: no recursion
	loop {
		let mut current_node = &mut interpretable_tree[current_index];
		//This is bad coding but I lack creativity to fix this
		if current_node.inner_node.key == "cond" && current_node.interpreted_param == 1 {
			//The first parameter has been executed
			if current_node.returned_values[0].is_empty() || current_node.returned_values[0] == "0" {
				if current_node.inner_node.parameters.len() == 3 {
					current_node.interpreted_param = 2;
				} else {
					current_index = current_node.inner_node.parent.unwrap();
					current_node = &mut interpretable_tree[current_index];
					current_node.returned_subvalues.push(String::from(""));
				}
			} else {
				//To do somewhere in the future: Extract this into a function
				let current_parameter = &current_node.inner_node.parameters[current_node.interpreted_param];
				match current_parameter {
					tree_creator::Parameter::Nodes(nodes) => {
						//Handle nodes as parameters
						if nodes.len() == current_node.returned_subvalues.len() {
							/* All parameter chunks were ran, now push the result of the parameter to the parent,
							clear the subvalues, increment the parameter pointer and zero the subparameter pointer
							*/
							current_node.finish_subvalue();
							current_node.interpreted_param += 1;
						} else {
							//Handle unfinished parameters
							current_index = nodes[current_node.interpreted_subparam];
							current_node.interpreted_subparam += 1;
						}
					}
					tree_creator::Parameter::String(text) => {
						//Handle just text as parameter
						current_node.returned_values.push(text.clone());
						current_node.interpreted_param += 1;
						current_node.interpreted_subparam = 0;
					}
				}
			}
		} else if current_node.inner_node.key == "cond" && current_node.interpreted_param == 2 {
			if current_node.returned_values[0].is_empty() || current_node.returned_values[0] == "0" {
				let current_parameter = &current_node.inner_node.parameters[current_node.interpreted_param];
				match current_parameter {
					tree_creator::Parameter::Nodes(nodes) => {
						//Handle nodes as parameters
						if nodes.len() == current_node.returned_subvalues.len() {
							/* All parameter chunks were ran, now push the result of the parameter to the parent,
							clear the subvalues, increment the parameter pointer and zero the subparameter pointer
							*/
							current_node.finish_subvalue();
							current_node.interpreted_param += 1;
						} else {
							//Handle unfinished parameters
							current_index = nodes[current_node.interpreted_subparam];
							current_node.interpreted_subparam += 1;
						}
					}
					tree_creator::Parameter::String(text) => {
						//Handle just text as parameter
						current_node.returned_values.push(text.clone());
						current_node.interpreted_param += 1;
						current_node.interpreted_subparam = 0;
					}
				}
			} else {
				//First param is truthy, return the second param
				let to_return = current_node.returned_values[1].clone();
				current_index = current_node.inner_node.parent.unwrap();
				current_node = &mut interpretable_tree[current_index];
				current_node.returned_subvalues.push(to_return);
			}
		} else if current_node.inner_node.key == "cond" && current_node.interpreted_param == 3 {
			//First param is falsey, return the third param
			let to_return = current_node.returned_values[1].clone();
			current_index = current_node.inner_node.parent.unwrap();
			current_node = &mut interpretable_tree[current_index];
			current_node.returned_subvalues.push(to_return);
		} else if current_node.inner_node.parameters.len() == current_node.interpreted_param {
			//Finished executing all the parameters, now execute the key and return to parent
			match current_node.inner_node.parent {
				Some(parent) => {
					let returned;
					if current_node.inner_node.key == "ars" {
						next_rule = Some(current_node.returned_values.join(""));
						returned = String::new();
					} else if current_node.inner_node.key == "literal" {
						returned = current_node.returned_values[0].clone();
					} else if current_node.inner_node.key == "exit" {
						//Stop the interepreter
						return InterpretationResult {
							message: current_node.returned_values.join(""),
							embed: environment.embed,
							next_rule: next_rule,
							attachments: environment.attachments,
							reactions: environment.reactions_to_add,
							self_delete: environment.delete_option,
						};
					} else {
						returned = key_list.get(&current_node.inner_node.key).unwrap().get_key_function()(&current_node.returned_values, &mut environment);
					}
					current_index = parent;
					current_node = &mut interpretable_tree[current_index];
					current_node.returned_subvalues.push(returned);
				}
				None => {
					//No more keys to interpret, return the result
					return InterpretationResult {
						message: current_node.returned_values.join(""),
						embed: environment.embed,
						next_rule: next_rule,
						attachments: environment.attachments,
						reactions: environment.reactions_to_add,
						self_delete: environment.delete_option,
					};
				}
			}
		} else {
			let current_parameter = &current_node.inner_node.parameters[current_node.interpreted_param];
			match current_parameter {
				tree_creator::Parameter::Nodes(nodes) => {
					//Handle nodes as parameters
					if nodes.len() == current_node.returned_subvalues.len() {
						/* All parameter chunks were ran, now push the result of the parameter to the parent,
						clear the subvalues, increment the parameter pointer and zero the subparameter pointer
						*/
						current_node.finish_subvalue();
						current_node.interpreted_param += 1;
					} else {
						//Handle unfinished parameters
						current_index = nodes[current_node.interpreted_subparam];
						current_node.interpreted_subparam += 1;
					}
				}
				tree_creator::Parameter::String(text) => {
					//Handle just text as parameter
					current_node.returned_values.push(text.clone());
					current_node.interpreted_param += 1;
					current_node.interpreted_subparam = 0;
				}
			}
		}
	}
}
#[derive(Debug)]
pub struct InterpretationResult {
	pub message: String,
	pub embed: Option<key_base::embed::Embed>,
	pub next_rule: Option<String>,
	pub attachments: Vec<String>,
	pub reactions: Vec<String>,
	pub self_delete: Option<std::time::Duration>,
}

struct InterpretableNode {
	pub inner_node: tree_creator::TreeNode,
	pub interpreted_param: usize,
	pub interpreted_subparam: usize,
	pub returned_values: Vec<String>,
	pub returned_subvalues: Vec<String>,
	pub cond_if_false: bool,
}

impl InterpretableNode {
	fn finish_subvalue(&mut self) {
		self.returned_values.push(self.returned_subvalues.join(""));
		self.returned_subvalues.clear();
		self.interpreted_subparam = 0;
	}
}