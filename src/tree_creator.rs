#[allow(clippy::needless_return)]
use crate::tokenizer;

type Id = usize;

//TODO: refactor
#[allow(dead_code)]
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
		let mut nodes_to_push = Vec::new();
		let top_node_list_size = top_node_list.len(); //satisfying the borrow checker
		use tokenizer::TokenType;
		match token.token_type {
			TokenType::OpenBracket => {
				match top_node_list[current_node_index].inner_node {
					NodeEntryType::Unconditional(ref mut inner) => {
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
											nodes_to_push.push(new_node);
											current_node_index = top_node_list_size;
										}
										Parameter::String(text) => {
											let mut child_nodes = vec![ //Children IDs
												top_node_list_size,
											];
											current_node_index = top_node_list_size;
											if text.is_empty() {
												//Empty parameter
												let new_node = TreeNode {
													inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
														key: String::new(),
														parameter: None,
														is_editing_parameter: false,
													}),
													parent: Some(current_node_index - 1),
												};
												nodes_to_push.push(new_node);
											} else {
												//Not empty parameter
												let node_from_text = TreeNode {
													inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
														key: String::from("literal"),
														parameter: Some(Parameter::String(text.to_string())),
														is_editing_parameter: true,
													}),
													parent: Some(current_node_index - 1),
												};
												nodes_to_push.push(node_from_text);
												child_nodes.push(top_node_list_size + 1);
												current_node_index += 1;
												let new_node = TreeNode {
													inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
														key: String::new(),
														parameter: None,
														is_editing_parameter: false,
													}),
													parent: Some(current_node_index - 2),
												};
												nodes_to_push.push(new_node);
											}
											inner.parameter = Some(Parameter::Nodes(child_nodes));
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
									nodes_to_push.push(new_node);
									current_node_index = top_node_list_size;
								}
							}
						} else {
							inner.key.push_str(&token.text);
						}
					}
					NodeEntryType::Conditional(ref mut inner) => {
						match inner.currently_edited_part {
							CurrentlyEditedPartOfConditional::Condition => {
								match inner.condition {
									Parameter::Nodes(ref mut nodes) => {
										let new_node = TreeNode {
											inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
												key: String::new(),
												parameter: None,
												is_editing_parameter: false,
											}),
											parent: Some(current_node_index),
										};
										nodes_to_push.push(new_node);
										current_node_index = top_node_list_size + 1;
										nodes.push(current_node_index);
									}
									Parameter::String(ref mut text) => {
										let mut child_nodes = vec![ //Children IDs
											top_node_list_size,
										];
										current_node_index = top_node_list_size;
										if text.is_empty() {
											//Empty parameter
											let new_node = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::new(),
													parameter: None,
													is_editing_parameter: false,
												}),
												parent: Some(current_node_index - 1),
											};
											nodes_to_push.push(new_node);
										} else {
											//Not empty parameter
											let node_from_text = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::from("literal"),
													parameter: Some(Parameter::String(text.to_string())),
													is_editing_parameter: true,
												}),
												parent: Some(current_node_index - 1),
											};
											nodes_to_push.push(node_from_text);
											child_nodes.push(top_node_list_size + 1);
											current_node_index += 1;
											let new_node = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::new(),
													parameter: None,
													is_editing_parameter: false,
												}),
												parent: Some(current_node_index - 2),
											};
											nodes_to_push.push(new_node);
										}
										inner.condition = Parameter::Nodes(child_nodes);
									}
								}
							}
							CurrentlyEditedPartOfConditional::ConditionTrue => {
								match inner.if_condition_true {
									Parameter::Nodes(ref mut nodes) => {
										let new_node = TreeNode {
											inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
												key: String::new(),
												parameter: None,
												is_editing_parameter: false,
											}),
											parent: Some(current_node_index),
										};
										nodes_to_push.push(new_node);
										current_node_index = top_node_list_size + 1;
										nodes.push(current_node_index);
									}
									Parameter::String(ref mut text) => {
										let mut child_nodes = vec![ //Children IDs
											top_node_list_size,
										];
										current_node_index = top_node_list_size;
										if text.is_empty() {
											//Empty parameter
											let new_node = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::new(),
													parameter: None,
													is_editing_parameter: false,
												}),
												parent: Some(current_node_index - 1),
											};
											nodes_to_push.push(new_node);
										} else {
											//Not empty parameter
											let node_from_text = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::from("literal"),
													parameter: Some(Parameter::String(text.to_string())),
													is_editing_parameter: true,
												}),
												parent: Some(current_node_index - 1),
											};
											nodes_to_push.push(node_from_text);
											child_nodes.push(top_node_list_size + 1);
											current_node_index += 1;
											let new_node = TreeNode {
												inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
													key: String::new(),
													parameter: None,
													is_editing_parameter: false,
												}),
												parent: Some(current_node_index - 2),
											};
											nodes_to_push.push(new_node);
										}
										inner.if_condition_true = Parameter::Nodes(child_nodes);
									}
								}
							}
							CurrentlyEditedPartOfConditional::ConditionFalse => {
								match inner.if_condition_false {
									Some(ref mut value) => {
										match value {
											Parameter::Nodes(ref mut nodes) => {
												let new_node = TreeNode {
													inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
														key: String::new(),
														parameter: None,
														is_editing_parameter: false,
													}),
													parent: Some(current_node_index),
												};
												nodes_to_push.push(new_node);
												current_node_index = top_node_list_size + 1;
												nodes.push(current_node_index);
											}
											Parameter::String(ref mut text) => {
												let mut child_nodes = vec![ //Children IDs
													top_node_list_size,
												];
												current_node_index = top_node_list_size;
												if text.is_empty() {
													//Empty parameter
													let new_node = TreeNode {
														inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
															key: String::new(),
															parameter: None,
															is_editing_parameter: false,
														}),
														parent: Some(current_node_index - 1),
													};
													nodes_to_push.push(new_node);
												} else {
													//Not empty parameter
													let node_from_text = TreeNode {
														inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
															key: String::from("literal"),
															parameter: Some(Parameter::String(text.to_string())),
															is_editing_parameter: true,
														}),
														parent: Some(current_node_index - 1),
													};
													nodes_to_push.push(node_from_text);
													child_nodes.push(top_node_list_size + 1);
													current_node_index += 1;
													let new_node = TreeNode {
														inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
															key: String::new(),
															parameter: None,
															is_editing_parameter: false,
														}),
														parent: Some(current_node_index - 2),
													};
													nodes_to_push.push(new_node);
												}
												inner.if_condition_false = Some(Parameter::Nodes(child_nodes));
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
											parent: Some(current_node_index - 1),
										};
										nodes_to_push.push(new_node);
										inner.if_condition_false = Some(Parameter::Nodes(vec![current_node_index]));
									}
								}
							}
						}
					}
				}
			}
			TokenType::CloseBracket => {
				if let Some(parent_node) = top_node_list[current_node_index].parent {
					current_node_index = parent_node;
				} //TODO: adding the bracket as string if no parent
			}
			TokenType::ParameterDelimiter => {
				match top_node_list[current_node_index].inner_node {
					NodeEntryType::Unconditional(ref mut inner) => {
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
											nodes_to_push.push(new_node);
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
					NodeEntryType::Conditional(ref mut inner) => {
						match inner.currently_edited_part {
							CurrentlyEditedPartOfConditional::Condition => {
								inner.currently_edited_part = CurrentlyEditedPartOfConditional::ConditionTrue;
							}
							CurrentlyEditedPartOfConditional::ConditionTrue => {
								inner.currently_edited_part = CurrentlyEditedPartOfConditional::ConditionFalse;
							}
							CurrentlyEditedPartOfConditional::ConditionFalse => {
								eprintln!("This will error");
							}
						}
					}
				}
			}
			TokenType::StringLiteral => {
				match top_node_list[current_node_index].inner_node {
					NodeEntryType::Unconditional(ref mut inner) => {
						//TODO: change to conditional if needed
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
											nodes_to_push.push(new_node);
											current_node_index = top_node_list_size + 1;
										}
									}
								}
								None => {
									inner.parameter = Some(Parameter::String(token.text));
								}
							}
						} else {
							inner.key.push_str(&token.text);
						}
					}
					NodeEntryType::Conditional(ref mut inner) => {
						match inner.currently_edited_part {
							CurrentlyEditedPartOfConditional::Condition => {
								match inner.condition {
									Parameter::Nodes(ref mut nodes) => {
										let new_node = TreeNode {
											inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
												key: String::from("literal"),
												parameter: Some(Parameter::String(token.text)),
												is_editing_parameter: true,
											}),
											parent: Some(current_node_index),
										};
										nodes.push(top_node_list_size);
										nodes_to_push.push(new_node);
										current_node_index = top_node_list_size + 1;
									}
									Parameter::String(ref mut text) => {
										text.push_str(&token.text);
									}
								}
							}
							CurrentlyEditedPartOfConditional::ConditionTrue => {
								match inner.if_condition_true {
									Parameter::Nodes(ref mut nodes) => {
										let new_node = TreeNode {
											inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
												key: String::from("literal"),
												parameter: Some(Parameter::String(token.text)),
												is_editing_parameter: true,
											}),
											parent: Some(current_node_index),
										};
										nodes.push(top_node_list_size);
										nodes_to_push.push(new_node);
										current_node_index = top_node_list_size + 1;
									}
									Parameter::String(ref mut text) => {
										text.push_str(&token.text);
									}
								}
							}
							CurrentlyEditedPartOfConditional::ConditionFalse => {
								match inner.if_condition_false {
									Some(ref mut value) => {
										match value {
											Parameter::Nodes(ref mut nodes) => {
												let new_node = TreeNode {
													inner_node: NodeEntryType::Unconditional(UnconditionalNodeEntry {
														key: String::from("literal"),
														parameter: Some(Parameter::String(token.text)),
														is_editing_parameter: true,
													}),
													parent: Some(current_node_index),
												};
												nodes.push(top_node_list_size);
												nodes_to_push.push(new_node);
												current_node_index = top_node_list_size + 1;
											}
											Parameter::String(ref mut text) => {
												text.push_str(&token.text);
											}
										}
									}
									None => {
										inner.if_condition_false = Some(Parameter::String(token.text));
									}
								}
							}
						}
					}
				}
			}
		}
		top_node_list.append(&mut nodes_to_push);
	}
	return top_node_list;
}

#[derive(Debug, PartialEq)]
pub struct TreeNode {
	inner_node: NodeEntryType,
	parent: Option<Id>, //Pointer, except that it's a vector index instead of a memory address
}

#[derive(Debug, PartialEq)]
pub struct UnconditionalNodeEntry {
	key: String, //Cannot be ars code, as it would require getting opcodes on the fly. Could work with an interpreter tho
	parameter: Option<Parameter>, //String for literals, Nodes for variable values
	is_editing_parameter: bool,
}

#[derive(Debug, PartialEq)]
pub struct ConditionalNodeEntry {
	condition: Parameter,
	if_condition_true: Parameter,
	if_condition_false: Option<Parameter>,
	currently_edited_part: CurrentlyEditedPartOfConditional,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum CurrentlyEditedPartOfConditional {
	Condition,
	ConditionTrue,
	ConditionFalse,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum NodeEntryType {
	Conditional(ConditionalNodeEntry),
	Unconditional(UnconditionalNodeEntry),
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)] //Linter suggests that neither of the two variants are constructed so it's silenced
pub enum Parameter {
	Nodes(Vec<Id>),
	String(String)
}

//A wall of text is incoming. You probably should collapse them
#[cfg(test)]
mod tests {
	use super::*;
	use NodeEntryType::*;
	#[test]
	fn tree_small_nesting() {
		let tested_string = String::from("{abc:{def:ghi}}");
		let correct = vec![
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("top"),
						parameter: Some(
							Parameter::Nodes(
								vec![
									1,
								],
							),
						),
						is_editing_parameter: true,
					},
				),
				parent: None,
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("abc"),
						parameter: Some(
							Parameter::Nodes(
								vec![
									2,
								],
							),
						),
						is_editing_parameter: true,
					},
				),
				parent: Some(
					0,
				),
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("def"),
						parameter: Some(
							Parameter::String(
								String::from("ghi"),
							),
						),
						is_editing_parameter: true,
					},
				),
				parent: Some(
					1,
				),
			},
		];
		let output = create_ars_tree(tested_string);
		assert_eq!(output, correct);
	}
	#[test]
	fn tree_no_nesting_only_keys() {
		let input = String::from("{abc}{def}{ghi}");
		let correct = vec![
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("top"),
						parameter: Some(
							Parameter::Nodes(
								vec![
									1,
									2,
									3,
								],
							),
						),
						is_editing_parameter: true,
					},
				),
				parent: None,
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("abc"),
						parameter: None,
						is_editing_parameter: false,
					},
				),
				parent: Some(
					0,
				),
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("def"),
						parameter: None,
						is_editing_parameter: false,
					},
				),
				parent: Some(
					0,
				),
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("ghi"),
						parameter: None,
						is_editing_parameter: false,
					},
				),
				parent: Some(
					0,
				),
			},
		];
		let output = create_ars_tree(input);
		assert_eq!(output, correct);
	}
	#[test]
	fn tree_nesting() {
		let input = String::from("abc{def}{ghi:jkm}{abc:{def:ghi}}");
		let correct_output = vec![
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("top"),
						parameter: Some(
							Parameter::Nodes(
								vec![
									1,
									2,
									3,
									4,
								],
							),
						),
						is_editing_parameter: true,
					},
				),
				parent: None,
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("literal"),
						parameter: Some(
							Parameter::String(
								String::from("abc"),
							),
						),
						is_editing_parameter: true,
					},
				),
				parent: Some(
					0,
				),
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("def"),
						parameter: None,
						is_editing_parameter: false,
					},
				),
				parent: Some(
					0,
				),
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("ghi"),
						parameter: Some(
							Parameter::String(
								String::from("jkm"),
							),
						),
						is_editing_parameter: true,
					},
				),
				parent: Some(
					0,
				),
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("abc"),
						parameter: Some(
							Parameter::Nodes(
								vec![
									5,
								],
							),
						),
						is_editing_parameter: true,
					},
				),
				parent: Some(
					0,
				),
			},
			TreeNode {
				inner_node: Unconditional(
					UnconditionalNodeEntry {
						key: String::from("def"),
						parameter: Some(
							Parameter::String(
								String::from("ghi"),
							),
						),
						is_editing_parameter: true,
					},
				),
				parent: Some(
					4,
				),
			},
		];
		let output = create_ars_tree(input);
		assert_eq!(output, correct_output);
	}
}