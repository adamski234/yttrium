#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use serenity::model::id::{UserId, RoleId};
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_hasrole {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_hasrole {
		info: create_key_info(),
		function: key_function,
	});
}

/*
Parameters:
Required, user ID
Required, role ID / name
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("hasrole"),
		parameters_required: vec![2],
	};
}

#[allow(non_camel_case_types)]
struct std_hasrole {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_hasrole {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> String {
	let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
	if matcher.is_match(&parameter[0]) {
		let guild_id = environment.guild_id.clone();
		let user_id = UserId::from(parameter[0].parse::<u64>().unwrap());
		let role_id;
		if matcher.is_match(&parameter[1]) {
			role_id = RoleId::from(parameter[1].parse::<u64>().unwrap());
		} else {
			let guild = futures::executor::block_on(environment.discord_context.cache.guild(guild_id.clone())).unwrap();
			match guild.role_by_name(&parameter[1]) {
				Some(role) => {
					role_id = role.id;
				}
				None => {
					return String::from("0");
				}
			}
		}
		let member = futures::executor::block_on(environment.discord_context.cache.member(guild_id, user_id)).unwrap();
		let has_role = member.roles.contains(&role_id);
		return String::from(if has_role { "1" } else { "0" });
	} else {
		return String::from("0");
	}
}