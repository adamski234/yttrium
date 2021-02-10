#![allow(clippy::needless_return)]

use yttrium_key_base as key_base;
use serenity::async_trait;
use rand::Rng;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_rand {
		info: create_key_info(),
	});
}

/*
Parameters:
Optional, lowest value, default 0
Optional, highest value, default 10
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("rand"),
		parameters_required: vec![0, 1, 2],
	};
}

#[allow(non_camel_case_types)]
struct std_rand {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_rand {}
unsafe impl Sync for std_rand {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_rand {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], _environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		let lower = if !parameter.is_empty() { parameter[0].parse().unwrap() } else { 0 };
		let upper = if parameter.len() == 2 { parameter[1].parse().unwrap() } else { 10 };
		if lower > upper {
			return Err(String::from("Lower bound was higher than upper bound in `rand`"));
		}
		let result = rand::thread_rng().gen_range(lower..upper);
		return Ok(result.to_string());
	}
}