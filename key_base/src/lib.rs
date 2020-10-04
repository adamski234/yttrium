pub mod environment;
pub mod embed;
pub mod databases;

pub trait Key {
	fn get_key_info(&self) -> &KeyInfo;
	fn get_key_function(&self) -> fn(parameter: &Vec<String>, environment: &mut environment::Environment) -> String;
}

#[derive(Clone, Debug)]
pub struct KeyInfo {
	pub parameters_required: Vec<usize>,
	pub name: String,
}