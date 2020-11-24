#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use futures::executor;
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_guild {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
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
struct std_guild {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_guild {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}
fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
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
			//
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
			return Ok(String::new());
		}
	}
}