pub struct Environment {
	pub embed: Option<Embed>,
	pub database_manager: DatabaseManager,
	pub target: String, //Default is the channel in which the bot was triggered
	pub guild_id: String,
	pub message_id: String,
	pub user_id: String,
	//TODO: Add user who triggered, the trigger, attachments, etc
}

//This is gonna have A LOT of functions
impl Environment {
	pub fn new(message_id: String, trigger_channel: String, guild_id: String, user_id: String) -> Self {
		return Self {
			embed: None,
			message_id: message_id,
			target: trigger_channel,
			guild_id: guild_id.clone(),
			user_id: user_id,
			database_manager: DatabaseManager::new(guild_id),
		};
	}
}

#[derive(Debug)]
pub struct Embed;

impl Embed {
	pub fn new() -> Self {
		return Self {};
	}
}

#[derive(Debug)]
pub struct DatabaseManager {
	pub guild_id: String,
}

//This will be an issue with multiple people trying to write to a single database at the same time
impl DatabaseManager {
	pub fn new(guild_id: String) -> Self {
		return Self {
			guild_id
		};
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