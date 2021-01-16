#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use yttrium_key_base as key_base;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::{
		Environment,
		events,
	},
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_trigger {
		info: create_key_info(),
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("trigger"),
		parameters_required: vec![0],
	};
}

#[allow(non_camel_case_types)]
struct std_trigger {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_trigger {}
unsafe impl Sync for std_trigger {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_trigger {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn run_key(&self, _parameter: &[String], environment: &mut Environment<Manager, DB>) -> Result<String, String> {
		match &environment.event_info {
			events::EventType::Message(event) => {
				return Ok(event.trigger.clone());
			}
			events::EventType::Default => {
				return Ok(String::from("EventType::Default"));
			}
			events::EventType::MemberJoin(_) => {
				return Ok(String::from("EventType::MemberJoin"));
			}
			events::EventType::MemberLeave(_) => {
				return Ok(String::from("EventType::MemberLeave"));
			}
			events::EventType::MemberUpdate(_) => {
				return Ok(String::from("EventType::MemberUpdate"));
			}
			events::EventType::RoleCreate(_) => {
				return Ok(String::from("EventType::RoleCreate"));
			}
			events::EventType::RoleDelete(_) => {
				return Ok(String::from("EventType::RoleDelete"));
			}
			events::EventType::RoleUpdate(_) => {
				return Ok(String::from("EventType::RoleUpdate"));
			}
			events::EventType::ChannelCreate(_) => {
				return Ok(String::from("EventType::ChannelCreate"));
			}
			events::EventType::ChannelDelete(_) => {
				return Ok(String::from("EventType::ChannelDelete"));
			}
			events::EventType::ChannelUpdate(_) => {
				return Ok(String::from("EventType::ChannelUpdate"));
			}
			events::EventType::GuildUpdate(_) => {
				return Ok(String::from("EventType::GuildUpdate"));
			}
			events::EventType::VoiceUpdate(_) => {
				return Ok(String::from("EventType::VoiceUpdate"));
			}
			events::EventType::ReactionAdd(_) => {
				return Ok(String::from("EventType::ReactionAdd"));
			}
			events::EventType::ReactionRemove(_) => {
				return Ok(String::from("EventType::ReactionRemove"));
			}
		}
	}
}