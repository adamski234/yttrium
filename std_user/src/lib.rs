#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use key_base::environment::events::*;
use serenity::model::id::UserId;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	/*
	Parameters:
	Optional, chooses information to return, possible values: id, nickname, username, avatar, discriminator
	Optional, user ID to target
	*/
	let key_info = key_base::KeyInfo {
		name: String::from("user"),
		parameters_required: vec![2],
	};
	return Box::into_raw(Box::new(std_user {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_user {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_user {
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
	if parameter.len() == 2 {
		user_id = UserId::from(parameter[1].parse::<u64>().unwrap());
	} else {
		match &environment.event_info {
			EventType::MemberJoin(event) => {
				user_id = event.user_id.clone();
			}
			EventType::MemberLeave(event) => {
				user_id = event.user_id.clone();
			}
			EventType::MemberUpdate(event) => {
				user_id = event.user_id.clone();
			}
			EventType::Message(event) => {
				user_id = event.user_id.clone();
			}
			EventType::VoiceUpdate(event) => {
				user_id = event.user_id.clone();
			}
			EventType::ReactionAdd(event) => {
				user_id = event.user_id.clone();
			}
			EventType::ReactionRemove(event) => {
				user_id = event.user_id.clone();
			}
			_ => {
				return String::new();
			}
		}
	}
	let user = futures::executor::block_on(environment.discord_context.cache.member(guild_id, user_id)).unwrap();
	match parameter[0].as_str() {
		"id" => {
			return user.user.id.to_string();
		}
		"nickname" => {
			match user.nick {
				Some(nick) => {
					return nick;
				}
				None => {
					return user.user.name;
				}
			}
		}
		"username" => {
			return user.user.name;
		}
		"avatar" => {
			match user.user.avatar_url() {
				Some(url) => {
					return url;
				}
				None => {
					return String::new();
				}
			}
		}
		"discriminator" => {
			return user.user.discriminator.to_string();
		}
		_ => {
			return String::new();
		}
	}
}