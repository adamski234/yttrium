#[allow(clippy::needless_return)]
#[path = "tokenizer.rs"] mod tokenizer;

type Id = usize;

pub fn create_ars_tree(ars_string: String) /*-> Vec<Parameter>*/ {
	/*
	How things work:
	node_list is a flat vector of all nodes in the tree.
	This way I don't have to work with pointers but rather just vector indices
	After creating a new node, push it to node_list and use the new index as parent pointer
	I'll probably want to jump off a bridge after finishing it
	*/
	let mut node_list = vec![
		TreeNode {
			key: String::from("top"), //`top` is the top level TreeNode containing all other nodes
			parameter: Some(Parameter::String(String::new())),
			is_editing_parameter: true,
			parent: None
		}
	];
	let mut current_node_index = 0;
	let tokens = tokenizer::split_into_tokens(ars_string);
	for token in tokens {
		use tokenizer::TokenType;
		match token.token_type {
			TokenType::OpenBracket => {
				if node_list[current_node_index].is_editing_parameter {
					match &mut node_list[current_node_index].parameter {
						Some(param) => {
							//
						}
						None => {
							//
						}
					}
				} else {
					node_list[current_node_index].key.push_str(&token.text);
				}
			}
			TokenType::CloseBracket => {
				//
			}
			TokenType::ParameterDelimiter => {
				//
			}
			TokenType::StringLiteral => {
				//
			}
		}
	}
	//return node_list;
}

#[derive(Debug)]
pub struct TreeNode {
	key: String, //Cannot be ars code, as it would require getting opcodes on the fly. Could work with an interpreter tho
	parameter: Option<Parameter>, //String for literals, Nodes for variable values
	is_editing_parameter: bool,
	parent: Option<Id>, //Pointer, except that it's a vector index instead of a memory address
}

#[derive(Debug)]
pub enum Parameter {
	Nodes(Vec<Id>),
	String(String)
}