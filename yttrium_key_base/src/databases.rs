/// Trait for accessing databases
pub trait DatabaseManager<T: Database> {
	/// Gets the database from the manager, creating it if it does not exist
	fn get_database(&mut self, name: &str) -> T;
	/// Deletes a database from the manager
	fn remove_database(&mut self, name: &str);
	/// Removes all entries from a database
	fn clear_database(&mut self, name: &str);
}

/// Trait for accessing data from a database
pub trait Database {
	/// Retrieves a key from the database
	fn get_key(&self, name: &str) -> Option<StringOrArray>;
	/// Inserts a key into the database, overwriting the old one if it already existed
	fn write_key(&mut self, name: String, value: StringOrArray);
	/// Deletes a key from the database
	fn remove_key(&mut self, name: &str);
	/// Checks if a key exists in the database
	fn key_exists(&self, name: &str) -> bool;
}

pub struct Placeholder;

impl Database for Placeholder {
	fn get_key(&self, _name: &str) -> Option<StringOrArray> {
		unimplemented!("`Placeholder is a placeholder")
    }
	
    fn write_key(&mut self, _name: String, _value: StringOrArray) {
		unimplemented!("`Placeholder is a placeholder")
    }
	
    fn remove_key(&mut self, _name: &str) {
		unimplemented!("`Placeholder is a placeholder")
    }
	
    fn key_exists(&self, _name: &str) -> bool {
		unimplemented!("`Placeholder is a placeholder")
    }
}

impl DatabaseManager<Placeholder> for Placeholder {
    fn get_database(&mut self, _name: &str) -> Placeholder {
        unimplemented!("`Placeholder is a placeholder")
    }

    fn remove_database(&mut self, _name: &str) {
        unimplemented!("`Placeholder is a placeholder")
    }

    fn clear_database(&mut self, _name: &str) {
        unimplemented!("`Placeholder is a placeholder")
    }
}

/// Enum used for differentiation between a string and an array
#[derive(Debug, PartialEq, Clone)]
pub enum StringOrArray {
	String(String),
	Array(Vec<String>),
}