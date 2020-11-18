#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use key_base::environment::events;
use serenity::model::id::{UserId, RoleId};
use futures::executor;
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_role {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_role {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("role"),
		parameters_required: vec![1, 2],
	};
}

#[allow(non_camel_case_types)]
struct std_role {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_role {
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
	if parameter.len() == 1 {
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
	} else {
		user_id = UserId::from(parameter[1].parse::<u64>().unwrap());
	}
	let guild = executor::block_on(environment.discord_context.cache.guild(guild_id)).unwrap();
	let mut role_id;
	if matcher.is_match(&parameter[0]) {
		role_id = RoleId::from(parameter[0].parse::<u64>().unwrap());
		if !guild.roles.contains_key(&role_id) {
			//Safeguard against 18 characters long role names composed only of digits
			role_id = guild.role_by_name(&parameter[0]).unwrap().id;
		}
	} else {
		role_id = guild.role_by_name(&parameter[0]).unwrap().id;
	}
	let mut member = executor::block_on(guild.member(&environment.discord_context.http, user_id)).unwrap();
	executor::block_on(member.add_role(&environment.discord_context.http, role_id)).unwrap();
	return String::new();
}