//This is a binary made specifically for testing and should be removed at some point
#![allow(clippy::needless_return, clippy::redundant_field_names)]
#![deny(clippy::implicit_return)]
use ars::key_loader;
use serenity::{
    client::{bridge::gateway::ShardMessenger, Context},
	prelude::{RwLock, TypeMap},
	model::id::GuildId,
};

use std::io::stdin;

fn main() {
	let keys = key_loader::load_keys("./keys");
	loop {
		let mut c = Context {
			data: std::sync::Arc::new(RwLock::new(TypeMap::new())),
			shard: ShardMessenger::new(futures::channel::mpsc::unbounded().0),
			shard_id: 0,
			http: Default::default(),
			cache: Default::default(),
		};
		use key_base::environment::events::EventType;
		let manager = Box::new(key_base::databases::JSONDatabaseManager::new("guild"));
		let env = key_base::environment::Environment::new(EventType::Default, GuildId::from(1), &mut c, manager);
		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("An error has happened while reading from the console");
		println!("{:#?}", ars::run_ars_string(input.trim().into(), &keys.keys, env)); //This will crash when I'm done
	}
}