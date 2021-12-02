use std::collections::HashMap;
use std::time::Duration;
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
pub async fn interpret_tree<'a, Manager: DatabaseManager<DB>, DB: Database>(tree: Vec<tree_creator::TreeNode>, key_list: &HashMap<String, Box<dyn yttrium_key_base::Key<Manager, DB> + Send + Sync>>, mut environment: Environment<'a, Manager, DB>) -> Result<InterpretationResultOrSleep<'a, Manager, DB>, String> {
	let mut current_index = 0; //Pointer to the currently interpreted node
	let mut interpretable_tree = Vec::with_capacity(tree.len());
	for node in tree {
		let param_count = node.parameters.len();
		interpretable_tree.push(InterpretableNode {
			inner_node: node,
			interpreted_param: 0, //For `inner_node.parameters`
			interpreted_subparam: 0, //For `inner_node.parameters[interpreted_param]` (Nodes)
			returned_values: Vec::with_capacity(param_count),
			returned_subvalues: Vec::with_capacity(2),
		});
	}
	loop {
		let mut current_node = &mut interpretable_tree[current_index];
		//This is bad coding but I lack creativity to fix this
		// TODO function handling `cond` in its entirety
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
					if current_node.inner_node.key == "literal" {
						returned = current_node.returned_values[0].clone();
					} else if current_node.inner_node.key == "exit" {
						//Stop the interepreter
						return Ok(InterpretationResultOrSleep::Result(
							InterpretationResult {
								message: current_node.returned_values.join(""),
								environment: environment,
							}
						));
					} else {
						match key_list.get(&current_node.inner_node.key) {
							Some(key) => {
								match key.run_key(&current_node.returned_values, &mut environment).await {
									Ok(result) => {
										returned = result;
									}
									Err(error) => {
										return Err(error);
									}
								}
							}
							None => {
								return Err(String::from("One of the keys does not exist"));
							}
						}
					}
					current_index = parent;
					current_node = &mut interpretable_tree[current_index];
					current_node.returned_subvalues.push(returned);
				}
				None => {
					//No more keys to interpret, return the result
					return Ok(InterpretationResultOrSleep::Result(
						InterpretationResult {
							message: current_node.returned_values.join(""),
							environment: environment,
						}
					));
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
		if environment.sleep_time.is_some() {
			// Serialize code up the point of execution and hand it over to the caller for further processing
			// Remove current sleep node
			let param = &mut current_node.inner_node.parameters[current_node.interpreted_param];
			match param {
				tree_creator::Parameter::Nodes(nodes) => {
					for i in current_node.interpreted_subparam..(nodes.len() - 1) {
						nodes[i] = nodes[i + 1];
					}
					nodes.pop();
				}
				tree_creator::Parameter::String(_) => {
					unreachable!("`sleep_time` was Some but the parameter was a string");
				}
			}
			return Ok(InterpretationResultOrSleep::Sleep(
				SleepResult {
					duration: environment.sleep_time.unwrap(),
					code_to_execute: interpretable_tree[0].serialize(),
					environment: environment,
				}
			))
		}
	}
}

/// Enum that describes the further behavior of the interpreter
#[derive(Debug)]
pub enum InterpretationResultOrSleep<'a, Manager: DatabaseManager<DB>, DB: Database> {
	/// The resulting text of the interpretation
	Result(InterpretationResult<'a, Manager, DB>),
	/// Information regarding when to wake the interpreter again and with what data
	Sleep(SleepResult<'a, Manager, DB>)
}

/// Struct containing everything that the script might return
#[derive(Debug)]
pub struct InterpretationResult<'a, Manager: DatabaseManager<DB>, DB: Database> {
	/// The returned message
	pub message: String,
	/// Environment passed as the argument
	pub environment: Environment<'a, Manager, DB>,
}

#[derive(Debug)]
pub struct SleepResult<'a, Manager: DatabaseManager<DB>, DB: Database> {
	/// Duration for which the interpreter should be paused. Time resolution is up to the implementer
	pub duration: Duration,
	/// Code that should be executed after the sleep time is over
	pub code_to_execute: String,
	/// The resulting environment passed as the argument
	pub environment: Environment<'a, Manager, DB>,
}

struct InterpretableNode {
	pub inner_node: tree_creator::TreeNode,
	pub interpreted_param: usize,
	pub interpreted_subparam: usize,
	pub returned_values: Vec<String>,
	pub returned_subvalues: Vec<String>,
}

impl InterpretableNode {
	fn finish_subvalue(&mut self) {
		self.returned_values.push(self.returned_subvalues.join(""));
		self.returned_subvalues.clear();
		self.interpreted_subparam = 0;
	}
	/// Recursively serializes itself along with all the child nodes, returning code that should be executed after resuming
	fn serialize(&self) -> String {
		//TODO
		return String::new();
	}
}