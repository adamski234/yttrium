use std::fs;
use key_base;

//TODO: document this being a raw pointer. VERY IMPORTANT
type key_create_function = fn() -> *mut key_base::Key;
const KEY_CREATE_FUNCTION_NAME: &[u8] = b"key_create";

//TODO actually implement loading. This is currently just a placeholder
pub fn load_keys(directory: &str) -> Vec<Box<dyn key_base::Key>> {
	if directory.is_empty() {
		panic!("Key location directory was empty");
	}
	let files = fs::read_dir(directory).expect("Key directory did not exist");
	let mut keys = Vec::<Box<dyn key_base::Key>>::new();
	for file in files {
		let file = file.expect("Error while reading file");
		let library = libloading::Library::new(file.path()).expect(&format!("Error while opening library at `{}`", file.path().to_str().unwrap()));
		unsafe {
			let creator_function: libloading::Symbol<key_create_function> = library.get(KEY_CREATE_FUNCTION_NAME).expect(&format!(
				"Library at `{}` had no {} function",
				file.path().to_str().unwrap(),
				String::from_utf8(KEY_CREATE_FUNCTION_NAME.into()).unwrap())
			);
			keys.push(Box::from_raw(creator_function()));
		}
	}
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
	if keys.is_empty() {
		panic!("No valid keys were found in the key directory");
	}
	return keys;
}