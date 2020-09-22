#[derive(Debug)]
pub struct DatabaseManager {
	pub guild_id: String,
}

//This will be an issue with multiple people trying to write to a single database at the same time
impl DatabaseManager {
	pub fn new(guild_id: String) -> Self {
		return Self { guild_id };
	}
	pub fn get_database(&self, name: String) -> Database {
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
