#[allow(clippy::needless_return)]
#[path = "tokenizer.rs"] mod tokenizer;

pub fn create_ars_tree(ars_string: String) -> Vec<TreeNodeOrString> {
	/*
	How things work:
	node_list is a flat vector of all nodes in the tree.
	This way I don't have to work with pointers but rather just vector indices
	I'll probably want to jump off a bridge after finishing it
	*/
	let mut node_list = vec![
		TreeNodeOrString::String(String::new())
	];
	let mut current_node_index = 0;
	let tokens = tokenizer::split_into_tokens(ars_string);
	for token in tokens {
		match token.token_type {
			tokenizer::TokenType::OpenBracket => {
				//
			}
			tokenizer::TokenType::CloseBracket => {
				//
			}
			tokenizer::TokenType::ParameterDelimiter => {
				//
			}
			tokenizer::TokenType::StringLiteral => {
				//
			}
		}
	}
	return node_list;
}

#[derive(Debug)]
pub struct TreeNode {
	key: TreeNodeOrString,
	parameter: TreeNodeOrString,
	is_editing_parameter: bool,
	parent: Option<usize>, //Pointer, except that it's a vector index instead of a memory address
}

#[derive(Debug)]
pub enum TreeNodeOrString {
	NodesOrStrings(Vec<TreeNodeOrString>),
	Node(Box<TreeNode>),
	String(String)
}