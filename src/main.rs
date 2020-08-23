#![allow(clippy::needless_return)] //I'm gonna use returns whether clippy likes it or not
//use ars::errors_and_warns;
use ars::tokenizer;
use ars::tree_creator;
use ars::key_loader;

use std::io::stdin;
use std::io::Read;

fn main() {
	let key_list = key_loader::load_keys("abc");
	loop {
		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("An error has happened while reading from the console");
		println!("{:#?}", tree_creator::create_ars_tree(input, &key_list).unwrap().tree);
		//key_loader::load_keys("directory: &str");
	}
}
