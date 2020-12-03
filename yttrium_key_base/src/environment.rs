use crate::embed;
use crate::databases;
use serenity::model::id::GuildId;
#[path = "./events.rs"] pub mod events;

/// The environment shared between all called keys, containing the global state
pub struct Environment<'a> {
	/// Unfinished
	pub embed: Option<embed::Embed>,
	/// The database manager used for accessing databases
	pub database_manager: Box<dyn databases::DatabaseManager>,
	/// The ID of the guild the message was sent in
	pub guild_id: GuildId,
	/// The target channel ID, defaults to the channel in which the bot was triggered
	pub target: String,
	/// URLs of attachments to send
	pub attachments: Vec<String>,
	/// The event
	pub event_info: events::EventType,
	/// Shared serenity context used for accessing discord
	pub discord_context: &'a serenity::client::Context,
	/// Used for deleting the response message
	pub delete_option: Option<std::time::Duration>,
	/// Used for adding reactions to the response message
	pub reactions_to_add: Vec<String>,
}

impl<'a> Environment<'a> {
	pub fn new(event_info: events::EventType, guild_id: GuildId, context: &'a serenity::client::Context, db_manager: Box<dyn databases::DatabaseManager>) -> Self {
		return Self {
			embed: None,
			target: String::new(),
			database_manager: db_manager,
			guild_id: guild_id,
			attachments: Vec::new(),
			event_info: event_info,
			discord_context: context,
			delete_option: None,
			reactions_to_add: vec![],
		};
	}
}

unsafe impl<'a> Send for Environment<'a> {}
unsafe impl<'a> Sync for Environment<'a> {}