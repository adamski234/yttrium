mod ars;

fn main() {
	println!("{:?}", ars::compile_ars(String::from("}}{abc}{def}{efg{ghy}}")));
}