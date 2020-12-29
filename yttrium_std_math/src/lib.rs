#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
#[allow(unused_imports)]
use cxx::{CxxString, UniquePtr};
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn safe_create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_math {
		info: create_key_info(),
		function: key_function,
	});
}

fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("math"),
		parameters_required: vec![1],
	};
}

#[allow(non_camel_case_types)]
struct std_math<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_math<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_math<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_math<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], _environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	#[allow(unused_unsafe)]
	return Ok(unsafe { ffi::calculate(&parameter[0]).to_str().unwrap().to_string() });
}

#[cxx::bridge]
mod ffi {
	unsafe extern "C++" {
		include!("yttrium_std_math/cpp/qalc.hpp");
		fn calculate(expression: &str) -> UniquePtr<CxxString>;
	}
}