#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use key_base::environment::events;
use serenity::model::id::UserId;
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_ban {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_ban {
		info: create_key_info(),
		function: key_function,
	});
}

/*
Parameters:
Optional, reason, defaults to nothing
Optional, days to remove messages from, default 0
Optional, user id, defaults to the sender
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("ban"),
		parameters_required: vec![0, 1, 2, 3],
	};
}
#[allow(non_camel_case_types)]
struct std_ban {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_ban {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}
fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
	let guild_id = environment.guild_id.clone();
	let user_id;
	let mut days_to_remove = 0;
	if parameter.len() >= 2 {
		if let Ok(value) = parameter[1].parse() {
			days_to_remove = value;
		}
	}
	if parameter.len() == 3 && matcher.is_match(&parameter[2]) {
		user_id = UserId::from(parameter[2].parse::<u64>().unwrap());
	} else {
		match &environment.event_info {
			events::EventType::Message(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::MemberJoin(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::MemberUpdate(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::VoiceUpdate(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::ReactionAdd(event) => {
				user_id = event.user_id.clone();
			}
			events::EventType::ReactionRemove(event) => {
				user_id = event.user_id.clone();
			}
			_ => {
				environment.runtime_error = Some(String::from("`ban` called without user ID on invalid event"));
				return String::new();
			}
		}
	}
	let member;
	match futures::executor::block_on(environment.discord_context.cache.member(guild_id, user_id)) {
		Some(memb) => {
			member = memb;
		}
		None => {
			environment.runtime_error = Some(String::from("User does not exist"));
			return String::new();
		}
	}
	let result;
	if parameter.len() >= 2 {
		let reason = &parameter[0];
		result = futures::executor::block_on(member.ban_with_reason(&environment.discord_context.http, days_to_remove, reason));
	} else {
		result = futures::executor::block_on(member.ban(&environment.discord_context.http, days_to_remove));
	}
	if let Err(error) = result {
		environment.runtime_error = Some(error.to_string());
	}
	return String::new();
}