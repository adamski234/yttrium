use std::collections::HashMap;
use yttrium_key_base::databases::{
	DatabaseManager,
	Database,
};

#[path = "keys/std_attach.rs"] mod std_attach;
#[path = "keys/std_ban.rs"] mod std_ban;
#[path = "keys/std_channel.rs"] mod std_channel;
#[path = "keys/std_db_read.rs"] mod std_db_read;
#[path = "keys/std_db_write_str.rs"] mod std_db_write_str;
#[path = "keys/std_delete.rs"] mod std_delete;
#[path = "keys/std_embed.rs"] mod std_embed;
#[path = "keys/std_everyone.rs"] mod std_everyone;
#[path = "keys/std_guild.rs"] mod std_guild;
#[path = "keys/std_hasrole.rs"] mod std_hasrole;
#[path = "keys/std_joined.rs"] mod std_joined;
#[path = "keys/std_kick.rs"] mod std_kick;
#[path = "keys/std_mention.rs"] mod std_mention;
#[path = "keys/std_parameter.rs"] mod std_parameter;
#[path = "keys/std_pin.rs"] mod std_pin;
#[path = "keys/std_rand.rs"] mod std_rand;
#[path = "keys/std_redirect.rs"] mod std_redirect;
#[path = "keys/std_role.rs"] mod std_role;
#[path = "keys/std_selfdelete.rs"] mod std_selfdelete;
#[path = "keys/std_selfreact.rs"] mod std_selfreact;
#[path = "keys/std_setnickname.rs"] mod std_setnickname;
#[path = "keys/std_sleep.rs"] mod std_sleep;
#[path = "keys/std_take.rs"] mod std_take;
#[path = "keys/std_text.rs"] mod std_text;
#[path = "keys/std_trigger.rs"] mod std_trigger;
#[path = "keys/std_user.rs"] mod std_user;

pub fn load_keys<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> HashMap<String, Box<dyn yttrium_key_base::Key<Manager, DB> + Send + Sync>> {
	let mut keys = HashMap::new();
	let key = std_attach::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_ban::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_channel::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_db_read::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_db_write_str::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_delete::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_embed::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_everyone::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_guild::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_hasrole::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_joined::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_kick::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_math::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_mention::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_parameter::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_pin::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_rand::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_redirect::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_role::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_selfdelete::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_selfreact::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_setnickname::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_sleep::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_take::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_text::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_trigger::create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = std_user::create();
	keys.insert(key.get_key_info().name.clone(), key);
	return keys;
}