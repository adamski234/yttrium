#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use key_base::environment::events::*;
use serenity::model::id::{ChannelId, MessageId};
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
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
	if parameter.is_empty() {
		match &environment.event_info {
			EventType::Message(event) => {
				let message_id = MessageId::from(event.message_id.parse::<u64>().unwrap());
				let channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
				environment.discord_context.cache.read().message(channel_id, message_id).unwrap().delete(&environment.discord_context.http).unwrap();
			}
			EventType::ReactionAdd(event) => {
				let message_id = MessageId::from(event.message_id.parse::<u64>().unwrap());
				let channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
				environment.discord_context.cache.read().message(channel_id, message_id).unwrap().delete(&environment.discord_context.http).unwrap();
			}
			EventType::ReactionRemove(event) => {
				let message_id = MessageId::from(event.message_id.parse::<u64>().unwrap());
				let channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
				environment.discord_context.cache.read().message(channel_id, message_id).unwrap().delete(&environment.discord_context.http).unwrap();
			}
			_ => {}
		}
	} else {
		let time = humantime::parse_duration(&parameter[0]).unwrap();
		if parameter.len() == 1 {
			std::thread::sleep(time);
			match &environment.event_info {
				EventType::Message(event) => {
					let message_id = MessageId::from(event.message_id.parse::<u64>().unwrap());
					let channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
					environment.discord_context.cache.read().message(channel_id, message_id).unwrap().delete(&environment.discord_context.http).unwrap();
				}
				EventType::ReactionAdd(event) => {
					let message_id = MessageId::from(event.message_id.parse::<u64>().unwrap());
					let channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
					environment.discord_context.cache.read().message(channel_id, message_id).unwrap().delete(&environment.discord_context.http).unwrap();
				}
				EventType::ReactionRemove(event) => {
					let message_id = MessageId::from(event.message_id.parse::<u64>().unwrap());
					let channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
					environment.discord_context.cache.read().message(channel_id, message_id).unwrap().delete(&environment.discord_context.http).unwrap();
				}
				_ => {}
			}
		} else {
			if parameter.len() == 2 {
				let channel_id;
				let delete_count = parameter[1].parse().unwrap();
				match &environment.event_info {
					EventType::Message(event) => {
						channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
					}
					EventType::ReactionAdd(event) => {
						channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
					}
					EventType::ReactionRemove(event) => {
						channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
					}
					EventType::ChannelUpdate(event) => {
						channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
					}
					_ => {
						return String::new();
					}
				}
				let channel = environment.discord_context.cache.read().channel(&channel_id).unwrap();
				if let serenity::model::channel::Channel::Guild(chan) = channel {
					let messages = chan.read().messages(&environment.discord_context.http, |retriever| {
						return retriever.limit(delete_count);
					}).unwrap();
					chan.read().delete_messages(&environment.discord_context.http, &messages).unwrap();
				}
			} else {
				let matcher = regex::Regex::new("[0-9]{18}").unwrap();
				if matcher.is_match(&parameter[2]) {
					let channel_id;
					let delete_count = parameter[1].parse().unwrap();
					match &environment.event_info {
						EventType::Message(event) => {
							channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
						}
						EventType::ReactionAdd(event) => {
							channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
						}
						EventType::ReactionRemove(event) => {
							channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
						}
						EventType::ChannelUpdate(event) => {
							channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
						}
						_ => {
							return String::new();
						}
					}
					let channel = environment.discord_context.cache.read().channel(&channel_id).unwrap();
					if let serenity::model::channel::Channel::Guild(chan) = channel {
						let mut messages = chan.read().messages(&environment.discord_context.http, |retriever| {
							return retriever.limit(100);
						}).unwrap();
						messages = messages.into_iter().filter(|message| {
							return message.author.id.to_string() == parameter[2];
						}).take(delete_count).collect();
						chan.read().delete_messages(&environment.discord_context.http, &messages).unwrap();
					}
				}
			}
		}
	}
	return String::new();
}