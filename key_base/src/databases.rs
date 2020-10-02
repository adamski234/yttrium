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
	pub fn get_database(&mut self, name: &String) -> Option<&mut Database> {
		return self.databases.get_mut(name);
	}
	///Creates a database if it doesn't exist and returns it, otherwise returns a pre-existing database
	pub fn create_database(&mut self, name: &String) -> &mut Database {
		if self.databases.contains_key(name) {
			return self.databases.get_mut(name).unwrap();
		} else {
			self.databases.insert(name.clone(), Database::new_empty());
			return self.databases.get_mut(name).unwrap();
		}
	}
	///This serializes the databases and saves them into the file
	pub fn write(self) {
		let mut result = HashMap::new();
		for (name, db) in self.databases {
			let db_values = db.get_values();
			let mut serialized_db = HashMap::with_capacity(db_values.len());
			for (value_name, value) in db_values {
				match value {
				    StringOrArray::String(string) => {
						serialized_db.insert(value_name, serde_json::Value::String(string));
					}
				    StringOrArray::Array(array) => {
						let mut new_array = Vec::with_capacity(array.len());
						for item in array {
							new_array.push(serde_json::Value::String(item));
						}
						serialized_db.insert(value_name, serde_json::Value::Array(new_array));
					}
				}
			}
			result.insert(name, serialized_db);
		}
		fs::write(format!("{}{}.json", DATABASE_DIR, self.guild_id), serde_json::to_string(&result).unwrap()).unwrap();
	}
}
#[derive(Debug, PartialEq)]
pub struct Database {
	values: HashMap<String, StringOrArray>,
}

impl Database {
	pub fn new_empty() -> Self {
		return Self {
			values: HashMap::new(),
		};
	}
	pub fn new_from_value(value: serde_json::Value) -> Self {
		let mut result = Self::new_empty();
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
	pub(crate) fn get_values(self) -> HashMap<String, StringOrArray> {
		return self.values;
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
	#[test]
	fn simple_manager() {
		use crate::databases::*;
		let input = r#"{"db1": {"string_value": "string", "array_value": ["entry1", "entry2"]}}"#;
		let guild_id = String::from("abc");
		let mut correct_hashmap = HashMap::new();
		//This bases on `simple_database` succeeding
		correct_hashmap.insert(String::from("db1"), Database::new_from_value(serde_json::from_str(r#"{"string_value": "string", "array_value": ["entry1", "entry2"]}"#).unwrap()));
		let mut correct_output = DatabaseManager {
			guild_id: guild_id,
			databases: HashMap::new(),
		};
	}
}