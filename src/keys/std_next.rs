#![allow(clippy::needless_return)]

use yttrium_key_base as key_base;
use serenity::async_trait;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_next {
		info: create_key_info(),
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("next"),
		parameters_required: vec![1],
	};
}
#[allow(non_camel_case_types)]
struct std_next {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_next {}
unsafe impl Sync for std_next {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_next {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut key_base::environment::Environment<'_, Manager, DB>) -> Result<String, String> {
		environment.next_rule = Some(parameter[0].clone());
		return Ok(String::new());
	}
}