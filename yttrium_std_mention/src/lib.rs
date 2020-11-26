#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use key_base::environment::events::*;
#[cfg(feature = "loader")]
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	return Box::into_raw(Box::new(std_mention {
		info: create_key_info(),
		function: key_function,
	}));
}

pub fn safe_create() -> Box<dyn key_base::Key> {
	return Box::new(std_mention {
		info: create_key_info(),
		function: key_function,
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
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String>,
}

impl key_base::Key for std_mention {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
		return self.function;
	}
}

fn key_function(parameter: &[String], environment: &mut key_base::environment::Environment) -> Result<String, String> {
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