use crate::embed;
use crate::databases;

pub struct Environment {
	pub embed: Option<embed::Embed>,
	pub database_manager: databases::DatabaseManager,
	pub target: String, //Default is the channel in which the bot was triggered
	pub channel_id: String,
	pub guild_id: String,
	pub message_id: String,
	pub user_id: String,
	pub attachments: Vec<String>, //For the attachments to send in url form
	pub trigger: String,
	pub event_info: EventInfo,
}

impl Environment {
	pub fn new(message_id: String, trigger_channel: String, guild_id: String, user_id: String, trigger: String) -> Self {
		return Self {
			embed: None,
			message_id: message_id,
			target: trigger_channel.clone(),
			channel_id: trigger_channel,
			guild_id: guild_id.clone(),
			user_id: user_id,
			database_manager: databases::DatabaseManager::new(guild_id),
			attachments: Vec::new(),
			trigger: trigger,
			event_info: EventInfo::Default,
		};
	}
}
#[derive(Debug)]
pub enum EventInfo {
	Default,
}