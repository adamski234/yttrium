use key_base;

//TODO actually implement loading. This is currently just a placeholder
//Also, make this a hashmap
pub fn load_keys(directory: &str) -> Vec<Box<dyn key_base::Key>> {
	//let keys = Vec::new();
	let mut keys = Vec::<Box<dyn key_base::Key>>::new();
	keys.push(
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![0],
				name: String::from("abc"),
				opcode: 0,
				allowed_key_names: vec![],
			}
		})
	);
	keys.push(
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![1],
				name: String::from("def"),
				opcode: 0,
				allowed_key_names: vec![],
			}
		})
	);
	keys.push(
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![0, 1, 3],
				name: String::from("ghi"),
				opcode: 0,
				allowed_key_names: vec![],
			}
		})
	);
	keys.push(
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![],
				name: String::from("jkm"),
				opcode: 0,
				allowed_key_names: vec![],
			}
		})
	);
	keys.push(
		Box::new(Key1 {
			function: placeholder_fn,
			info: key_base::KeyInfo {
				parameters_required: vec![2],
				name: String::from("ab"),
				opcode: 0,
				allowed_key_names: vec![],
			}
		})
	);
	//Verification and information step
	let mut to_remove = Vec::new();
	for (index, key) in keys.iter().enumerate() {
		let key_info = key.get_key_info();
		println!("Loaded key {}", key_info.name);
		if key_info.parameters_required.is_empty() {
			to_remove.push(index - to_remove.len());
			println!("Key {} had `parameters_required` empty", key_info.name);
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
fn placeholder_fn(_param: &String) -> bool {
	return true;
}