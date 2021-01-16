#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::{
		Environment,
		events::*,
	},
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_mention {
		info: create_key_info(),
	});
}

/*
Parameters:
Required, chooses the kind of mention to use, possible values: channels, users, roles
Required, offset, starting with 0
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("mention"),
		parameters_required: vec![2],
	};
}

#[allow(non_camel_case_types)]
struct std_mention {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_mention {}
unsafe impl Sync for std_mention {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_mention {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn run_key(&self, parameter: &[String], environment: &mut Environment<Manager, DB>) -> Result<String, String> {
		let message_id;
		let channel_id;
		match &environment.event_info {
			EventType::Message(event) => {
				message_id = event.message_id.clone();
				channel_id = event.channel_id.clone();
			}
			EventType::ReactionAdd(event) => {
				message_id = event.message_id.clone();
				channel_id = event.channel_id.clone();
			}
			EventType::ReactionRemove(event) => {
				message_id = event.message_id.clone();
				channel_id = event.channel_id.clone();
			}
			_ => {
				return Err(String::from("`mention` called on an invalid event type"));
			}
		}
		let index: usize = parameter[1].parse().unwrap();
		let message;
		match futures::executor::block_on(environment.discord_context.cache.message(channel_id, message_id)) {
			Some(msg) => {
				message = msg;
			}
			None => {
				return Err(String::from("Message couldn't be found"));
			}
		}
		match parameter[0].as_str() {
			"channels" => {
				return Ok(message.mention_channels[index].id.to_string());
			}
			"users" => {
				return Ok(message.mentions[index].id.to_string());
			}
			"roles" => {
				return Ok(message.mention_roles[index].to_string());
			}
			_ => {
				return Err(String::from("`mention` called with invalid property parameter"));
			}
		}
	}
}