#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
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

struct std_setnickname {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_setnickname {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
	let guild = environment.discord_context.cache.read().guild(environment.guild_id.parse::<u64>().unwrap()).unwrap();
	if parameter.len() == 1 {
		match &environment.event_info {
			key_base::environment::events::EventType::MemberJoin(event) => {
				guild.read().edit_member(&environment.discord_context.http, event.user_id.parse::<u64>().unwrap(), |member| {
					member.nickname(parameter[0].clone());
					return member;
				}).unwrap();
			}
			key_base::environment::events::EventType::Message(event) => {
				guild.read().edit_member(&environment.discord_context.http, event.user_id.parse::<u64>().unwrap(), |member| {
					member.nickname(parameter[0].clone());
					return member;
				}).unwrap();
			}
			key_base::environment::events::EventType::MemberUpdate(event) => {
				guild.read().edit_member(&environment.discord_context.http, event.user_id.parse::<u64>().unwrap(), |member| {
					member.nickname(parameter[0].clone());
					return member;
				}).unwrap();
			}
			key_base::environment::events::EventType::ReactionAdd(event) => {
				guild.read().edit_member(&environment.discord_context.http, event.user_id.parse::<u64>().unwrap(), |member| {
					member.nickname(parameter[0].clone());
					return member;
				}).unwrap();
			}
			key_base::environment::events::EventType::ReactionRemove(event) => {
				guild.read().edit_member(&environment.discord_context.http, event.user_id.parse::<u64>().unwrap(), |member| {
					member.nickname(parameter[0].clone());
					return member;
				}).unwrap();
			}
			_ => {}
		}
	} else {
		guild.read().edit_member(&environment.discord_context.http, parameter[1].parse::<u64>().unwrap(), |member| {
			member.nickname(parameter[0].clone());
			return member;
		}).unwrap();
	}
	return String::new();
}