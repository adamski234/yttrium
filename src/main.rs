//This is a binary made specifically for testing and should be removed at some point

#![allow(clippy::needless_return)] //I'm gonna use returns whether clippy likes it or not
use std::collections::HashMap;
use std::sync::Arc;
use ars::key_loader;
use reqwest::blocking::Client;
use serenity::{
    cache::CacheRwLock,
    client::{bridge::gateway::ShardMessenger, Context},
    http::client::Http,
    prelude::{RwLock, ShareMap},
};

use std::io::stdin;

fn main() {
	let keys = key_loader::load_keys("./keys");
	loop {
		let data = Arc::new(RwLock::new(ShareMap::custom()));
	    let (tx, _) = std::sync::mpsc::channel();
    	let shard = ShardMessenger::new(tx);
	    let shard_id = 0;
    	let client = Arc::new(Client::new());
	    let http = Arc::new(Http::new(client, ""));
    	let cache = CacheRwLock::default();
		let mut c = Context {data, shard, shard_id, http, cache};
		let env = key_base::environment::Environment {
			attachments: vec![],
			database_manager: key_base::databases::DatabaseManager::new(&String::from("guild")),
			discord_context: &mut c,
			embed: None,
			event_info: key_base::environment::events::EventType::Default,
			guild_id: String::from("guild"),
			target: String::from("channel"),
		};
		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("An error has happened while reading from the console");
		println!("{:#?}", ars::run_ars_string(input.trim().into(), &keys.keys, env)); //This will crash when I'm done
	}
}