#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use key_base::environment::events::*;
use serenity::model::id::{ChannelId};
use futures::executor;
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_delete {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
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
struct std_delete {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_delete {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

//TODO: rework this and move some variables
fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	if !parameter.is_empty() {
		match humantime::parse_duration(&parameter[0]) {
			Ok(time) => {
				std::thread::sleep(time);
			}
			Err(error) => {
				environment.runtime_error = Some(format!("Invalid time value in `delete`: `{}`", error));
				return String::new();
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
				return String::new();
			}
		}
		let message;
		match executor::block_on(environment.discord_context.cache.message(channel_id, message_id)) {
			Some(msg) => {
				message = msg;
			}
			None => {
				environment.runtime_error = Some(String::from("Message couldn't be found"));
				return String::new();
			}
		}
		if let Err(error) = executor::block_on(message.delete(&environment.discord_context.http)) {
			environment.runtime_error = Some(format!("Couldn't delete the message: `{}`", error));
		}
	} else {
		let delete_count;
		match parameter[1].parse::<u64>() {
			Ok(result) => {
				delete_count = result;
			}
			Err(error) => {
				environment.runtime_error = Some(format!("Invalid delete count in `delete`: `{}`", error));
				return String::new();
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
					return String::new();
				}
			}
			let channel;
			match executor::block_on(environment.discord_context.cache.channel(&channel_id)) {
				Some(chan) => {
					channel = chan;
				}
				None => {
					environment.runtime_error = Some(String::from("Couldn't get channel"));
					return String::new();
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
						environment.runtime_error = Some(format!("Couldn't get messages: `{}`", error));
						return String::new();
					}
				}
				if let Err(error) = executor::block_on(chan.delete_messages(&environment.discord_context.http, &messages)) {
					environment.runtime_error = Some(format!("Couldn't delete messages: `{}`", error));
					return String::new();
				}
			} else {
				return String::new();
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
						environment.runtime_error = Some(format!("Couldn't get channel in `delete`"));
						return String::new();
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
							environment.runtime_error = Some(format!("Couldn't get messages: `{}`", error));
							return String::new();
						}
					}
					if let Err(error) = executor::block_on(chan.delete_messages(&environment.discord_context.http, &messages)) {
						environment.runtime_error = Some(format!("Couldn't delete messages: `{}`", error));
						return String::new();
					}
				}
			}
		}
	}
	return String::new();
}