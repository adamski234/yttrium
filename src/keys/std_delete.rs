#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use key_base::environment::events::*;
use serenity::model::id::{ChannelId};
use futures::executor;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_delete {
		info: create_key_info(),
		function: key_function,
	});
}

/*
Parameters:
Optional, time after which to delete the messages, default 0
Optional, amount of messages to delete, default 1
Optional, user ID for filtering messages, default no filtering
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("delete"),
		parameters_required: vec![0, 1, 2, 3],
	};
}

#[allow(non_camel_case_types)]
struct std_delete<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_delete<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_delete<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_delete<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

//TODO: rework this and move some variables
fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	if !parameter.is_empty() {
		match humantime::parse_duration(&parameter[0]) {
			Ok(time) => {
				std::thread::sleep(time);
			}
			Err(error) => {
				return Err(format!("Invalid time value in `delete`: `{}`", error));
			}
		}
		
	}
	if parameter.len() <= 1 {
		let message_id;
		let channel_id;
		match &environment.event_info {
			EventType::Message(event) => {
				message_id = event.message_id.clone();
				channel_id = event.channel_id.clone();
			}
			EventType::ReactionAdd(event) => {
				message_id = event.message_id.clone();
				channel_id = event.channel_id.clone();
			}
			EventType::ReactionRemove(event) => {
				message_id = event.message_id.clone();
				channel_id = event.channel_id.clone();
			}
			_ => {
				return Err(String::from("`delete` called on an invalid event"))
			}
		}
		let message;
		match executor::block_on(environment.discord_context.cache.message(channel_id, message_id)) {
			Some(msg) => {
				message = msg;
			}
			None => {
				return Err(String::from("Message couldn't be found"));
			}
		}
		if let Err(error) = executor::block_on(message.delete(&environment.discord_context.http)) {
			return Err(format!("Couldn't delete the message: `{}`", error));
		}
	} else {
		let delete_count;
		match parameter[1].parse::<u64>() {
			Ok(result) => {
				delete_count = result;
			}
			Err(error) => {
				return Err(format!("Invalid delete count in `delete`: `{}`", error));
			}
		};
		if parameter.len() == 2 {
			let channel_id;
			match &environment.event_info {
				EventType::Message(event) => {
					channel_id = event.channel_id.clone();
				}
				EventType::ReactionAdd(event) => {
					channel_id = event.channel_id.clone();
				}
				EventType::ReactionRemove(event) => {
					channel_id = event.channel_id.clone();
				}
				EventType::ChannelUpdate(event) => {
					channel_id = event.channel_id.clone();
				}
				_ => {
					return Err(String::from("`delete` called on an invalid event"));
				}
			}
			let channel;
			match executor::block_on(environment.discord_context.cache.channel(&channel_id)) {
				Some(chan) => {
					channel = chan;
				}
				None => {
					return Err(String::from("Couldn't get channel"));
				}
			}
			if let serenity::model::channel::Channel::Guild(chan) = channel {
				let messages_attempt = executor::block_on(chan.messages(&environment.discord_context.http, |retriever| {
					return retriever.limit(delete_count);
				}));
				let messages;
				match messages_attempt {
					Ok(message_list) => {
						messages = message_list;
					}
					Err(error) => {
						return Err(format!("Couldn't get messages: `{}`", error));
					}
				}
				if let Err(error) = executor::block_on(chan.delete_messages(&environment.discord_context.http, &messages)) {
					return Err(format!("Couldn't delete messages: `{}`", error));
				}
			} else {
				return Err(String::from("`delete` called on an invalid channel type"));
			}
		} else {
			let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
			if matcher.is_match(&parameter[2]) {
				let channel_id = ChannelId::from(parameter[2].parse::<u64>().unwrap());
				let channel;
				match executor::block_on(environment.discord_context.cache.channel(&channel_id)) {
					Some(chan) => {
						channel = chan;
					}
					None => {
						return Err(format!("Couldn't get channel in `delete`"));
					}
				}
				if let serenity::model::channel::Channel::Guild(chan) = channel {
					let messages_attempt = executor::block_on(chan.messages(&environment.discord_context.http, |retriever| {
						return retriever.limit(100);
					}));
					let messages: Vec<serenity::model::channel::Message>;
					match messages_attempt {
						Ok(message_list) => {
							messages = message_list.into_iter().filter(|message| {
								return message.author.id.to_string() == parameter[2];
							}).take(delete_count as usize).collect();
						}
						Err(error) => {
							return Err(format!("Couldn't get messages: `{}`", error));
						}
					}
					if let Err(error) = executor::block_on(chan.delete_messages(&environment.discord_context.http, &messages)) {
						return Err(format!("Couldn't delete messages: `{}`", error));
					}
				}
			}
		}
	}
	return Ok(String::new());
}