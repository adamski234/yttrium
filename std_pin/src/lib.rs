#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use key_base::environment::events;
#[no_mangle]
pub fn key_create() -> *mut dyn key_base::Key {
	let key_info = key_base::KeyInfo {
		name: String::from("pin"),
		parameters_required: vec![0],
	};
	return Box::into_raw(Box::new(std_pin {
		info: key_info,
		function: key_function,
	}));
}

struct std_pin {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String,
}

impl key_base::Key for std_pin {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
		return self.function;
	}
}

fn key_function(_parameter: &Vec<String>, environment: &mut key_base::environment::Environment) -> String {
	match &environment.event_info {
	    events::EventType::Message(event) => {
			let message_id = serenity::model::id::MessageId::from(event.message_id.parse::<u64>().unwrap());
			let channel_id = serenity::model::id::ChannelId::from(event.channel_id.parse::<u64>().unwrap());
			let message = environment.discord_context.cache.read().message(channel_id, message_id).unwrap();
			message.pin(&environment.discord_context.http).unwrap();
		}
		_ => {}
	}
	return String::new();
}