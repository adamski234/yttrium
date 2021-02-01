#![allow(clippy::needless_return, clippy::redundant_field_names)]
#![deny(clippy::implicit_return)]

pub mod environment;
pub mod databases;
pub mod regexes;

use databases::{Database, DatabaseManager};
use serenity::async_trait;
/// Trait used for implementing keys
#[async_trait]
pub trait Key<Manager: DatabaseManager<DB>, DB: Database> {
	/// Returns a reference to a [KeyInfo] describing the key
	fn get_key_info(&self) -> &KeyInfo;
	/// Returns the key function that gets called during interpretation
	async fn run_key(&self, parameter: &[String], environment: &mut environment::Environment<'_, Manager, DB>) -> Result<String, String>;
}

/// Struct describing the key
#[derive(Clone, Debug)]
pub struct KeyInfo {
	/// How many parameters the key takes
	/// Contains every possible variation, like `[1, 3, 4]`
	pub parameters_required: Vec<usize>,
	/// The key's name
	/// Must be unique
	pub name: String,
}