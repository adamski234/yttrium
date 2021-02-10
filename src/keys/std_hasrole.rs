#![allow(clippy::needless_return)]

use yttrium_key_base as key_base;
use serenity::model::id::{UserId, RoleId};
use serenity::async_trait;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_hasrole {
		info: create_key_info(),
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
}

unsafe impl Send for std_hasrole {}
unsafe impl Sync for std_hasrole {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_hasrole {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
		if matcher.is_match(&parameter[0]) {
			let guild_id = environment.guild_id;
			let user_id = UserId::from(parameter[0].parse::<u64>().unwrap());
			let role_id;
			if matcher.is_match(&parameter[1]) {
				role_id = RoleId::from(parameter[1].parse::<u64>().unwrap());
			} else {
				let guild = environment.discord_context.cache.guild(guild_id).await.unwrap();
				match guild.role_by_name(&parameter[1]) {
					Some(role) => {
						role_id = role.id;
					}
					None => {
						return Ok(String::from("0"));
					}
				}
			}
			let member;
			match environment.discord_context.cache.member(guild_id, user_id).await {
				Some(result) => {
					member = result;
				}
				None => {
					return Err(String::from("Member couldn't be found"));
				}
			}
			let has_role = member.roles.contains(&role_id);
			return Ok(String::from(if has_role { "1" } else { "0" }));
		} else {
			return Ok(String::from("0"));
		}
	}
}