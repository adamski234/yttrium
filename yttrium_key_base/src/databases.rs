use std::fs;
use std::io::Write;
use std::collections::HashMap;

const DATABASE_DIR: &str = "./databases/";

/// Trait for accessing databases
pub trait DatabaseManager {
	/// Gets the database from the manager, possibly creating it
	fn get_database(&mut self, name: &str) -> Option<&mut Database>;
	/// Creates a database and returns it. If the database already exists, it might get overwritten but that does not have to happen
	fn create_database(&mut self, name: &str) -> &mut Database;
	/// Deletes a database from the manager
	fn remove_database(&mut self, name: &str);
	/// Removes all entries from a database
	fn clear_database(&mut self, name: &str);
}

///This is a simple JSON based database manager that is good enough for testing but I wouldn't rely on it too much
#[derive(Debug)]
pub struct JSONDatabaseManager {
	pub guild_id: String,
	databases: HashMap<String, Database>,
}

impl JSONDatabaseManager {
	pub fn new(guild_id: &str) -> Self {
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
						file.write_all(b"{}").unwrap();
						let empty_manager = Self {
							guild_id: guild_id.to_owned(),
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
	pub fn new_from_json(json: &str, guild_id: &str) -> Self {
		return Self::new_from_value(serde_json::from_str(json).unwrap(), guild_id);
	}
	pub fn new_from_value(value: serde_json::Value, guild_id: &str) -> Self {
		let mut result = Self {
			guild_id: guild_id.to_owned(),
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

impl DatabaseManager for JSONDatabaseManager {
	fn get_database(&mut self, name: &str) -> Option<&mut Database> {
		return self.databases.get_mut(name);
	}
	fn create_database(&mut self, name: &str) -> &mut Database {
		if self.databases.contains_key(name) {
			return self.databases.get_mut(name).unwrap();
		} else {
			self.databases.insert(name.to_owned(), Database::new_empty());
			return self.databases.get_mut(name).unwrap();
		}
	}
	fn remove_database(&mut self, name: &str) {
		self.databases.remove_entry(name);
	}
	fn clear_database(&mut self, name: &str) {
		if self.databases.contains_key(name) {
			self.databases.insert(String::from(name), Database::new_empty());
		}
	}
}

impl PartialEq for JSONDatabaseManager {
	fn eq(&self, other: &Self) -> bool {
		return self.databases == other.databases;
	}
}

/// This struct is a simple wrapper around a HashMap
#[derive(Debug, PartialEq)]
pub struct Database {
	values: HashMap<String, StringOrArray>,
}

impl Database {
	/// Creates an empty database
	pub fn new_empty() -> Self {
		return Self {
			values: HashMap::new(),
		};
	}
	/// Builds a database from [serde_json::Value]
	pub fn new_from_value(value: serde_json::Value) -> Self {
		let mut result = Self::new_empty();
		if let serde_json::Value::Object(object) = value {
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
		};
		return result;
	}
	/// Retrieves a key from the database
	pub fn get_key(&self, name: &str) -> Option<StringOrArray> {
		match self.values.get(name) {
			Some(value) => {
				return Some(value.clone());
			}
			None => {
				return None;
			}
		}
	}
	/// Inserts a key into the database, overwriting the old one if it already existed
	pub fn write_key(&mut self, name: String, value: StringOrArray) {
		self.values.insert(name, value);
	}
	/// Deletes a key from the database
	pub fn remove_key(&mut self, name: &str) {
		self.values.remove(name);
	}
	/// Checks if a key exists in the database
	pub fn key_exists(&self, name: &str) -> bool {
		return self.values.contains_key(name);
	}
	pub(crate) fn get_values(self) -> HashMap<String, StringOrArray> {
		return self.values;
	}
}

unsafe impl Send for JSONDatabaseManager {}
unsafe impl Sync for JSONDatabaseManager {}

/// Enum used for differentiation between a string and an array
#[derive(Debug, PartialEq, Clone)]
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
		let input = String::from(r#"{"db1": {"string_value": "string", "array_value": ["entry1", "entry2"]}}"#);
		let guild_id = String::from("abc");
		let mut correct_hashmap = HashMap::new();
		//This bases on `simple_database` succeeding
		correct_hashmap.insert(String::from("db1"), Database::new_from_value(serde_json::from_str(r#"{"string_value": "string", "array_value": ["entry1", "entry2"]}"#).unwrap()));
		let output = JSONDatabaseManager::new_from_json(&input, &guild_id);
		let correct_output = JSONDatabaseManager {
			guild_id: guild_id,
			databases: correct_hashmap,
		};
		assert_eq!(output, correct_output);
	}
}