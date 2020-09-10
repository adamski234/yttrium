use crate::tokenizer;
use crate::errors_and_warns;
use key_base;

type Id = usize;

#[allow(dead_code)]
pub fn create_ars_tree(ars_string: String, key_list: &Vec<Box<dyn key_base::Key>>) -> Result<TreeReturn, errors_and_warns::Error> {
	/*
	How things work:
	node_list is a flat vector of all nodes in the tree.
	This way I don't have to work with pointers but rather just vector indices
	After creating a new node, push it to node_list and use the new index as parent pointer
	*/
	let tokens = tokenizer::split_into_tokens(ars_string);
	let mut top_node_list = vec![
		TreeNode {
			key: String::from("top"), //`top` is the top level TreeNode containing all other nodes
			parameters: vec![Parameter::String(String::new())],
			is_editing_parameter: true,
			edited_parameter: 0,
			parent: None
		}
	];
	let mut warnings = Vec::new();
	let mut current_node_index = 0;
	let mut nodes_to_push = Vec::new();
	for token in tokens {
		let top_node_list_size = top_node_list.len(); //satisfying the borrow checker
		use tokenizer::TokenType;
		match token.token_type {
			TokenType::OpenBracket => {
				let inner = &mut top_node_list[current_node_index];
				if inner.is_editing_parameter {
					match &mut inner.parameters[inner.edited_parameter] {
						Parameter::Nodes(child_nodes) => {
							let new_node = TreeNode::new(current_node_index);
							child_nodes.push(top_node_list_size);
							nodes_to_push.push(new_node);
							current_node_index = top_node_list_size;
						}
						Parameter::String(text) => {
							let mut child_nodes = vec![ //Children IDs
								top_node_list_size,
							];
							if text.is_empty() {
								//Empty parameter
								let new_node = TreeNode::new(current_node_index);
								current_node_index = top_node_list_size;
								nodes_to_push.push(new_node);
							} else {
								//Not empty parameter
								let node_from_text = TreeNode::new_literal(current_node_index, text.to_string());
								nodes_to_push.push(node_from_text);
								child_nodes.push(top_node_list_size + 1);
								let new_node = TreeNode::new(current_node_index);
								nodes_to_push.push(new_node);
								current_node_index = top_node_list_size + 1;
							}
							inner.parameters[inner.edited_parameter] = Parameter::Nodes(child_nodes);
						}
					}
				} else {
					inner.key.push_str(&token.text);
				}
			}
			TokenType::CloseBracket => {
				if let Some(parent_node) = top_node_list[current_node_index].parent {
					current_node_index = parent_node;
				} else {
					let inner = &mut top_node_list[current_node_index];
					if inner.is_editing_parameter {
						match inner.parameters[inner.edited_parameter] {
							Parameter::Nodes(ref mut nodes) => {
								let new_node = TreeNode::new_literal(current_node_index, token.text);
								nodes_to_push.push(new_node);
								nodes.push(top_node_list_size);
							}
							Parameter::String(ref mut text) => {
								text.push_str(&token.text);
							}
						}
					} else {
						panic!("Top level was not editing parameter and a closing bracket was found");
					}
				}
			}
			TokenType::ParameterDelimiter => {
				let mut inner = &mut top_node_list[current_node_index];
				if inner.is_editing_parameter {
					inner.edited_parameter += 1;
					inner.parameters.push(Parameter::String(String::new()));
				} else {
					//No parameter
					inner.parameters = vec![Parameter::String(String::new())];
					inner.is_editing_parameter = true;
				}
			}
			TokenType::StringLiteral => {
				let inner = &mut top_node_list[current_node_index];
				if inner.is_editing_parameter {
					match &mut inner.parameters[inner.edited_parameter] {
						Parameter::String(text) => {
							text.push_str(&token.text);
						}
						Parameter::Nodes(child_nodes) => {
							let new_node = TreeNode::new_literal(current_node_index, token.text);
							child_nodes.push(top_node_list_size);
							nodes_to_push.push(new_node);
						}
					}
				} else {
					inner.key.push_str(&token.text);
				}
			}
		}
		top_node_list.append(&mut nodes_to_push);
	}
	if let Some(error) = errors_and_warns::check_for_errors(&top_node_list, key_list) {
		return Err(error);
	}
	if current_node_index != 0 {
		warnings.push(errors_and_warns::Warning::UnclosedKeys);
	}
	let to_return = TreeReturn {
		tree: top_node_list,
		warnings: if warnings.is_empty() { None } else { Some(warnings) },
	};
	return Ok(to_return);
}

#[derive(Debug)]
pub struct TreeNode {
	pub key: String, //Cannot be ars code, as it would require getting opcodes on the fly. Could work with an interpreter tho
	pub parameters: Vec<Parameter>, //String for literals, Nodes for variable values
	pub is_editing_parameter: bool,
	pub edited_parameter: usize,
	pub parent: Option<Id>, //Pointer, except that it's a vector index instead of a memory address
}

impl PartialEq for TreeNode {
	fn eq(&self, other: &Self) -> bool {
		return self.key == other.key && self.parameters == other.parameters && self.parent == other.parent;
	}
}

impl TreeNode {
	fn new(parent: Id) -> Self {
		return Self {
			key: String::new(),
			parameters: Vec::new(),
			is_editing_parameter: false,
			edited_parameter: 0,
			parent: Some(parent),
		};
	}
	fn new_literal(parent: Id, literal_text: String) -> Self {
		return Self {
			key: String::from("literal"),
			parameters: vec![Parameter::String(literal_text)],
			is_editing_parameter: true,
			edited_parameter: 0,
			parent: Some(parent),
		};
	}
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)] //Linter suggests that neither of the two variants are constructed so it's silenced
pub enum Parameter {
	Nodes(Vec<Id>),
	String(String)
}

#[derive(Debug, PartialEq)]
pub struct TreeReturn {
	pub tree: Vec<TreeNode>,
	pub warnings: Option<Vec<errors_and_warns::Warning>>,
}

//A wall of text is incoming. You probably should collapse them
#[cfg(test)]
mod tests {
	//Some basic definitions for testing
	struct Key1 {
		function: fn(parameter: &Vec<String>) -> bool,
		info: key_base::KeyInfo,
	}
	impl key_base::Key for Key1 {
		fn get_key_info(&self) -> &key_base::KeyInfo {
			return &self.info;
		}
		fn get_key_function(&self) -> fn(parameter: &Vec<String>) -> bool {
			return self.function;
		}
	}
	#[allow(dead_code)]
	fn placeholder_fn(_param: &Vec<String>) -> bool {
		return true;
	}
	fn load_keys_test() -> Vec<Box<dyn key_base::Key>> {
		//let keys = Vec::new();
		let mut keys = Vec::<Box<dyn key_base::Key>>::new();
		keys.push(
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![0],
					name: String::from("abc"),
					opcode: 0,
					allowed_key_names: vec![String::from("*")],
				}
			})
		);
		keys.push(
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![1],
					name: String::from("def"),
					opcode: 0,
					allowed_key_names: vec![String::from("*")],
				}
			})
		);
		keys.push(
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![0, 1, 3],
					name: String::from("ghi"),
					opcode: 0,
					allowed_key_names: vec![String::from("*")],
				}
			})
		);
		keys.push(
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![0],
					name: String::from("jkm"),
					opcode: 0,
					allowed_key_names: vec![String::from("*")],
				}
			})
		);
		keys.push(
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![2],
					name: String::from("ab"),
					opcode: 0,
					allowed_key_names: vec![String::from("*")],
				}
			})
		);
		keys.push(
			Box::new(Key1 {
				function: placeholder_fn,
				info: key_base::KeyInfo {
					parameters_required: vec![1, 2],
					name: String::from("bc"),
					opcode: 0,
					allowed_key_names: vec![String::from("*")],
				}
			})
		);
		return keys;
	}
	
	use super::*;
	#[test]
	fn tree_small_nesting() {
		let tested_string = String::from("{abc:{def:ghi}}");
		let correct = vec![
			TreeNode {
				key: String::from("top"),
				parameters: vec![
					Parameter::Nodes(
						vec![
							1,
						],
					),
				],
				is_editing_parameter: true,
				edited_parameter: 0,
				parent: None,
			},
			TreeNode {
				key: String::from("abc"),
				parameters: vec![
					Parameter::Nodes(
						vec![
							2,
						],
					),
				],
				is_editing_parameter: true,
				edited_parameter: 0,
				parent: Some(
					0,
				),
			},
			TreeNode {
				key: String::from("def"),
				parameters: vec![
					Parameter::String(
						String::from("ghi"),
					),
				],
				is_editing_parameter: true,
				edited_parameter: 0,
				parent: Some(
					1,
				),
			},
		];
		let output = create_ars_tree(tested_string, &load_keys_test()).unwrap().tree;
		assert_eq!(output, correct);
	}
	#[test]
	fn tree_no_nesting_only_keys() {
		let input = String::from("{abc}{def}{ghi}");
		let correct = vec![
			TreeNode {
				key: String::from("top"),
				parameters: vec![
					Parameter::Nodes(
						vec![
							1,
							2,
							3,
						],
					),
				],
				is_editing_parameter: true,
				edited_parameter: 0,
				parent: None,
			},
			TreeNode {
				key: String::from("abc"),
				parameters: vec![],
				is_editing_parameter: false,
				edited_parameter: 0,
				parent: Some(
					0,
				),
			},
			TreeNode {
				key: String::from("def"),
				parameters: vec![],
				is_editing_parameter: false,
				edited_parameter: 0,
				parent: Some(
					0,
				),
			},
			TreeNode {
				key: String::from("ghi"),
				parameters: vec![],
				is_editing_parameter: false,
				edited_parameter: 0,
				parent: Some(
					0,
				),
			},
		];
		let output = create_ars_tree(input, &load_keys_test()).unwrap().tree;
		assert_eq!(output, correct);
	}
	#[test]
	fn tree_nesting() {
		let input = String::from("abc{def}{ghi:jkm}{abc:{def:ghi}}");
		let correct_output = vec![
			TreeNode {
				key: String::from("top"),
				parameters: vec![
					Parameter::Nodes(
						vec![
							1,
							2,
							3,
							4,
						],
					),
				],
				is_editing_parameter: true,
				edited_parameter: 0,
				parent: None,
			},
			TreeNode {
				key: String::from("literal"),
				parameters: vec![
					Parameter::String(
						String::from("abc"),
					),
				],
				is_editing_parameter: true,
				edited_parameter: 0,
				parent: Some(
					0,
				),
			},
			TreeNode {
				key: String::from("def"),
				parameters: vec![],
				is_editing_parameter: false,
				edited_parameter: 0,
				parent: Some(
					0,
				),
			},
			TreeNode {
				key: String::from("ghi"),
				parameters: vec![
					Parameter::String(
						String::from("jkm"),
					),
				],
				is_editing_parameter: true,
				edited_parameter: 0,
				parent: Some(
					0,
				),
			},
			TreeNode {
				key: String::from("abc"),
				parameters: vec![
					Parameter::Nodes(
						vec![
							5,
						],
					),
				],
				is_editing_parameter: true,
				edited_parameter: 0,
				parent: Some(
					0,
				),
			},
			TreeNode {
				key: String::from("def"),
				parameters: vec![
					Parameter::String(
						String::from("ghi"),
					),
				],
				is_editing_parameter: true,
				edited_parameter: 0,
				parent: Some(
					4,
				),
			},
		];
		let output = create_ars_tree(input, &load_keys_test()).unwrap().tree;
		assert_eq!(output, correct_output);
	}
	#[test]
	fn unclosed_keys() {
		let input = String::from("{abc");
		let output_warns = create_ars_tree(input, &load_keys_test()).unwrap().warnings;
		assert_eq!(output_warns, Some(vec![errors_and_warns::Warning::UnclosedKeys]));
	}
}