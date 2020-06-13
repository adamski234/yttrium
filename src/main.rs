#[allow(clippy::needless_return)] //I'm gonna use returns whether clippy likes it or not
mod lib;
mod tokenizer;
mod tree_creator;

use std::io::stdin;

fn main() {
	loop {
		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("An error has happened while reading from the console");
		println!("{:?}", tree_creator::create_ars_tree(input));
		//println!("{:?}", tokenizer::split_into_tokens(input));
	}
}
