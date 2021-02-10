#![allow(clippy::needless_return)]

use yttrium_key_base as key_base;
use serenity::async_trait;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_setnickname {
		info: create_key_info(),
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
}

unsafe impl Send for std_setnickname {}
unsafe impl Sync for std_setnickname {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_setnickname {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		let guild = environment.discord_context.cache.guild(environment.guild_id).await.unwrap();
		let member_id;
		if parameter.len() == 1 {
			match &environment.event_info {
				key_base::environment::events::EventType::MemberJoin(event) => {
					member_id = event.user_id;
				}
				key_base::environment::events::EventType::Message(event) => {
					member_id = event.user_id;
				}
				key_base::environment::events::EventType::MemberUpdate(event) => {
					member_id = event.user_id;
				}
				key_base::environment::events::EventType::ReactionAdd(event) => {
					member_id = event.user_id;
				}
				key_base::environment::events::EventType::ReactionRemove(event) => {
					member_id = event.user_id;
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
		let result = guild.edit_member(&environment.discord_context.http, member_id, |member| {
			member.nickname(&parameter[0]);
			return member;
		}).await;
		if let Err(error) = result {
			return Err(format!("Could not change nickname of user: `{}`", error));
		}
		return Ok(String::new());
	}
}