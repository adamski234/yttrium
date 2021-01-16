#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use yttrium_key_base as key_base;
use key_base::environment::events;
use serenity::model::id::{UserId, RoleId};
use futures::executor;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
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
struct std_role<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_role<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_role<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_role<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
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
				return Err(String::from("`role` called on invalid event without an ID"));
			}
		}
	} else {
		match parameter[1].parse::<u64>() {
			Ok(result) => {
				user_id = UserId::from(result);
			}
			Err(error) => {
				return Err(format!("Invalid ID passed to `role`: `{}`", error));
			}
		}
	}
	match executor::block_on(environment.discord_context.cache.guild(guild_id)) {
		Some(guild) => {
			let mut role_id;
			if matcher.is_match(&parameter[0]) {
				role_id = RoleId::from(parameter[0].parse::<u64>().unwrap());
				//Safeguard against 18 characters long role names composed only of digits
				if !guild.roles.contains_key(&role_id) {
					match guild.role_by_name(&parameter[0]) {
						Some(result) => {
							role_id = result.id;
						}
						None => {
							return Err(String::from("Role could not be found"))
						}
					}
				}
			} else {
				match guild.role_by_name(&parameter[0]) {
					Some(result) => {
						role_id = result.id;
					}
					None => {
						return Err(String::from("Role could not be found"))
					}
				}
			}
			match executor::block_on(guild.member(&environment.discord_context.http, user_id)) {
				Ok(mut member) => {
					if let Err(error) = executor::block_on(member.add_role(&environment.discord_context.http, role_id)) {
						return Err(format!("Could not add role: `{}`", error));
					}
				}
				Err(error) => {
					return Err(format!("Member could not be found: `{}`", error));
				}
			}
		}
		None => {
			return Err(String::from("Guild could not be found"));
		}
	}
	
	return Ok(String::new());
}