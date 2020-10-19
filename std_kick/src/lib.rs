#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use key_base::environment::events;
use serenity::model::id::{UserId, GuildId};
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("kick"),
		parameters_required: vec![0, 1],
	};
	return Box::into_raw(Box::new(std_kick {
		info: key_info,
		function: key_function,
	}));
}

struct std_kick {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_kick {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}
fn key_function(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
	let guild_id = GuildId::from(environment.guild_id.parse::<u64>().unwrap());
	let user_id;
	match &environment.event_info {
		events::EventType::Message(event) => {
			user_id = UserId::from(event.user_id.parse::<u64>().unwrap());
		}
		events::EventType::MemberJoin(event) => {
			user_id = UserId::from(event.user_id.parse::<u64>().unwrap());
		}
		events::EventType::MemberUpdate(event) => {
			user_id = UserId::from(event.user_id.parse::<u64>().unwrap());
		}
		events::EventType::VoiceUpdate(event) => {
			user_id = UserId::from(event.user_id.parse::<u64>().unwrap());
		}
		events::EventType::ReactionAdd(event) => {
			user_id = UserId::from(event.user_id.parse::<u64>().unwrap());
		}
		events::EventType::ReactionRemove(event) => {
			user_id = UserId::from(event.user_id.parse::<u64>().unwrap());
		}
		_ => {
			return String::new();
		}
	}
	if parameter.len() == 1 {
		environment.discord_context.cache.read().member(guild_id, user_id).unwrap().kick_with_reason(&environment.discord_context.http, &parameter[0]).unwrap();
	} else {
		environment.discord_context.cache.read().member(guild_id, user_id).unwrap().kick(&environment.discord_context.http).unwrap();
	}
	return String::new();
}