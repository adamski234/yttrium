/// `split_into_tokens` takes a string of valid or invalid ARS code and separates it into a vector of tokens
#[allow(dead_code)]
pub fn split_into_tokens(ars_string: String) -> Vec<Token> {
	let mut output = Vec::new();
	let mut current_string = String::with_capacity(50);
	let mut count = 0;
	for current_char in ars_string.chars() {
		count += 1;
		match current_char {
			'{' => {
				if !current_string.is_empty() {
					output.push(Token {
						text: current_string,
						token_type: TokenType::StringLiteral,
					});
					current_string = String::with_capacity(50);
				}
				output.push(Token {
					text: current_char.to_string(),
					token_type: TokenType::OpenBracket,
				});
			}
			'}' => {
				if !current_string.is_empty() {
					output.push(Token {
						text: current_string,
						token_type: TokenType::StringLiteral,
					});
					current_string = String::with_capacity(50);
				}
				output.push(Token {
					text: current_char.to_string(),
					token_type: TokenType::CloseBracket,
				})
			}
			':' => {
				if !current_string.is_empty() {
					output.push(Token {
						text: current_string,
						token_type: TokenType::StringLiteral,
					});
					current_string = String::with_capacity(50);
				}
				output.push(Token {
					text: current_char.to_string(),
					token_type: TokenType::ParameterDelimiter,
				})
			}
			_ => {
				current_string.push(current_char);
			}
		}
		if count == ars_string.len() && !current_string.is_empty() {
			output.push(Token {
				text: current_string,
				token_type: TokenType::StringLiteral,
			});
			current_string = String::with_capacity(50);
		}
	}
	return output;
}

#[derive(Debug)]
pub struct Token {
	text: String,
	token_type: TokenType
}

#[allow(dead_code)]
#[derive(Debug)]
enum TokenType {
	OpenBracket,
	CloseBracket,
	StringLiteral,
	ParameterDelimiter,
}