pub fn create_ars_tree(ars_string: String) -> ARSTreeItem {
	let mut top_level_node = ARSTreeItem {
		key: ARSStringOrTree::Text(String::new()),
		parameter: ARSStringOrTree::Text(String::new()),
		parent_item: None,
	};
	let mut current_node = &mut top_level_node; //Reference to the currently edited node
	let mut current_part = &mut current_node.key; //Reference to the currently edited string
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
pub struct ARSTreeItem {
	key: ARSStringOrTree,
	parameter: ARSStringOrTree,
	parent_item: Option<Box<ARSTreeItem>>,
}

#[derive(Debug)]
enum ARSStringOrTree {
	Text(String),
	Keys(Vec<ARSTreeItem>),
}
