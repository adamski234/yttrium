use crate::embed;
use crate::databases;
#[path = "./events.rs"] pub mod events;

#[derive(Debug)]
pub struct Environment {
	pub embed: Option<embed::Embed>,
	pub database_manager: databases::DatabaseManager,
	pub guild_id: String,
	pub target: String, //Default is the channel in which the bot was triggered
	pub attachments: Vec<String>, //For the attachments to send in url form
	pub event_info: events::EventType,
}

impl Environment {
	pub fn new(event_info: events::EventType, guild_id: String) -> Self {
		return Self {
			embed: None,
			target: String::new(),
			guild_id: guild_id.clone(),
			database_manager: databases::DatabaseManager::new(guild_id),
			attachments: Vec::new(),
			event_info: event_info,
		};
	}
}