use std::fmt::Display;

use serde::Serialize;
use uuid::Uuid;

/// User identifier for API user lookup
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserIdentifier<'a> {
	/// User's minecraft UUID
	Uuid(Uuid),
	/// User's minecraft IGN
	Nickname(&'a str),
	/// User's discord snowflake id
	DiscordId(u64),
}
impl<'a> Display for UserIdentifier<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			UserIdentifier::Uuid(uuid) => write!(f, "{}", uuid),
			UserIdentifier::Nickname(name) => write!(f, "{}", name),
			UserIdentifier::DiscordId(snowflake) => write!(f, "discord.{snowflake}"),
		}
	}
}

impl<'a> Serialize for UserIdentifier<'a> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_newtype_struct("UserIdentifier", &self.to_string())
	}
}