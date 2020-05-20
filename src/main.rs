mod ars;

fn main() {
	println!("{:?}", ars::compile_ars(String::from("123{abc}{def:ghi}{jkl:{mno}}")));
}