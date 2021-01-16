#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use yttrium_key_base as key_base;
use key_base::environment::events;
use futures::executor;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
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
struct std_kick<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_kick<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_kick<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_kick<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
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