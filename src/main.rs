//This is a binary made specifically for testing and should be removed at some point
#![allow(clippy::needless_return, clippy::redundant_field_names)]
#![deny(clippy::implicit_return)]
use ars::key_loader;
use serenity::{
    client::{bridge::gateway::ShardMessenger, Context},
    prelude::{RwLock, ShareMap},
};

use std::io::stdin;

fn main() {
	let keys = key_loader::load_keys("./keys");
	loop {
		let mut c = Context {
			data: std::sync::Arc::new(RwLock::new(ShareMap::custom())),
			shard: ShardMessenger::new(std::sync::mpsc::channel().0),
			shard_id: 0,
			http: Default::default(),
			cache: Default::default(),
		};
		use key_base::environment::events::EventType;
		let env = key_base::environment::Environment::new(EventType::Default, String::from("guild"), &mut c);
		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("An error has happened while reading from the console");
		println!("{:#?}", ars::run_ars_string(input.trim().into(), &keys.keys, env)); //This will crash when I'm done
	}
}