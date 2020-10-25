#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use serenity::model::id::GuildId;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("guild"),
		parameters_required: vec![1],
	};
	return Box::into_raw(Box::new(std_guild {
		info: key_info,
		function: key_function,
	}));
}

#[allow(non_camel_case_types)]
struct std_guild {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_guild {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}
fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	if parameter.len() == 0 {
		return String::new();
	}
	let guild_id = GuildId::from(environment.guild_id.parse::<u64>().unwrap());
	let guild = environment.discord_context.cache.read().guild(&guild_id).unwrap();
	match parameter[0].as_str() {
		"id" => {
			return guild.read().id.to_string();
		}
		"owner" => {
			return guild.read().owner_id.to_string();
		}
		"membercount" => {
			return guild.read().members.len().to_string();
		}
		"rolecount" => {
			return guild.read().roles.len().to_string();
		}
		"channelcount" => {
			return guild.read().channels.len().to_string();
		}
		"icon" => {
			match guild.read().icon_url() {
				Some(url) => {
					return url;
				}
				None => {
					return String::new();
				}
			}
		}
		"region" => {
			return guild.read().region.clone();
		}
		_ => {
			return String::new();
		}
	}
}