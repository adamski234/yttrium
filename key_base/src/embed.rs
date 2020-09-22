#[derive(Debug)]
pub struct Embed {
	pub title: Option<String>,
	pub description: Option<String>,
	pub footer: Option<EmbedFooter>,
	pub author: Option<EmbedAuthor>,
	pub fields: Vec<EmbedField>,
	pub color: Option<String>,
	pub timestamp: Option<String>,
}

impl Embed {
	pub fn new() -> Self {
		return Self {
			title: None,
			description: None,
			footer: None,
			author: None,
			fields: Vec::new(),
			color: None,
			timestamp: None,
		};
	}
	pub fn add_field(&mut self, title: String, content: String, inline: bool) {
		self.fields.push(EmbedField::new(title, content, inline));
	}
}

#[derive(Debug)]
pub struct EmbedField {
	pub title: String,
	pub content: String,
	pub inline: bool,
}

impl EmbedField {
	pub fn new(title: String, content: String, inline: bool) -> Self {
		return EmbedField { title, content, inline };
	}
}

#[derive(Debug)]
pub struct EmbedFooter {
	pub text: String,
	pub icon: Option<String>,
}

#[derive(Debug)]
pub struct EmbedAuthor {
	pub name: String,
	pub url: Option<String>,
	pub avatar: Option<String>,
}
