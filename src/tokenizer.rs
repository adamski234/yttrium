#[allow(clippy::needless_return)]
/// `split_into_tokens` takes a string of valid or invalid ARS code and separates it into a vector of tokens
#[allow(dead_code)]
pub fn split_into_tokens(ars_string: String) -> Vec<Token> {
	let mut output = Vec::new();
	let mut current_string = String::with_capacity(50);
	for (count, current_char) in ars_string.chars().enumerate() {
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
		if count == ars_string.len() - 1 && !current_string.is_empty() {
			output.push(Token {
				text: current_string,
				token_type: TokenType::StringLiteral,
			});
			current_string = String::with_capacity(50);
		}
	}
	return output;
}

#[derive(Debug, PartialEq)]
pub struct Token {
	pub text: String,
	pub token_type: TokenType
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
	OpenBracket,
	CloseBracket,
	StringLiteral,
	ParameterDelimiter,
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn tokenizer_test_correct() {
		let input = String::from("abc{def}{ghi:jkm}");
		let output = split_into_tokens(input);
		let correct_output = vec![
			Token {
				text: String::from("abc"),
				token_type: TokenType::StringLiteral,
			},
			Token {
				text: String::from("{"),
				token_type: TokenType::OpenBracket,
			},
			Token {
				text: String::from("def"),
				token_type: TokenType::StringLiteral,
			},
			Token {
				text: String::from("}"),
				token_type: TokenType::CloseBracket,
			},
			Token {
				text: String::from("{"),
				token_type: TokenType::OpenBracket,
			},
			Token {
				text: String::from("ghi"),
				token_type: TokenType::StringLiteral,
			},
			Token {
				text: String::from(":"),
				token_type: TokenType::ParameterDelimiter,
			},
			Token {
				text: String::from("jkm"),
				token_type: TokenType::StringLiteral,
			},
			Token {
				text: String::from("}"),
				token_type: TokenType::CloseBracket,
			},
		];
		assert_eq!(output, correct_output);
	}
}