use std::fs;
use std::collections::HashMap;

const DATABASE_DIR: &str = "./databases/";

#[derive(Debug)]
pub struct DatabaseManager {
	pub guild_id: String,
}

//This will be an issue with multiple people trying to write to a single database at the same time
impl DatabaseManager {
	pub fn new(guild_id: String) -> Self {
		//Converts a json file to a HashMap<String, Database>
		match fs::read(format!("{}{}.json", DATABASE_DIR, guild_id)) {
			Ok(value) => {
				//Convert the file
				let values: serde_json::Value = serde_json::from_str(&String::from_utf8(value).unwrap()).unwrap();
				match values {
					serde_json::Value::Object(value) => {
						let mut map = HashMap::with_capacity(value.len());
						for (db_name, db_value) in value.into_iter() {
							map.insert(db_name, Database::new_from_value(db_value));
						}
					}
					_ => {
						panic!("Top level item of {}.json was not an object", guild_id);
					}
				}
			}
			Err(error) => {
				//Create a new JSON file if it doesn't exist
			}
		}
		return Self { guild_id };
	}
	pub fn get_database(&mut self, name: String) -> Option<&mut Database> {
		todo!();
	}
	pub fn create_database(&mut self, name: String) -> &mut Database {
		todo!();
	}
	pub fn write(&self) {
		todo!();
	}
}

#[derive(Debug, PartialEq)]
pub struct Database {
	pub values: HashMap<String, StringOrArray>,
}

impl Database {
	//TODO: Major!! Remove the panics and just ignore invalid values
	pub fn new_from_value(value: serde_json::Value) -> Self {
		let mut result = Self {
			values: HashMap::new(),
		};
		match value {
			serde_json::Value::Object(object) => {
				let mut map = HashMap::with_capacity(object.len());
				for (name, value) in object {
					match value {
						serde_json::Value::String(text) => {
							map.insert(name, StringOrArray::String(text));
						}
						serde_json::Value::Array(array) => {
							let mut values = Vec::with_capacity(array.len());
							for entry in array {
								match entry {
									serde_json::Value::String(text) => {
										values.push(text);
									}
									_ => {
										panic!("Array didn't only contain strings");
									}
								};
							}
							map.insert(name, StringOrArray::Array(values));
						}
						_ => {
							panic!("Database entry was not a string nor array of strings");
						}
					};
				}
				result.values = map;
			}
			_ => {
				panic!("new_from_value: value passed wasn't an object");
			}
		};
		return result;
	}
	//TODO: Arrays?
	pub fn get_key(&self, name: String) -> Option<String> {
		todo!();
	}
	pub fn write_key(&mut self, name: String, value: String) {
		todo!();
	}
}

#[derive(Debug, PartialEq)]
pub enum StringOrArray {
	String(String),
	Array(Vec<String>),
}

#[cfg(test)]
mod tests {
	#[test]
	fn simple_database() {
		use crate::databases::*;
		let input = r#"{"string_value": "string", "array_value": ["entry1", "entry2"]} "#;
		let mut correct_output = Database {
			values: HashMap::new(),
		};
		correct_output.values.insert(String::from("string_value"), StringOrArray::String(String::from("string")));
		correct_output.values.insert(String::from("array_value"), StringOrArray::Array(vec![String::from("entry1"), String::from("entry2")]));
		let output = Database::new_from_value(serde_json::from_str(input).unwrap());
		assert_eq!(correct_output, output);
	}
}