fn main() {
	println!("{:?}", ars::compile_ars(String::from("{abc}{def}{efg{ghy}}")));
}

pub mod ars {
	pub fn compile_ars(ars_string: String) -> Vec<String> {
		let mut compiled = Vec::<u8>::new();
		/* Compilation steps
		1. No checking for syntax correctness - that would crash with unclosed brackets in text
		2. Go through each character
		3. Find the key in the database and append the opcode and parameters to `compiled`
		*/
		let words = split_into_keys(ars_string);
		for word in words.iter() {
			//
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
				opened_brackets -= 1;
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
}
