#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use futures::executor;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("setnickname"),
		parameters_required: vec![1, 2],
	};
	return Box::into_raw(Box::new(std_setnickname {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_setnickname {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_setnickname {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
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
				return String::new();
			}
		}
	} else {
		member_id = serenity::model::id::UserId::from(parameter[1].parse::<u64>().unwrap());
	}
	executor::block_on(guild.edit_member(&environment.discord_context.http, member_id, |member| {
		member.nickname(parameter[0].clone());
		return member;
	})).unwrap();
	return String::new();
}