#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use serenity::model::id::ChannelId;
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_channel {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_channel {
		info: create_key_info(),
		function: key_function,
	});
}


/*
Parameters:
Optional, indicates what should be returned. Possible values: id, name, position, type. Defaults to id
Optional, channel ID to target 
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("channel"),
		parameters_required: vec![0, 1, 2],
	};
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
				channel_id = event.channel_id.clone();
			}
			EventType::ChannelCreate(event) => {
				channel_id = event.channel_id.clone();
			}
			EventType::ChannelUpdate(event) => {
				channel_id = event.channel_id.clone();
			}
			EventType::VoiceUpdate(event) => {
				channel_id = event.channel_id.clone();
			}
			EventType::ReactionAdd(event) => {
				channel_id = event.channel_id.clone();
			}
			EventType::ReactionRemove(event) => {
				channel_id = event.channel_id.clone();
			}
			_ => {
				environment.runtime_error = Some(String::from("Invalid return value type in `channel`"));
				return String::new();
			}
		}
	}
	if parameter.is_empty() || parameter[0] == "id" {
		return channel_id.to_string();
	}
	let channel;
	match futures::executor::block_on(environment.discord_context.cache.guild_channel(channel_id)) {
		Some(chan) => {
			channel = chan;
		}
		None => {
			environment.runtime_error = Some(String::from("Channel does not exist"));
			return String::new();
		}
	}
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