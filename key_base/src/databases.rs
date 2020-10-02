use std::fs;
use std::io::Write;
use std::collections::HashMap;

const DATABASE_DIR: &str = "./databases/";

#[derive(Debug)]
pub struct DatabaseManager {
	pub guild_id: String,
	databases: HashMap<String, Database>,
}

//This will be an issue with multiple people trying to write to a single database at the same time
impl DatabaseManager {
	pub fn new(guild_id: &String) -> Self {
		//Converts a json file to a HashMap<String, Database>
		match fs::read(format!("{}{}.json", DATABASE_DIR, guild_id)) {
			Ok(content) => {
				//Convert the file
				let values: serde_json::Value = serde_json::from_str(&String::from_utf8(content).unwrap()).unwrap();
				return Self::new_from_value(values, guild_id);
			}
			Err(error) => {
				match error.kind() {
				    std::io::ErrorKind::NotFound => {
						//Create a new JSON file if it doesn't exist
						let mut file = fs::File::create(format!("{}{}.json", DATABASE_DIR, guild_id)).unwrap();
						file.write(b"{}").unwrap();
						let empty_manager = Self {
							guild_id: guild_id.clone(),
							databases: HashMap::new(),
						};
						return empty_manager;
					}
				    std::io::ErrorKind::PermissionDenied => {
						panic!("Permission denied on file {}{}.json", DATABASE_DIR, guild_id);
					}
					_ => {
						panic!(error);
					}
				}
			}
		}
	}
	pub fn new_from_json(json: &String, guild_id: &String) -> Self {
		return Self::new_from_value(serde_json::from_str(json).unwrap(), guild_id);
	}
	pub fn new_from_value(value: serde_json::Value, guild_id: &String) -> Self {
		let mut result = Self {
			guild_id: guild_id.clone(),
			databases: HashMap::new(),
		};
		match value {
			serde_json::Value::Object(value) => {
				for (db_name, db_value) in value.into_iter() {
					result.databases.insert(db_name, Database::new_from_value(db_value));
				}
			}
			_ => {
				panic!("Top level item of {}.json was not an object", guild_id);
			}
		}
		return result;
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
	pub fn new_from_value(value: serde_json::Value) -> Self {
		let mut result = Self {
			values: HashMap::new(),
		};
		match value {
			serde_json::Value::Object(object) => {
				for (name, value) in object {
					match value {
						serde_json::Value::String(text) => {
							result.values.insert(name, StringOrArray::String(text));
						}
						serde_json::Value::Array(array) => {
							let mut values = Vec::with_capacity(array.len());
							for entry in array {
								match entry {
									serde_json::Value::String(text) => {
										values.push(text);
									}
									serde_json::Value::Array(array) => {
										values.push(serde_json::to_string(&array).unwrap());
									}
									serde_json::Value::Bool(inner) => {
										values.push(inner.to_string());
									}
									serde_json::Value::Number(number) => {
										values.push(number.to_string());
									}
									serde_json::Value::Object(object) => {
										values.push(serde_json::to_string(&object).unwrap());
									}
									serde_json::Value::Null => {
										values.push(String::from("null"));
									}
								};
							}
							result.values.insert(name, StringOrArray::Array(values));
						}
						serde_json::Value::Bool(inner) => {
							result.values.insert(name, StringOrArray::String(inner.to_string()));
						}
						serde_json::Value::Number(number) => {
							result.values.insert(name, StringOrArray::String(number.to_string()));
						}
						serde_json::Value::Object(object) => {
							result.values.insert(name, StringOrArray::String(serde_json::to_string(&object).unwrap()));
						}
						serde_json::Value::Null => {
							result.values.insert(name, StringOrArray::String(String::from("null")));
						}
					};
				}
			}
			_ => {}
		};
		return result;
	}
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
		let input = r#"{"string_value": "string", "array_value": ["entry1", "entry2"]}"#;
		let mut correct_output = Database {
			values: HashMap::new(),
		};
		correct_output.values.insert(String::from("string_value"), StringOrArray::String(String::from("string")));
		correct_output.values.insert(String::from("array_value"), StringOrArray::Array(vec![String::from("entry1"), String::from("entry2")]));
		let output = Database::new_from_value(serde_json::from_str(input).unwrap());
		assert_eq!(correct_output, output);
	}
}