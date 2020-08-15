use key_base;

//TODO actually implement loading. This is currently just a placeholder
pub fn load_keys(directory: &str) -> Vec<Box<dyn key_base::Key>> {
	let keys = Vec::new();
	return keys;
}

struct Key1 {
	function: fn(parameter: &String) -> bool,
	info: key_base::KeyInfo,
}

impl key_base::Key for Key1 {
	fn get_key_info(&self) -> key_base::KeyInfo {
		return self.info.clone();
	}
	fn get_key_function(&self) -> fn(parameter: &String) -> bool {
		return self.function;
	}
}