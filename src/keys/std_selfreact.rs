#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
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
	return Box::new(std_selfreact {
		info: create_key_info(),
	});
}

/*
Parameters:
Required, the reaction to add, either the ID or the Unicode emoji itself (NOT the name)
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("selfreact"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_selfreact {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_selfreact {}
unsafe impl Sync for std_selfreact {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_selfreact {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		//I'm not sure how reactions work, this might fail
		let matcher = regex::Regex::new(key_base::regexes::DISCORD_ID).unwrap();
		let reaction;
		if matcher.is_match(&parameter[0]) {
			//Guild reaction
			reaction = parameter[0].clone();
		} else {
			//Normal unicode reaction
			let grapheme_count = unicode_segmentation::UnicodeSegmentation::graphemes(parameter[0].as_str(), true).count();
			if grapheme_count == 1 {
				reaction = parameter[0].clone();
			} else {
				return Err(String::from("Too many characters passed to `selfreact`"));
			}
		}
		environment.reactions_to_add.push(reaction);
		return Ok(String::new());
	}
}