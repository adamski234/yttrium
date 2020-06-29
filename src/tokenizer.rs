#[allow(clippy::needless_return)]
/// `split_into_tokens` takes a string of valid or invalid ARS code and separates it into a vector of tokens
#[allow(dead_code)]
pub fn split_into_tokens(ars_string: String) -> Vec<Token> {
	const DEFAULT_STR_LENGTH: usize = 20;
	let mut output = Vec::new();
	let mut current_string = String::with_capacity(DEFAULT_STR_LENGTH);
	let mut is_backslashed = false;
	for (count, current_char) in ars_string.chars().enumerate() {
		match current_char {
			'{' => {
				if is_backslashed {
					current_string.push(current_char);
					is_backslashed = false;
				} else {
					if !current_string.is_empty() {
						output.push(Token {
							text: current_string,
							token_type: TokenType::StringLiteral,
						});
						current_string = String::with_capacity(DEFAULT_STR_LENGTH);
					}
					output.push(Token {
						text: current_char.to_string(),
						token_type: TokenType::OpenBracket,
					});
				}
			}
			'}' => {
				if is_backslashed {
					current_string.push(current_char);
					is_backslashed = false;
				} else {
					if !current_string.is_empty() {
						output.push(Token {
							text: current_string,
							token_type: TokenType::StringLiteral,
						});
						current_string = String::with_capacity(DEFAULT_STR_LENGTH);
					}
					output.push(Token {
						text: current_char.to_string(),
						token_type: TokenType::CloseBracket,
					});
				}
			}
			'(' => {
				if is_backslashed {
					current_string.push(current_char);
					is_backslashed = false;
				} else {
					if !current_string.is_empty() {
						output.push(Token {
							text: current_string,
							token_type: TokenType::StringLiteral,
						});
						current_string = String::with_capacity(DEFAULT_STR_LENGTH);
					}
					output.push(Token {
						text: current_char.to_string(),
						token_type: TokenType::OpenParentheses,
					});
				}
			}
			')' => {
				if is_backslashed {
					current_string.push(current_char);
					is_backslashed = false;
				} else {
					if !current_string.is_empty() {
						output.push(Token {
							text: current_string,
							token_type: TokenType::StringLiteral,
						});
						current_string = String::with_capacity(DEFAULT_STR_LENGTH);
					}
					output.push(Token {
						text: current_char.to_string(),
						token_type: TokenType::CloseParentheses,
					});
				}
			}
			':' => {
				if is_backslashed {
					current_string.push(current_char);
					is_backslashed = false;
				} else {
					if !current_string.is_empty() {
						output.push(Token {
							text: current_string,
							token_type: TokenType::StringLiteral,
						});
						current_string = String::with_capacity(DEFAULT_STR_LENGTH);
					}
					output.push(Token {
						text: current_char.to_string(),
						token_type: TokenType::ParameterDelimiter,
					});
				}
			}
			'\\' => {
				if is_backslashed {
					current_string.push(current_char);
					is_backslashed = false;
				} else {
					is_backslashed = true;
				}
			}
			_ => {
				if is_backslashed {
					current_string.push('\\');
					is_backslashed = false;
				}
				current_string.push(current_char);
			}
		}
		if count == ars_string.len() - 1 && !current_string.is_empty() {
			output.push(Token {
				text: current_string,
				token_type: TokenType::StringLiteral,
			});
			current_string = String::with_capacity(DEFAULT_STR_LENGTH);
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
	OpenParentheses,
	CloseParentheses
}

//110 lines of tests begin
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
	#[test]
	fn tokenizer_backslashes() {
		let input = String::from("\\{a\\bc\\}\\\\{def}");
		let output = split_into_tokens(input);
		let correct_output = vec![
			Token {
				text: String::from("{a\\bc}\\"),
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
		];
		assert_eq!(output, correct_output);
	}
	#[test]
	fn tokenizer_parentheses() {
		let input = String::from("({abc})\\()");
		let output = split_into_tokens(input);
		let correct_output = vec![
			Token {
				text: String::from("("),
				token_type: TokenType::OpenParentheses,
			},
			Token {
				text: String::from("{"),
				token_type: TokenType::OpenBracket,
			},
			Token {
				text: String::from("abc"),
				token_type: TokenType::StringLiteral,
			},
			Token {
				text: String::from("}"),
				token_type: TokenType::CloseBracket,
			},
			Token {
				text: String::from(")"),
				token_type: TokenType::CloseParentheses,
			},
			Token {
				text: String::from("("),
				token_type: TokenType::StringLiteral,
			},
			Token {
				text: String::from(")"),
				token_type: TokenType::CloseParentheses,
			},
		];
	}
}