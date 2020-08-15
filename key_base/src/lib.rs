pub trait Key {
	fn get_key_info(&self) -> KeyInfo;
	fn get_key_function(&self) -> fn(parameter: &String) -> bool;
}

#[derive(Clone)]
pub struct KeyInfo {
	is_parameter_required: bool,
	name: String,
}