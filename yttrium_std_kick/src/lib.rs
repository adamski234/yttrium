#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use yttrium_key_base as key_base;
use key_base::environment::events;
use futures::executor;

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_kick {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("kick"),
		parameters_required: vec![0, 1],
	};
}

#[allow(non_camel_case_types)]
struct std_kick {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_kick {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}
fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
	let guild_id = environment.guild_id.clone();
	let user_id;
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
			return Err(String::from("`kick` was called on an invalid event type"));
		}
	}
	match executor::block_on(environment.discord_context.cache.member(guild_id, user_id)) {
		Some(member) => {
			if parameter.len() == 1 {
				if let Err(error) = executor::block_on(member.kick_with_reason(&environment.discord_context.http, &parameter[0])) {
					return Err(error.to_string());
				}
			} else {
				if let Err(error) = executor::block_on(member.kick(&environment.discord_context.http)) {
					return Err(error.to_string());
				}
			}
		}
		None => {
			return Err(String::from("Could not find member"));
		}
	}
	return Ok(String::new());
}