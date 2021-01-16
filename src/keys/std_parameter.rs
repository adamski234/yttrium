#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
use yttrium_key_base as key_base;
use key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_parameter {
		info: create_key_info(),
		function: key_function,
	});
}


/*
Parameters:
Optional, the string to split on. If empty returns the entire parameter string
Required, the index of the split string to return
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("parameter"),
		parameters_required: vec![0, 2],
	};
}

#[allow(non_camel_case_types)]
struct std_parameter<Manager: DatabaseManager<DB>, DB: Database> {
	pub info: key_base::KeyInfo,
	pub function: fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String>,
}

unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Send for std_parameter<Manager, DB> {}
unsafe impl<Manager: DatabaseManager<DB>, DB: Database> Sync for std_parameter<Manager, DB> {}

impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_parameter<Manager, DB> {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	fn get_key_function(&self) -> fn(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
		return self.function;
	}
}

fn key_function<Manager: DatabaseManager<DB>, DB: Database>(parameter: &[String], environment: &mut key_base::environment::Environment<Manager, DB>) -> Result<String, String> {
	if let key_base::environment::events::EventType::Message(event) = &mut environment.event_info {
		if parameter.is_empty() {
			return Ok(event.parameter.clone());
		} else {
			match parameter[1].parse::<usize>() {
				Ok(index) => {
					if !event.split_parameters.contains_key(&parameter[0]) {
						//This is hacky and I don't like this
						let split = event.parameter.split(&parameter[0]).map(String::from).collect();
						event.split_parameters.insert(parameter[0].clone(), split);
					}
					let split = event.split_parameters.get(&parameter[0]).unwrap();
					if split.len() >= index + 1 {
						return Ok(split[index].clone());
					} else {
						return Err(format!("`parameter` split by `{}` didn't have `{}` elements", parameter[0], index + 1));
					}
				}
				Err(error) => {
					return Err(format!("Invalid number passed to `parameter`: `{}`", error.to_string()));
				}
			}
		}
	} else {
		return Err(String::from("`parameter` called on invalid event type"));
	}
}