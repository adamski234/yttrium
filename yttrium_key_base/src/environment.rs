use crate::databases;
use std::fmt::{Debug, Formatter, Error};
use std::time::Duration;
use databases::{Database, DatabaseManager};
use serenity::model::id::GuildId;
use serenity::builder::CreateEmbed;
#[path = "./events.rs"] pub mod events;

/// The environment shared between all called keys, containing the global state
pub struct Environment<'a, Manager: DatabaseManager<DB>, DB: Database> {
	/// The embed to return
	pub embed: Option<CreateEmbed>,
	/// The database manager used for accessing databases
	pub database_manager: Manager,
	/// The ID of the guild the message was sent in
	pub guild_id: GuildId,
	/// The target channel ID, defaults to the channel in which the bot was triggered
	pub target: String,
	/// Linked rule
	pub next_rule: Option<String>,
	/// URLs of attachments to send
	pub attachments: Vec<String>,
	/// The event
	pub event_info: events::EventType,
	/// Shared serenity context used for accessing discord
	pub discord_context: &'a serenity::client::Context,
	/// Used for deleting the response message
	pub delete_option: Option<Duration>,
	/// Used for adding reactions to the response message
	pub reactions_to_add: Vec<serenity::model::channel::ReactionType>,
	/// Informs the interpreter about when to go back to executing code
	pub sleep_time: Option<Duration>,
	_phantom: std::marker::PhantomData<DB>,
}

impl<'a, Manager: DatabaseManager<DB>, DB: Database> Environment<'a, Manager, DB> {
	pub fn new(event_info: events::EventType, guild_id: GuildId, context: &'a serenity::client::Context, db_manager: Manager) -> Self {
		return Self {
			embed: None,
			target: String::new(),
			next_rule: None,
			database_manager: db_manager,
			guild_id: guild_id,
			attachments: Vec::new(),
			event_info: event_info,
			discord_context: context,
			delete_option: None,
			reactions_to_add: vec![],
			sleep_time: None,
			_phantom: std::marker::PhantomData,
		};
	}
}

impl<'a, Manager: DatabaseManager<DB>, DB: Database> Debug for Environment<'a, Manager, DB> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		return f.debug_struct("Environment").field("embed", &self.embed).field("guild_id", &self.guild_id)
		.field("target", &self.target).field("attachments", &self.attachments).field("event_info", &self.event_info)
		.field("delete_option", &self.delete_option).field("reactions_to_add", &self.reactions_to_add).finish();
	}
}

unsafe impl<'a, Manager: DatabaseManager<DB>, DB: Database> Send for Environment<'a, Manager, DB> {}
unsafe impl<'a, Manager: DatabaseManager<DB>, DB: Database> Sync for Environment<'a, Manager, DB> {}