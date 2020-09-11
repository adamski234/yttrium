use std::fs;
use key_base;
use std::collections::HashMap;

//TODO: document this being a raw pointer. VERY IMPORTANT
type KeyCreateFunction = fn() -> *mut dyn key_base::Key;
const KEY_CREATE_FUNCTION_NAME: &[u8] = b"key_create";

pub fn load_keys(directory: &str) -> Keys {
	let mut keys_vec = Vec::new();
	let mut keys = Keys {
		keys: HashMap::new(), //HashMap ordered by the key name
		libraries: Vec::new(),
	};
	if directory.is_empty() {
		panic!("Key location directory was empty");
	}
	let files = fs::read_dir(directory).expect("Key directory did not exist");
	for file in files {
		if file.is_err() {
			println!("Could not read file due to error:");
			println!("{}", file.unwrap_err());
			continue;
		}
		let file = file.unwrap();
		let library = libloading::Library::new(file.path());
		if library.is_err() {
			println!("Could not open library at `{}` due to error:", file.path().to_str().unwrap());
			println!("{}", library.unwrap_err());
			continue;
		}
		let library = library.unwrap();
		unsafe {
			let creator_function: Result<libloading::Symbol<KeyCreateFunction>, libloading::Error> = library.get(KEY_CREATE_FUNCTION_NAME);
			if creator_function.is_err() {
				println!(
					"Library at `{}` had no {} function",
					file.path().to_str().unwrap(),
					String::from_utf8(KEY_CREATE_FUNCTION_NAME.into()).unwrap()
				);
				continue;
			}
			let creator_function = creator_function.unwrap();
			keys_vec.push(Box::from_raw(creator_function()));
			keys.libraries.push(library);
		}
	}
	//Verification and information step
	let mut to_remove = Vec::new();
	for (index, key) in keys_vec.iter().enumerate() {
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
		keys_vec.remove(index);
	}
	if keys_vec.is_empty() {
		panic!("No valid keys were found in the key directory");
	}
	for key in keys_vec {
		let name = &key.get_key_info().name;
		keys.keys.insert(name.into(), key);
	}
	return keys;
}

pub struct Keys {
	pub keys: HashMap<String, Box<dyn key_base::Key>>,
	pub libraries: Vec<libloading::Library>,
}