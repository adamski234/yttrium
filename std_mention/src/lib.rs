#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use key_base::environment::events::*;
use serenity::model::id::{ChannelId, MessageId};
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	/*
	Parameters:
	Required, chooses the kind of mention to use, possible values: channels, users, roles
	Required, offset, starting with 0
	*/
	let key_info = key_base::KeyInfo {
		name: String::from("mention"),
		parameters_required: vec![2],
	};
	return Box::into_raw(Box::new(std_mention {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_mention {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_mention {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	let message_id;
	let channel_id;
	match &environment.event_info {
		EventType::Message(event) => {
			message_id = MessageId::from(event.message_id.parse::<u64>().unwrap());
			channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
		}
		EventType::ReactionAdd(event) => {
			message_id = MessageId::from(event.message_id.parse::<u64>().unwrap());
			channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
		}
		EventType::ReactionRemove(event) => {
			message_id = MessageId::from(event.message_id.parse::<u64>().unwrap());
			channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
		}
		_ => {
			return String::new();
		}
	}
	let index: usize = parameter[1].parse().unwrap();
	let message = futures::executor::block_on(environment.discord_context.cache.message(channel_id, message_id)).unwrap();
	match parameter[0].as_str() {
		"channels" => {
			return message.mention_channels[index].id.to_string();
		}
		"users" => {
			return message.mentions[index].id.to_string();
		}
		"roles" => {
			return message.mention_roles[index].to_string();
		}
		_ => {
			return String::new();
		}
	}
}