#![allow(clippy::needless_return)]

use yttrium_key_base as key_base;
use serenity::async_trait;
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
	return Box::new(std_kick {
		info: create_key_info(),
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("kick"),
		parameters_required: vec![0, 1],
	};
}

#[allow(non_camel_case_types)]
struct std_kick {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_kick {}
unsafe impl Sync for std_kick {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_kick {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		let guild_id = environment.guild_id;
		let user_id;
		match &environment.event_info {
			events::EventType::Message(event) => {
				user_id = event.user_id;
			}
			events::EventType::MemberJoin(event) => {
				user_id = event.user_id;
			}
			events::EventType::MemberUpdate(event) => {
				user_id = event.user_id;
			}
			events::EventType::VoiceUpdate(event) => {
				user_id = event.user_id;
			}
			events::EventType::ReactionAdd(event) => {
				user_id = event.user_id;
			}
			events::EventType::ReactionRemove(event) => {
				user_id = event.user_id;
			}
			_ => {
				return Err(String::from("`kick` was called on an invalid event type"));
			}
		}
		match environment.discord_context.cache.member(guild_id, user_id).await {
			Some(member) => {
				if parameter.len() == 1 {
					if let Err(error) = member.kick_with_reason(&environment.discord_context.http, &parameter[0]).await {
						return Err(error.to_string());
					}
				} else if let Err(error) = member.kick(&environment.discord_context.http).await {
					return Err(error.to_string());
				}
			}
			None => {
				return Err(String::from("Could not find member"));
			}
		}
		return Ok(String::new());
	}
}