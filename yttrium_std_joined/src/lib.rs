#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use futures::executor;
use serenity::model::id::UserId;
use yttrium_key_base as key_base;
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_joined {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_joined {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("joined"),
		parameters_required: vec![0, 1],
	};
}

#[allow(non_camel_case_types)]
struct std_joined {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_joined {
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
	if parameter.is_empty() {
		match &environment.event_info {
			key_base::environment::events::EventType::MemberJoin(event) => {
				user_id = event.user_id.clone();
			}
			key_base::environment::events::EventType::Message(event) => {
				user_id = event.user_id.clone();
			}
			key_base::environment::events::EventType::MemberUpdate(event) => {
				user_id = event.user_id.clone();
			}
			key_base::environment::events::EventType::VoiceUpdate(event) => {
				user_id = event.user_id.clone();
			}
			key_base::environment::events::EventType::ReactionAdd(event) => {
				user_id = event.user_id.clone();
			}
			key_base::environment::events::EventType::ReactionRemove(event) => {
				user_id = event.user_id.clone();
			}
			_ => {
				return Err(String::from("`joined` was called on invalid event type"));
			}
		}
	} else {
		user_id = UserId::from(parameter[0].parse::<u64>().unwrap());
	}
	let member;
	match executor::block_on(environment.discord_context.cache.member(guild_id, user_id)) {
		Some(result) => {
			member = result;
		}
		None => {
			return Err(String::from("Member could not be found"));
		}
	}
	return Ok(member.joined_at.unwrap().timestamp_millis().to_string());
}