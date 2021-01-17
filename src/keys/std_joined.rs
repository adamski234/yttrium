#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use serenity::async_trait;
use serenity::model::id::UserId;
use yttrium_key_base as key_base;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_joined {
		info: create_key_info(),
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("joined"),
		parameters_required: vec![0, 1],
	};
}

#[allow(non_camel_case_types)]
struct std_joined {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_joined {}
unsafe impl Sync for std_joined {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_joined {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		let guild_id = environment.guild_id.clone();
		let user_id;
		if parameter.is_empty() {
			match &environment.event_info {
				key_base::environment::events::EventType::MemberJoin(event) => {
					user_id = event.user_id.clone();
				}
				key_base::environment::events::EventType::Message(event) => {
					user_id = event.user_id.clone();
				}
				key_base::environment::events::EventType::MemberUpdate(event) => {
					user_id = event.user_id.clone();
				}
				key_base::environment::events::EventType::VoiceUpdate(event) => {
					user_id = event.user_id.clone();
				}
				key_base::environment::events::EventType::ReactionAdd(event) => {
					user_id = event.user_id.clone();
				}
				key_base::environment::events::EventType::ReactionRemove(event) => {
					user_id = event.user_id.clone();
				}
				_ => {
					return Err(String::from("`joined` was called on invalid event type"));
				}
			}
		} else {
			user_id = UserId::from(parameter[0].parse::<u64>().unwrap());
		}
		let member;
		match environment.discord_context.cache.member(guild_id, user_id).await {
			Some(result) => {
				member = result;
			}
			None => {
				return Err(String::from("Member could not be found"));
			}
		}
		return Ok(member.joined_at.unwrap().timestamp_millis().to_string());
	}
}