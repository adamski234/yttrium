use rand;
use std::fs;
use std::io::Write;

fn main() {
	let estimated_file_len = 1_000_000;
	let mut output = String::with_capacity(estimated_file_len + 1000);
	let mut file = fs::File::create("./longars.txt").unwrap();
	loop {
		output.push_str(&generate_random());
		if output.len() >= estimated_file_len {
			break;
		}
	}
	file.write_all(output.as_bytes()).unwrap();
	println!("Done, with size of {}", output.len());
}
fn generate_random() -> String {
	let random = rand::random::<usize>();
	if random % 3 == 0 {
		//generate normal
		return generate_normal();
	} else if random % 3 == 1 {
		//generate conditional
		return generate_conditional();
	} else {
		//A random string
		return String::from("a");
	}
}
fn generate_normal() -> String {
	if rand::random::<usize>() % 2 == 0 {
		//will not have a parameter
		return String::from("{a}");
	} else {
		//will have a parameter
		let parameter = generate_random();
		return format!("{{a:{}}}", parameter);
	}
}
fn generate_conditional() -> String{
	if rand::random::<usize>() % 2 == 0 {
		//will not have a false condition
		return format!("{{cond:{}:{}}}", generate_random(), generate_random());
	} else {
		//will have a false condition
		return format!("{{cond:{}:{}:{}}}", generate_random(), generate_random(), generate_random());
	}
}