/**
 * Compiles ARS into bytecode defined in docs/
 * # Arguments
 * * `ars_string` - string containing ARS code
 * # Returns
 * `Vec<u8>` containing compiled code
 */
pub fn compile_ars(ars_string: String) -> Vec<String> {
	let mut compiled = Vec::<u8>::new();
	let mut tree_from_ars = Vec::<ARSTreeItem>::new();
	/* Compilation steps
	1. No checking for syntax correctness - that would crash with unclosed brackets in text
	2. Go through each character
	3. Find the key in the database and append the opcode and parameters to `compiled`
	*/
	let words = split_into_keys(ars_string); //Words is a Vec<String> containing first level parsed and split keys
	for word in words.iter() {
		if word.is_empty() {
			continue; //Disregard all empty strings
		}
		//Compilation happens here I think
		//TODO: Add '?' as the split character for database keys
		let split_parts_of_current: Vec<&str> = word.split(':').collect();
		let key = String::from(split_parts_of_current[0]); //First part of the content split by `:` is the key
		let param = String::from(split_parts_of_current[1..].join("")); //The rest of the content is considered to be the parameter
		let current_part = ARSTreeItem {
			key: key,
			parameter: ARSTreeItemParameter::Text(param),
		};
	}
	return words;
}
/**
 * Splits the argument into a vector of keys
 * # Arguments
 * * `ars_string` - string containing ARS code enclosed in brackets
 */
fn split_into_keys(ars_string: String) -> Vec<String> {
	let mut current_word = String::new();
	let mut words = Vec::<String>::new();
	let mut opened_brackets: u8 = 0;
	//To prevent text before the first bracket from being suffixed to the content of the first bracket
	//Split the input into keys with brackets
	for current_char in ars_string.chars() {
		if opened_brackets == 0 {
			if current_char == '{' {
				opened_brackets += 1;
				if !current_word.is_empty() {
					words.push(current_word.clone());
				}
				current_word = current_char.to_string();
			} else {
				current_word.push(current_char);
			}
		} else if opened_brackets == 1 {
			current_word.push(current_char);
			if current_char == '{' {
				opened_brackets += 1;
			} else if current_char == '}' {
				opened_brackets -= 1;
				if !current_word.is_empty() {
					words.push(current_word.clone());
				}
				current_word = String::new();
			}
		} else {
			current_word.push(current_char);
			if current_char == '{' {
				opened_brackets += 1;
			} else if current_char == '}' {
				opened_brackets -= 1;
			}
		}
	}
	return words;
}
struct ARSTreeItem {
	key: String,
	parameter: ARSTreeItemParameter,
}
enum ARSTreeItemParameter {
	Text(String),
	Keys(Vec<ARSTreeItem>),
}
