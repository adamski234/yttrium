mod ars;

use std::io::stdin;

fn main() {
	loop {
		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("An error has happened while reading from the console");
		println!("{:?}", ars::compile_ars(input.clone()));
	}
}
