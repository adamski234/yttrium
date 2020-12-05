#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use futures::executor;

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_setnickname {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("setnickname"),
		parameters_required: vec![1, 2],
	};
}

#[allow(non_camel_case_types)]
struct std_setnickname {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_setnickname {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
	let guild = executor::block_on(environment.discord_context.cache.guild(environment.guild_id.clone())).unwrap();
	let member_id;
	if parameter.len() == 1 {
		match &environment.event_info {
			key_base::environment::events::EventType::MemberJoin(event) => {
				member_id = event.user_id.clone();
			}
			key_base::environment::events::EventType::Message(event) => {
				member_id = event.user_id.clone();
			}
			key_base::environment::events::EventType::MemberUpdate(event) => {
				member_id = event.user_id.clone();
			}
			key_base::environment::events::EventType::ReactionAdd(event) => {
				member_id = event.user_id.clone();
			}
			key_base::environment::events::EventType::ReactionRemove(event) => {
				member_id = event.user_id.clone();
			}
			_ => {
				return Err(String::from("`setnickname` was called on an invalid event type without an ID"));
			}
		}
	} else {
		let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
		if matcher.is_match(&parameter[1]) {
			member_id = serenity::model::id::UserId::from(parameter[1].parse::<u64>().unwrap());
		} else {
			return Err(String::from("Invalid user ID passed to `setnickname"));
		}
	}
	let result = executor::block_on(guild.edit_member(&environment.discord_context.http, member_id, |member| {
		member.nickname(parameter[0].clone());
		return member;
	}));
	if let Err(error) = result {
		return Err(format!("Could not change nickname of user: `{}`", error));
	}
	return Ok(String::new());
}