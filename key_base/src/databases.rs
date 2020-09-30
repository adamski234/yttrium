use std::fs;
use std::collections::HashMap;

static database_dir: &str = "./databases/";

#[derive(Debug)]
pub struct DatabaseManager {
	pub guild_id: String,
}

//This will be an issue with multiple people trying to write to a single database at the same time
impl DatabaseManager {
	pub fn new(guild_id: String) -> Self {
		//Converts a json file to a HashMap<String, Database>
		match std::fs::read(format!("{}{}.json", database_dir, guild_id)) {
		    Ok(value) => {
				//Convert the file
				let values: serde_json::Value = serde_json::from_str(&String::from_utf8(value).unwrap()).unwrap();
				match values {
					serde_json::Value::Object(value) => {
						let mut map = HashMap::new();
						for (db_name, db_value) in value.into_iter() {
							map.insert(db_name, value_into_database(db_value));
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
	pub fn get_database(&self, name: String) -> Option<&mut Database> {
		todo!();
	}
	pub fn create_database(&self, name: String) -> &mut Database {
		todo!();
	}
	pub fn write(&self) {
		todo!();
	}
}

#[derive(Debug)]
pub struct Database;

impl Database {
	pub fn new() -> Self {
		return Self {};
	}
	//TODO: Arrays?
	pub fn get_key(&self, name: String) -> Option<String> {
		todo!();
	}
	pub fn write_key(&mut self, name: String, value: String) {
		todo!();
	}
}

fn value_into_database(value: serde_json::Value) -> Database {
	todo!();
}