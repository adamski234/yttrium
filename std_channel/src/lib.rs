#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use serenity::model::id::ChannelId;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		/*
		Parameters:
		Optional, indicates what should be returned. Possible values: id, name, position, type. Defaults to id
		Optional, channel ID to target 
		*/
		name: String::from("channel"),
		parameters_required: vec![0, 1, 2],
	};
	return Box::into_raw(Box::new(std_channel {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_channel {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_channel {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
	let channel_id;
	if parameter.len() == 2 && matcher.is_match(&parameter[1]) {
		channel_id = ChannelId::from(parameter[1].parse::<u64>().unwrap());
	} else {
		use key_base::environment::events::EventType;
		match &environment.event_info {
			EventType::Message(event) => {
				channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
			}
			EventType::ChannelCreate(event) => {
				channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
			}
			EventType::ChannelUpdate(event) => {
				channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
			}
			EventType::VoiceUpdate(event) => {
				channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
			}
			EventType::ReactionAdd(event) => {
				channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
			}
			EventType::ReactionRemove(event) => {
				channel_id = ChannelId::from(event.channel_id.parse::<u64>().unwrap());
			}
			_ => {
				return String::new();
			}
		}
	}
	if parameter.is_empty() || parameter[0] == "id" {
		return channel_id.to_string();
	}
	let channel = futures::executor::block_on(environment.discord_context.cache.guild_channel(channel_id)).unwrap();
	match parameter[0].as_str() {
		"name" => {
			return channel.name;
		}
		"position" => {
			return channel.position.to_string();
		}
		"type" => {
			return String::from(channel.kind.name());
		}
		_ => {
			return String::new();
		}
	}
}