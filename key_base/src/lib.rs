pub trait Key {
	fn get_key_info(&self) -> KeyInfo;
	fn get_key_function(&self) -> fn(parameter: &String) -> bool;
}

#[derive(Clone)]
pub struct KeyInfo {
	parameters_required: Vec<usize>,
	name: String,
	opcode: u8,
	allowed_key_names: Vec<String>,
}