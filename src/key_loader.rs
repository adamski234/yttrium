use key_base;

//TODO actually implement loading. This is currently just a placeholder
pub fn load_keys(directory: &str) -> Vec<Box<dyn key_base::Key>> {
	let mut keys = Vec::<Box<dyn key_base::Key>>::new();
	//Verification and information step
	let mut to_remove = Vec::new();
	for (index, key) in keys.iter().enumerate() {
		let key_info = key.get_key_info();
		println!("Loaded key {}", key_info.name);
		if key_info.parameters_required.is_empty() {
			to_remove.push(index - to_remove.len());
			println!("Key {} had `parameters_required` empty", key_info.name);
		} else if !key_info.parameters_required.is_sorted() {
			to_remove.push(index - to_remove.len());
			println!("Key {} had `parameters_required` not sorted", key_info.name);
		} else if key_info.parameters_required.len() == 1 && key_info.parameters_required[0] != 0 && key_info.allowed_key_names.len() == 0 {
			to_remove.push(index - to_remove.len());
			println!("Key {} had `allowed_key_names` empty", key_info.name);
		}
	}
	for index in to_remove {
		keys.remove(index);
	}
	return keys;
}