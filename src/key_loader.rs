use std::collections::HashMap;
use yttrium_key_base::databases::{
	DatabaseManager,
	Database,
};

pub fn load_keys<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> HashMap<String, Box<dyn yttrium_key_base::Key<Manager, DB> + Send + Sync>> {
	let mut keys = HashMap::new();
	let key = yttrium_std_attach::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_ban::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_channel::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_db_read::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_db_write_str::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_delete::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_everyone::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_guild::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_hasrole::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_joined::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_kick::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_math::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_mention::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_parameter::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_pin::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_rand::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_redirect::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_role::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_selfdelete::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_selfreact::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_setnickname::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_sleep::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_take::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_text::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_trigger::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	let key = yttrium_std_user::safe_create();
	keys.insert(key.get_key_info().name.clone(), key);
	return keys;
}