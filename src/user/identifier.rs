use std::{fmt::Display, num::NonZeroU64};

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

impl From<u64> for UserIdentifier<'_> {
	fn from(value: u64) -> Self {
		Self::DiscordId(value)
	}
}
impl From<NonZeroU64> for UserIdentifier<'_> {
	fn from(value: NonZeroU64) -> Self {
		Self::DiscordId(value.get())
	}
}
impl From<Uuid> for UserIdentifier<'_> {
	fn from(value: Uuid) -> Self {
		Self::Uuid(value)
	}
}
impl<'a> From<&'a str> for UserIdentifier<'a> {
	fn from(value: &'a str) -> Self {
		Self::Nickname(value)
	}
}

impl Display for UserIdentifier<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			Self::Uuid(uuid) => write!(f, "{uuid}"),
			Self::Nickname(name) => write!(f, "{name}"),
			Self::DiscordId(snowflake) => write!(f, "discord.{snowflake}"),
		}
	}
}

impl Serialize for UserIdentifier<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_newtype_struct("UserIdentifier", &self.to_string())
	}
}
