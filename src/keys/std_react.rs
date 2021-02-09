#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use yttrium_key_base as key_base;
use serenity::async_trait;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::{
		Environment,
		events::EventType,
	},
};
use serenity::model::{
	channel::ReactionType,
	id::EmojiId,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_react {
		info: create_key_info(),
	});
}

/*
Parameters:
Optional, set to "here" to send a @here instead of an @everyone
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("react"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_react {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_react {}
unsafe impl Sync for std_react {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_react {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		if let EventType::Message(info) = &environment.event_info {
			match info.channel_id.message(&environment.discord_context, info.message_id).await {
				Ok(message) => {
					let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
					if matcher.is_match(&parameter[0]) {
						//Guild reaction
						message.react(&environment.discord_context, ReactionType::from(EmojiId::from(parameter[0].parse::<u64>().unwrap()))).await.unwrap();
					} else {
						//Normal unicode reaction
						let grapheme_count = unicode_segmentation::UnicodeSegmentation::graphemes(parameter[0].as_str(), true).count();
						if grapheme_count == 1 {
							message.react(&environment.discord_context, ReactionType::Unicode(parameter[0].clone())).await.unwrap();
						} else {
							return Err(String::from("Too many characters passed to `react`"));
						}
					}
					return Ok(String::new());
				}
				Err(_) => {
					return Err(String::from("Could not find the message in `react`"));
				}
			}
		} else {
			return Err(String::from("`react` called on invalid event"));
		}
	}
}