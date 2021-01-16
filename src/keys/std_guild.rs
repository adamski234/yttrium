#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use futures::executor;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn safe_create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_guild {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("guild"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_guild<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_guild<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_guild<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_guild<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	if parameter.is_empty() {
		return Ok(String::new());
	}
	let guild_id = environment.guild_id.clone();
	let guild;
	match executor::block_on(environment.discord_context.cache.guild(&guild_id)) {
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
			return Ok(guild.region.clone());
		}
		_ => {
			return Err(String::from("Invalid property passed to `guild`"));
		}
	}
}