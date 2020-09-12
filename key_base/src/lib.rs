pub mod environment;

pub trait Key {
	fn get_key_info(&self) -> &KeyInfo;
	fn get_key_function(&self) -> fn(parameter: &Vec<String>, environment: &environment::Environment) -> String;
}

#[derive(Clone, Debug)]
pub struct KeyInfo {
	pub parameters_required: Vec<usize>,
	pub name: String,
	pub opcode: u8,
	pub allowed_key_names: Vec<String>,
}