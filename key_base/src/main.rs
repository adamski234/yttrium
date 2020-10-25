use key_base::databases;
use std::io::stdin;
use std::io::Write;

//This is a very simple binary for manually testing and editing various modules
fn main() {
	let mut manager = databases::DatabaseManager::new(&String::from("test_db"));
	loop {
		println!("Choose your subsystem");
		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("An error has happened while reading from the console");
		let input = input.trim();
		match input {
			"database" => {
				loop {
					let mut input = String::new();
					print!("> ");
					std::io::stdout().flush().unwrap();
					stdin()
						.read_line(&mut input)
						.expect("An error has happened while reading from the console");
					let input: Vec<&str> = input.trim().split(' ').collect();
					match input[0] {
						"exit" => {
							break;
						}
						"print" => {
							if input.len() == 2 {
								println!("{:#?}", manager.get_database(&input[1].to_string()));
							} else {
								println!("{:#?}", manager);
							}
						}
						"flush" => {
							manager.write();
							manager = databases::DatabaseManager::new(&String::from("test_db"));
						}
						"make" => {
							if input.len() >= 2 {
								manager.create_database(&input[1].to_string());
							} else {
								println!("Not enough arguments");
							}
						}
						"insert_str" => {
							if input.len() >= 4 {
								match manager.get_database(&input[1].to_string()) {
									Some(db) => {
										db.write_key(input[2].to_string(), databases::StringOrArray::String(input[3].to_string()));
									}
									None => {
										println!("Database does not exist");
									}
								}
							}
						}
						"insert_arr" => {
							if input.len() >= 4 {
								match manager.get_database(&input[1].to_string()) {
									Some(db) => {
										let mut to_insert = Vec::with_capacity(input.len() - 2);
										for item in &input[3..] {
											to_insert.push(item.to_string());
										}
										db.write_key(input[2].to_string(), databases::StringOrArray::Array(to_insert));
									}
									None => {
										println!("Database does not exist");
									}
								}
							}
						}
						"delete" => {
							if input.len() == 2 {
								//Remove database
								manager.remove_database(&input[1].to_string());
							} else if input.len() >= 3 {
								if let Some(db) = manager.get_database(&input[1].to_string()) {
									db.remove_key(&input[2].to_string());
								}
								//Remove a database entry
							}
						}
						_ => {
							//
						}
					}
				}
			}
			"exit" => {
				break;
			}
			_ => {}
		};
	}
}