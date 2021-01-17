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
	return Box::new(std_everyone {
		info: create_key_info(),
	});
}

/*
Parameters:
Optional, set to "here" to send a @here instead of an @everyone
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("everyone"),
		parameters_required: vec![0, 1],
	};
}

#[allow(non_camel_case_types)]
struct std_everyone {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_everyone {}
unsafe impl Sync for std_everyone {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_everyone {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], _environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		if !parameter.is_empty() && parameter[0] == "here" {
			return Ok(String::from("@here"));
		} else {
			return Ok(String::from("@everyone"));
		}
	}
}