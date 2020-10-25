#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use key_base::environment::events;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("trigger"),
		parameters_required: vec![0],
	};
	return Box::into_raw(Box::new(std_trigger {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_trigger {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_trigger {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(_parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	match &environment.event_info {
		events::EventType::Message(event) => {
			return event.trigger.clone();
		}
		events::EventType::Default => {
			return String::from("EventType::Default");
		}
		events::EventType::MemberJoin(_) => {
			return String::from("EventType::MemberJoin");
		}
		events::EventType::MemberLeave(_) => {
			return String::from("EventType::MemberLeave");
		}
		events::EventType::MemberUpdate(_) => {
			return String::from("EventType::MemberUpdate");
		}
		events::EventType::RoleCreate(_) => {
			return String::from("EventType::RoleCreate");
		}
		events::EventType::RoleDelete(_) => {
			return String::from("EventType::RoleDelete");
		}
		events::EventType::RoleUpdate(_) => {
			return String::from("EventType::RoleUpdate");
		}
		events::EventType::ChannelCreate(_) => {
			return String::from("EventType::ChannelCreate");
		}
		events::EventType::ChannelDelete(_) => {
			return String::from("EventType::ChannelDelete");
		}
		events::EventType::ChannelUpdate(_) => {
			return String::from("EventType::ChannelUpdate");
		}
		events::EventType::GuildUpdate(_) => {
			return String::from("EventType::GuildUpdate");
		}
		events::EventType::VoiceUpdate(_) => {
			return String::from("EventType::VoiceUpdate");
		}
		events::EventType::ReactionAdd(_) => {
			return String::from("EventType::ReactionAdd");
		}
		events::EventType::ReactionRemove(_) => {
			return String::from("EventType::ReactionRemove");
		}
	}
}