#![allow(clippy::needless_return)]

use yttrium_key_base as key_base;
#[allow(unused_imports)]
use cxx::{CxxString, UniquePtr};
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};
use serenity::async_trait;

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_math {
		info: create_key_info(),
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("math"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_math {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_math {}
unsafe impl Sync for std_math {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_math {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], _environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		#[allow(unused_unsafe)]
		return Ok(unsafe { ffi::calculate(&parameter[0]).to_str().unwrap().to_string() });
	}
}

#[cxx::bridge]
mod ffi {
	unsafe extern "C++" {
		include!("yttrium_std_math/cpp/qalc.hpp");
		fn calculate(expression: &str) -> UniquePtr<CxxString>;
	}
}