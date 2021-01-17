use std::collections::HashMap;
use yttrium_key_base::environment::Environment;
use yttrium_key_base::databases::{
	DatabaseManager,
	Database,
};
use crate::tree_creator;

/// Runs the created ARS tree
/// # Arguments: 
/// * `tree` - The tree in vector form returned from [tree_creator::create_ars_tree]
/// * `key_list` - A HashMap of keys, probably returned from [crate::key_loader::load_keys]
/// * `environment` - The environment from [key_base::environment::Environment]
pub async fn interpret_tree<Manager: DatabaseManager<DB>, DB: Database>(tree: Vec<tree_creator::TreeNode>, key_list: &HashMap<String, Box<dyn yttrium_key_base::Key<Manager, DB> + Send + Sync>>, mut environment: Environment<'_, Manager, DB>) -> Result<InterpretationResult, String> {
	let mut current_index = 0; //Pointer to the currently interpreted node
	let mut interpretable_tree = Vec::with_capacity(tree.len());
	let mut next_rule = None;
	for node in tree {
		let cond_if_false = node.key == "cond" && node.parameters.len() == 3;
		let param_count = node.parameters.len();
		interpretable_tree.push(InterpretableNode {
			inner_node: node,
			interpreted_param: 0, //For `inner_node.parameters`
			interpreted_subparam: 0, //For `inner_node.parameters[interpreted_param]` (Nodes)
			returned_values: Vec::with_capacity(param_count),
			returned_subvalues: Vec::with_capacity(2),
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
						return Ok(InterpretationResult {
							message: current_node.returned_values.join(""),
							embed: environment.embed,
							next_rule: next_rule,
							attachments: environment.attachments,
							reactions: environment.reactions_to_add,
							self_delete: environment.delete_option,
							target: serenity::model::id::ChannelId::from(environment.target.parse::<u64>().unwrap()),
						});
					} else {
						match key_list.get(&current_node.inner_node.key).unwrap().run_key(&current_node.returned_values, &mut environment).await {
							Ok(result) => {
								returned = result;
							}
							Err(error) => {
								return Err(error);
							}
						}
					}
					current_index = parent;
					current_node = &mut interpretable_tree[current_index];
					current_node.returned_subvalues.push(returned);
				}
				None => {
					//No more keys to interpret, return the result
					return Ok(InterpretationResult {
						message: current_node.returned_values.join(""),
						embed: environment.embed,
						next_rule: next_rule,
						attachments: environment.attachments,
						reactions: environment.reactions_to_add,
						self_delete: environment.delete_option,
						target: serenity::model::id::ChannelId::from(environment.target.parse::<u64>().unwrap()),
					});
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

/// Struct containing everything that the script might return
#[derive(Debug)]
pub struct InterpretationResult {
	/// The returned message
	pub message: String,
	/// The created embed
	pub embed: Option<yttrium_key_base::embed::Embed>,
	/// The next rule to call
	pub next_rule: Option<String>,
	/// A list of attachments to send
	pub attachments: Vec<String>,
	/// A list of reactions to add to the sent message
	pub reactions: Vec<String>,
	/// Time after which the sent message should be deleted
	pub self_delete: Option<std::time::Duration>,
	/// Channel to send the result to
	pub target: serenity::model::id::ChannelId,
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