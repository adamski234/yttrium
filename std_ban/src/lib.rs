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
	let guild_id = environment.guild_id.clone();
	let user_id;
	let mut reason = &String::new();
	let mut days_to_remove = 0;
	if !parameter.is_empty() {
		reason = &parameter[0];
	}
	if parameter.len() >= 2 {
		if let Ok(value) = parameter[1].parse() {
			days_to_remove = value;
		}
	}
	if parameter.len() == 3 {
		user_id = UserId::from(parameter[3].parse::<u64>().unwrap());
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
				return String::new();
			}
		}
	}
	let member = futures::executor::block_on(environment.discord_context.cache.member(guild_id, user_id)).unwrap();
	if parameter.len() >= 2 {
		futures::executor::block_on(member.ban_with_reason(&environment.discord_context.http, days_to_remove, reason)).unwrap();
	} else {
		futures::executor::block_on(member.ban(&environment.discord_context.http, days_to_remove)).unwrap();
	}
	return String::new();
}