pub fn create_ars_tree<'a>(ars_string: String) -> ARSTreeItem<'a> {
	let mut top_level_node = ARSTreeItem {
		key: ARSStringOrTree::Keys(Vec::new()),
		parameter: ARSStringOrTree::Text(String::new()),
		parent_item: None,
	};
	let mut current_node = &top_level_node;
	let mut current_part = &current_node.key;
	let mut is_parameter = false;
	let mut index = 0;
	for current_char in ars_string.chars() {
		if current_char == '{' {
			//Handle new opening bracket
		} else if current_char == '}' {
			//Handle all closing brackets by adding the bracket and going up a level in the tree
		} else if current_char == ':' {
			//Ignore redundant switches
		} else {
			//Add the character to the output
		}
		//Handle unclosed brackets
	}
	return top_level_node;
}

#[derive(Debug)]
pub struct ARSTreeItem<'a> {
	key: ARSStringOrTree<'a>,
	parameter: ARSStringOrTree<'a>,
	parent_item: Option<&'a ARSTreeItem<'a>>,
}

#[derive(Debug)]
enum ARSStringOrTree<'a> {
	Text(String),
	Keys(Vec<ARSTreeItem<'a>>),
}

#[cfg(test)]
mod tests {
	//These tests probably won't ever be finished
	mod create_ars_tree_tests {
		use super::super::*;
		#[test]
		fn create_ars_tree_correct() {
			//Tests for a correctly formed string
			unimplemented!();
		}
	}
}
