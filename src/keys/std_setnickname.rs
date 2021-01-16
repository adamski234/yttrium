#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use futures::executor;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
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
struct std_setnickname<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_setnickname<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_setnickname<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_setnickname<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
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