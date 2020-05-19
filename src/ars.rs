/**
 * Compiles ARS into bytecode defined in docs/
 * # Arguments
 * * `ars_string` - string containing ARS code
 * # Returns
 * `Vec<u8>` containing compiled code
 */
pub fn compile_ars(ars_string: String) -> Vec<String> {
	let mut compiled = Vec::<u8>::new();
	let mut split = Vec::<ARSTreeItem>::new();
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
		let param = String::from(""); //The rest of the content is considered to be the parameter
		let current_part = ARSTreeItem {
			key: key,
			parameter: TreeItemParameter::Text(param),
		};
	}
	return words;
}
/**
 * Splits the argument into a vector of keys
 * # Arguments
 * * `ars_string` - string containing ARS code
 */
fn split_into_keys(ars_string: String) -> Vec<String> {
	let mut current_word: String = String::new();
	let mut words = Vec::<String>::new();
	let mut opened_brackets: u8 = 0;
	//Split the input into keys in brackets
	for current_char in ars_string.chars() {
		if current_char == '{' {
			if opened_brackets != 0 {
				current_word.push(current_char);
			}
			opened_brackets += 1;
		} else if current_char == '}' {
			if opened_brackets != 0 {
				opened_brackets -= 1;
			}
			if opened_brackets == 0 {
				words.push(current_word);
				current_word = String::new();
			} else {
				current_word.push(current_char);
			}
		} else {
			current_word.push(current_char);
		}
	}
	return words;
}
struct ARSTreeItem {
	key: String,
	parameter: TreeItemParameter,
}
enum TreeItemParameter {
	Text(String),
	Keys(Vec<ARSTreeItem>),
}
