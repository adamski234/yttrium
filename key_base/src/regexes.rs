//! Module with shared regexes to simplify matching

/// Regex describing a discord snowflake
pub static DISCORD_ID: &str = r"\d{18}";

#[cfg(test)]
mod tests {
	#[test]
	fn id_regex() {
		use regex::Regex;
		let matcher = Regex::new(super::DISCORD_ID).unwrap();
		assert!(matcher.is_match("123456789123456789"));
		assert!(!matcher.is_match("12345678912345678"));
	}
}