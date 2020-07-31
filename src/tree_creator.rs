#[allow(clippy::needless_return)]
use crate::tokenizer;

type Id = usize;

pub fn create_ars_tree(ars_string: String) -> Vec<TreeNode> {
	/*
	How things work:
	node_list is a flat vector of all nodes in the tree.
	This way I don't have to work with pointers but rather just vector indices
	After creating a new node, push it to node_list and use the new index as parent pointer
	I'll probably want to jump off a bridge after finishing it
	*/
	let tokens = tokenizer::split_into_tokens(ars_string); //TODO: multithread it
	let mut top_node_list = vec![
		TreeNode {
			inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
				key: String::from("top"), //`top` is the top level TreeNode containing all other nodes
				parameter: Some(Parameter::String(String::new())),
				is_editing_parameter: true,
			}),
			parent: None
		}
	];
	let mut current_node_index = 0;
	for token in tokens {
		let top_node_list_size = top_node_list.len(); //satisfying the borrow checker
		use tokenizer::TokenType;
		match token.token_type {
			TokenType::OpenBracket => {
				match top_node_list[current_node_index].inner_node {
					NodeEntryType::Unconditional(mut inner) => {
						if inner.is_editing_parameter {
							match &mut inner.parameter {
								Some(param) => {
									match param {
										Parameter::Nodes(child_nodes) => {
											let new_node = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::new(),
													parameter: None,
													is_editing_parameter: false,
												}),
												parent: Some(current_node_index),
											};
											child_nodes.push(top_node_list_size);
											top_node_list.push(new_node);
											current_node_index = top_node_list_size;
										}
										Parameter::String(text) => {
											let node_from_text = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::from("literal"),
													parameter: Some(Parameter::String(text.to_string())),
													is_editing_parameter: true,
												}),
												parent: Some(current_node_index)
											};
											let new_node = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::new(),
													parameter: None,
													is_editing_parameter: false,
												}),
												parent: Some(current_node_index),
											};
											let child_nodes = vec![ //Children IDs
												top_node_list_size,
												top_node_list_size + 1,
											];
											top_node_list.push(node_from_text);
											top_node_list.push(new_node);
											inner.parameter = Some(Parameter::Nodes(child_nodes));
											current_node_index = top_node_list_size + 1;
										}
									}
								}
								None => {
									let new_node = TreeNode {
										inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
											key: String::new(),
											parameter: None,
											is_editing_parameter: false,
										}),
										parent: Some(current_node_index),
									};
									inner.parameter = Some(Parameter::Nodes(vec![top_node_list_size]));
									top_node_list.push(new_node);
								}
							}
						} else {
							inner.key.push_str(&token.text);
						}
					}
					NodeEntryType::Conditional(mut inner) => {
						let new_node = TreeNode {
							inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
								key: String::new(),
								parameter: None,
								is_editing_parameter: false,
							}),
							parent: Some(top_node_list_size),
						};
						top_node_list.push(new_node);
						current_node_index = top_node_list_size;
						match inner.currently_edited_part {
							CurrentlyEditedPartOfConditional::Condition => {
								inner.condition = top_node_list_size;
							},
							CurrentlyEditedPartOfConditional::ConditionTrue => {
								inner.if_condition_true = top_node_list_size;
							},
							CurrentlyEditedPartOfConditional::ConditionFalse => {
								inner.if_condition_false = Some(top_node_list_size);
							}
						}
					}
				}
			}
			TokenType::CloseBracket => {
				if let Some(parent_node) = top_node_list[current_node_index].parent {
					current_node_index = parent_node;
				}
			}
			TokenType::ParameterDelimiter => {
				match top_node_list[current_node_index].inner_node {
					NodeEntryType::Unconditional(mut inner) => {
						if inner.is_editing_parameter {
							match &mut inner.parameter {
								Some(param) => {
									match param {
										Parameter::Nodes(child_nodes) => {
											let new_node = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::from("literal"),
													parameter: Some(Parameter::String(token.text)),
													is_editing_parameter: true,
												}),
												parent: Some(current_node_index),
											};
											child_nodes.push(top_node_list_size);
											top_node_list.push(new_node);
										}
										Parameter::String(text) => {
											text.push_str(&token.text);
										}
									}
								}
								None => {
									inner.parameter = Some(Parameter::String(String::new()));
								}
							}
						} else {
							//No parameter
							inner.parameter = Some(Parameter::String(String::new()));
							inner.is_editing_parameter = true;
						}
					}
					NodeEntryType::Conditional(mut inner) => {
						match inner.currently_edited_part {
							CurrentlyEditedPartOfConditional::Condition => {
								inner.currently_edited_part = CurrentlyEditedPartOfConditional::ConditionTrue;
								let new_node = TreeNode {
									inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
										key: String::from(""),
										parameter: Some(Parameter::String(token.text)),
										is_editing_parameter: true,
									}),
									parent: Some(current_node_index),
								};
								top_node_list.push(new_node);
								inner.if_condition_true = top_node_list_size;
								current_node_index = top_node_list_size;
							}
							CurrentlyEditedPartOfConditional::ConditionTrue => {
								inner.currently_edited_part = CurrentlyEditedPartOfConditional::ConditionFalse;
								let new_node = TreeNode {
									inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
										key: String::from(""),
										parameter: Some(Parameter::String(token.text)),
										is_editing_parameter: true,
									}),
									parent: Some(current_node_index),
								};
								top_node_list.push(new_node);
								inner.if_condition_false = Some(top_node_list_size);
								current_node_index = top_node_list_size;
							}
							CurrentlyEditedPartOfConditional::ConditionFalse => {
								eprintln!("This should throw an error. Should.");
							}
						}
					}
				}
			}
			TokenType::StringLiteral => {
				match top_node_list[current_node_index].inner_node {
					NodeEntryType::Unconditional(mut inner) => {
						if inner.is_editing_parameter {
							match &mut inner.parameter {
								Some(param) => {
									match param {
										Parameter::String(text) => {
											text.push_str(&token.text);
										}
										Parameter::Nodes(child_nodes) => {
											let new_node = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::from("literal"),
													parameter: Some(Parameter::String(token.text)),
													is_editing_parameter: true,
												}),
												parent: Some(current_node_index),
											};
											child_nodes.push(top_node_list_size);
											top_node_list.push(new_node);
											current_node_index = top_node_list_size;
										}
									}
								}
								None => {
									panic!(format!("top_node_list[{}] has `is_editing_parameter` set to true but `parameter` field is `None`!", current_node_index)	);
								}
							}
						} else {
							inner.key.push_str(&token.text);
						}
					}
					NodeEntryType::Conditional(inner) => {
						unimplemented!();
					}
				}
			}
		}
	}
	return top_node_list;
}

#[derive(Debug)]
pub struct TreeNode {
	inner_node: NodeEntryType,
	parent: Option<Id>, //Pointer, except that it's a vector index instead of a memory address
}

#[derive(Debug)]
pub struct UnconditionalNodeEntry {
	key: String, //Cannot be ars code, as it would require getting opcodes on the fly. Could work with an interpreter tho
	parameter: Option<Parameter>, //String for literals, Nodes for variable values
	is_editing_parameter: bool,
}

#[derive(Debug)]
pub struct ConditionalNodeEntry {
	condition: Id,
	if_condition_true: Id,
	if_condition_false: Option<Id>,
	currently_edited_part: CurrentlyEditedPartOfConditional,
}

#[derive(Debug)]
pub enum CurrentlyEditedPartOfConditional {
	Condition,
	ConditionTrue,
	ConditionFalse,
}

#[derive(Debug)]
pub enum NodeEntryType {
	Conditional(ConditionalNodeEntry),
	Unconditional(UnconditionalNodeEntry),
}

#[derive(Debug)]
#[allow(dead_code)] //Linter suggests that neither of the two variants are constructed so it's silenced
pub enum Parameter {
	Nodes(Vec<Id>),
	String(String)
}