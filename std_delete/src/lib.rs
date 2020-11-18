#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use key_base::environment::events::*;
use serenity::model::id::{ChannelId};
use futures::executor;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	/*
	Parameters:
	Optional, time after which to delete the messages, default 0
	Optional, amount of messages to delete, default 1
	Optional, user ID for filtering messages, default no filtering
	*/
	let key_info = key_base::KeyInfo {
		name: String::from("delete"),
		parameters_required: vec![0, 1, 2, 3],
	};
	return Box::into_raw(Box::new(std_delete {
		info: key_info,
		function: key_function,
	}));
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

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	if !parameter.is_empty() {
		let time = humantime::parse_duration(&parameter[0]).unwrap();
		std::thread::sleep(time);
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
		let message = executor::block_on(environment.discord_context.cache.message(channel_id, message_id)).unwrap();
		executor::block_on(message.delete(&environment.discord_context.http)).unwrap();
	} else {
		let delete_count = parameter[1].parse().unwrap();
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
			let channel = executor::block_on(environment.discord_context.cache.channel(&channel_id)).unwrap();
			if let serenity::model::channel::Channel::Guild(chan) = channel {
				let messages = executor::block_on(chan.messages(&environment.discord_context.http, |retriever| {
					return retriever.limit(delete_count);
				})).unwrap();
				executor::block_on(chan.delete_messages(&environment.discord_context.http, &messages)).unwrap();
			} else {
				return String::new();
			}
		} else {
			let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
			if matcher.is_match(&parameter[2]) {
				let channel_id = ChannelId::from(parameter[2].parse::<u64>().unwrap());
				let channel = executor::block_on(environment.discord_context.cache.channel(&channel_id)).unwrap();
				if let serenity::model::channel::Channel::Guild(chan) = channel {
					let mut messages = executor::block_on(chan.messages(&environment.discord_context.http, |retriever| {
						return retriever.limit(100);
					})).unwrap();
					messages = messages.into_iter().filter(|message| {
						return message.author.id.to_string() == parameter[2];
					}).take(delete_count as usize).collect();
					executor::block_on(chan.delete_messages(&environment.discord_context.http, &messages)).unwrap();
				}
			}
		}
	}
	return String::new();
}