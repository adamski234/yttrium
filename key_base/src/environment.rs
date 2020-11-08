use crate::embed;
use crate::databases;
#[path = "./events.rs"] pub mod events;

//#[derive(Debug)]
pub struct Environment<'a> {
	pub embed: Option<embed::Embed>,
	pub database_manager: databases::DatabaseManager,
	pub guild_id: String,
	pub target: String, //Default is the channel in which the bot was triggered
	pub attachments: Vec<String>, //For the attachments to send in url form
	pub event_info: events::EventType,
	pub discord_context: &'a mut serenity::client::Context,
	pub delete_option: Option<std::time::Duration>,
	pub reactions_to_add: Vec<String>
}

impl<'a> Environment<'a> {
	pub fn new(event_info: events::EventType, guild_id: String, context: &'a mut serenity::client::Context) -> Self {
		return Self {
			embed: None,
			target: String::new(),
			database_manager: databases::DatabaseManager::new(&guild_id),
			guild_id: guild_id,
			attachments: Vec::new(),
			event_info: event_info,
			discord_context: context,
			delete_option: None,
			reactions_to_add: vec![],
		};
	}
}