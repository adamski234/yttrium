#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

use yttrium_key_base as key_base;
use serenity::async_trait;
use key_base::{
	databases::{
		DatabaseManager,
		Database,
	},
	environment::Environment,
};

pub fn create<Manager: 'static + DatabaseManager<DB>, DB: 'static + Database>() -> Box<dyn key_base::Key<Manager, DB> + Send + Sync> {
	return Box::new(std_embed {
		info: create_key_info(),
	});
}

/*
Parameters:
Required, what operation to do. Possible values are: author, color, description, field, footer, image, title, delete
Optional, first parameter
Optional, second parameter
Optional, third parameter to `field`, 0 or empty for 
*/
fn create_key_info() -> key_base::KeyInfo {
	return key_base::KeyInfo {
		name: String::from("embed"),
		parameters_required: vec![1, 2, 3, 4],
	};
}

#[allow(non_camel_case_types)]
struct std_embed {
	pub info: key_base::KeyInfo,
}

unsafe impl Send for std_embed {}
unsafe impl Sync for std_embed {}

#[async_trait]
impl<Manager: DatabaseManager<DB>, DB: Database> key_base::Key<Manager, DB> for std_embed {
	fn get_key_info(&self) -> &key_base::KeyInfo {
		return &self.info;
	}

	async fn run_key(&self, parameter: &[String], environment: &mut Environment<'_, Manager, DB>) -> Result<String, String> {
		match parameter[0].as_str() {
			"delete" => {
				environment.embed = None;
			}
			"author" => {
				match &mut environment.embed {
					Some(embed) => {
						match parameter.len() {
							2 => {
								embed.author(|author| {
									return author.name(parameter[1].clone());
								});
							}
							3 => {
								embed.author(|author| {
									return author.name(parameter[1].clone()).icon_url(parameter[2].clone());
								});
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:author`"));
							}
						}
					}
					None => {
						let mut embed = serenity::builder::CreateEmbed::default();
						match parameter.len() {
							2 => {
								embed.author(|author| {
									return author.name(parameter[1].clone());
								});
							}
							3 => {
								embed.author(|author| {
									return author.name(parameter[1].clone()).icon_url(parameter[2].clone());
								});
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:author`"));
							}
						}
						environment.embed = Some(embed);
					}
				}
			}
			"color" => {
				match &mut environment.embed {
					Some(embed) => {
						match parameter.len() {
							2 => {
								let r = std::primitive::u8::from_str_radix(&parameter[1][0..2], 16);
								let g = std::primitive::u8::from_str_radix(&parameter[1][2..4], 16);
								let b = std::primitive::u8::from_str_radix(&parameter[1][4..6], 16);
								if r.is_err() || g.is_err() || b.is_err() {
									return Err(String::from("Invalid color passed to `embed:color`"));
								}
								let r = r.unwrap();
								let g = g.unwrap();
								let b = b.unwrap();
								embed.color((r, g, b));
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:color`"));
							}
						}
					}
					None => {
						let mut embed = serenity::builder::CreateEmbed::default();
						match parameter.len() {
							2 => {
								let r = std::primitive::u8::from_str_radix(&parameter[1][0..2], 16);
								let g = std::primitive::u8::from_str_radix(&parameter[1][2..4], 16);
								let b = std::primitive::u8::from_str_radix(&parameter[1][4..6], 16);
								if r.is_err() || g.is_err() || b.is_err() {
									return Err(String::from("Invalid color passed to `embed:color`"));
								}
								let r = r.unwrap();
								let g = g.unwrap();
								let b = b.unwrap();
								embed.color((r, g, b));
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:color`"));
							}
						}
						environment.embed = Some(embed);
					}
				}
			}
			"description" => {
				match &mut environment.embed {
					Some(embed) => {
						match parameter.len() {
							2 => {
								embed.description(parameter[1].clone());
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:description`"));
							}
						}
					}
					None => {
						let mut embed = serenity::builder::CreateEmbed::default();
						match parameter.len() {
							2 => {
								embed.description(parameter[1].clone());
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:description`"));
							}
						}
						environment.embed = Some(embed);
					}
				}
			}
			"field" => {
				match &mut environment.embed {
					Some(embed) => {
						match parameter.len() {
							3 => {
								embed.field(parameter[1].clone(), parameter[2].clone(), false);
							}
							4 => {
								let inline = !(parameter[3].is_empty() || parameter[3] == "0");
								embed.field(parameter[1].clone(), parameter[2].clone(), inline);
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:field`"));
							}
						}
					}
					None => {
						let mut embed = serenity::builder::CreateEmbed::default();
						match parameter.len() {
							3 => {
								embed.field(parameter[1].clone(), parameter[2].clone(), false);
							}
							4 => {
								let inline = !(parameter[3].is_empty() || parameter[3] == "0");
								embed.field(parameter[1].clone(), parameter[2].clone(), inline);
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:field`"));
							}
						}
						environment.embed = Some(embed);
					}
				}
			}
			"footer" => {
				match &mut environment.embed {
					Some(embed) => {
						match parameter.len() {
							2 => {
								embed.footer(|footer| {
									return footer.text(parameter[1].clone());
								});
							}
							3 => {
								embed.footer(|footer| {
									return footer.text(parameter[1].clone()).icon_url(parameter[2].clone());
								});
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:footer`"));
							}
						}
					}
					None => {
						let mut embed = serenity::builder::CreateEmbed::default();
						match parameter.len() {
							2 => {
								embed.footer(|footer| {
									return footer.text(parameter[1].clone());
								});
							}
							3 => {
								embed.footer(|footer| {
									return footer.text(parameter[1].clone()).icon_url(parameter[2].clone());
								});
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:footer`"));
							}
						}
						environment.embed = Some(embed);
					}
				}
			}
			"image" => {
				match &mut environment.embed {
					Some(embed) => {
						match parameter.len() {
							2 => {
								embed.image(parameter[1].clone());
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:image`"));
							}
						}
					}
					None => {
						let mut embed = serenity::builder::CreateEmbed::default();
						match parameter.len() {
							2 => {
								embed.image(parameter[1].clone());
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:image`"));
							}
						}
						environment.embed = Some(embed);
					}
				}
			}
			"title" => {
				match &mut environment.embed {
					Some(embed) => {
						match parameter.len() {
							2 => {
								embed.title(parameter[1].clone());
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:title`"));
							}
						}
					}
					None => {
						let mut embed = serenity::builder::CreateEmbed::default();
						match parameter.len() {
							2 => {
								embed.title(parameter[1].clone());
							}
							_ => {
								return Err(String::from("Invalid amount of parameters passed to `embed:title`"));
							}
						}
						environment.embed = Some(embed);
					}
				}
			}
			_ => {
				return Err(String::from("Invalid operation passed to `embed`"));
			}
		}
		return Ok(String::from(""));
	}
}