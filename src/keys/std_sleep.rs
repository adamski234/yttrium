#![allow(clippy::needless_return)]

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
	return Box::new(std_sleep {
		info: create_key_info(),
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("sleep"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_sleep {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_sleep {}
unsafe impl Sync for std_sleep {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_sleep {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], _environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		match humantime::parse_duration(&parameter[0]) {
			Ok(result) => {
				tokio::time::sleep(result).await;
				return Ok(String::new());
			}
			Err(error) => {
				return Err(format!("Invalid time passed to `sleep`: `{}`", error));
			}
		}
	}
}