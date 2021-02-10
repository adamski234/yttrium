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
	return Box::new(std_guild {
		info: create_key_info(),
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("guild"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_guild {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_guild {}
unsafe impl Sync for std_guild {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_guild {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		if parameter.is_empty() {
			return Ok(String::new());
		}
		let guild_id = environment.guild_id;
		let guild;
		match environment.discord_context.cache.guild(&guild_id).await {
			Some(result) => {
				guild = result;
			}
			None => {
				return Err(String::from("Guild couldn't be found"))
			}
		}
		match parameter[0].as_str() {
			"id" => {
				return Ok(guild.id.to_string());
			}
			"owner" => {
				return Ok(guild.owner_id.to_string());
			}
			"membercount" => {
				return Ok(guild.members.len().to_string());
			}
			"rolecount" => {
				return Ok(guild.roles.len().to_string());
			}
			"channelcount" => {
				return Ok(guild.channels.len().to_string());
			}
			"icon" => {
				match guild.icon_url() {
					Some(url) => {
						return Ok(url);
					}
					None => {
						return Ok(String::new());
					}
				}
			}
			"region" => {
				return Ok(guild.region);
			}
			_ => {
				return Err(String::from("Invalid property passed to `guild`"));
			}
		}
	}
}