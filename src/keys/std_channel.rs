#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use serenity::model::id::ChannelId;
use serenity::async_trait;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_channel {
		info: create_key_info(),
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
}

unsafe impl Send for std_channel {}
unsafe impl Sync for std_channel {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_channel {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
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
					return Err(String::from("Invalid event type in `channel`"));
				}
			}
		}
		if parameter.is_empty() || parameter[0] == "id" {
			return Ok(channel_id.to_string());
		}
		let channel;
		match environment.discord_context.cache.guild_channel(channel_id).await {
			Some(chan) => {
				channel = chan;
			}
			None => {
				return Err(String::from("Channel does not exist"));
			}
		}
		match parameter[0].as_str() {
			"name" => {
				return Ok(channel.name);
			}
			"position" => {
				return Ok(channel.position.to_string());
			}
			"type" => {
				return Ok(String::from(channel.kind.name()));
			}
			_ => {
				return Err(format!("Invalid property parameter given to `channel`: `{}`", parameter[0]));
			}
		}
	}
}