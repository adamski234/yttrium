#[allow(clippy::needless_return)]

use std::sync::mpsc;
/// `split_into_tokens` takes a string of valid or invalid ARS code and separates it into a vector of tokens
#[allow(dead_code)]
pub fn split_into_tokens(ars_string: String, sender: mpsc::Sender<Token>) {
	let mut current_string = String::with_capacity(50);
	for (count, current_char) in ars_string.chars().enumerate() {
		match current_char {
			'{' => {
				if !current_string.is_empty() {
					sender.send(Token {
						text: current_string,
						token_type: TokenType::StringLiteral,
					}).unwrap();
					current_string = String::with_capacity(50);
				}
				sender.send(Token {
					text: current_char.to_string(),
					token_type: TokenType::OpenBracket,
				}).unwrap();
			}
			'}' => {
				if !current_string.is_empty() {
					sender.send(Token {
						text: current_string,
						token_type: TokenType::StringLiteral,
					}).unwrap();
					current_string = String::with_capacity(50);
				}
				sender.send(Token {
					text: current_char.to_string(),
					token_type: TokenType::CloseBracket,
				}).unwrap();
			}
			':' => {
				if !current_string.is_empty() {
					sender.send(Token {
						text: current_string,
						token_type: TokenType::StringLiteral,
					}).unwrap();
					current_string = String::with_capacity(50);
				}
				sender.send(Token {
					text: current_char.to_string(),
					token_type: TokenType::ParameterDelimiter,
				}).unwrap();
			}
			_ => {
				current_string.push(current_char);
			}
		}
		if count == ars_string.len() - 1 && !current_string.is_empty() {
			sender.send(Token {
				text: current_string,
				token_type: TokenType::StringLiteral,
			}).unwrap();
			current_string = String::with_capacity(50);
		}
	}
}

#[derive(Debug)]
pub struct Token {
	pub text: String,
	pub token_type: TokenType
}

#[derive(Debug)]
pub enum TokenType {
	OpenBracket,
	CloseBracket,
	StringLiteral,
	ParameterDelimiter,
}