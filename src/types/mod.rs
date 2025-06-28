use std::{
	collections::HashMap,
	fmt::{self, Display},
};

use serde::{de, Deserialize, Deserializer};
#[cfg(feature = "serialize")]
use serde::{ser::SerializeMap, Serialize};
use uuid::Uuid;

#[cfg(test)]
mod tests;

pub type Elo = u16;
pub type EloChange = i16;
pub type PhasePoints = u16;
pub type Rank = u32;
pub type Season = u8;
pub type Phase = u8;
#[cfg(feature = "matches")]
pub type MatchId = u64;
#[cfg(feature = "weekly_races")]
pub type WeeklyRaceId = u32;
pub type MinecraftSeed = u64;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct Time(pub u64);

impl Time {
	pub const fn new(value: u64) -> Self {
		Self(value)
	}
	pub const fn millis(&self) -> u64 {
		self.0 % 1000
	}
	pub const fn seconds(&self) -> u64 {
		(self.0 / 1000) % 60
	}
	pub const fn minutes(&self) -> u64 {
		(self.0 / 60000) % 60
	}
	pub const fn hours(&self) -> u64 {
		self.0 / 3600000
	}
}

#[doc(hidden)]
/// Result with this crate's own `Error` type as default
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[doc(hidden)]
/// Error returned by a request to the API
#[derive(Debug)]
pub enum Error {
	/// Ranked API error
	Api(Option<Box<str>>),
	/// Reqwest library error
	Reqwest(reqwest::Error),
}

impl PartialEq for Error {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Error::Api(lhs), Error::Api(rhs)) => lhs == rhs,
			(Error::Reqwest(lhs), Error::Reqwest(rhs)) => lhs.to_string() == rhs.to_string(),
			_ => false,
		}
	}
}
impl Eq for Error {}

impl From<reqwest::Error> for Error {
	fn from(value: reqwest::Error) -> Self {
		Self::Reqwest(value)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::Api(Some(api_err)) => write!(f, "API Error: {api_err}"),
			Error::Api(None) => f.write_str("API Error! (No message)"),
			Error::Reqwest(req_err) => write!(f, "Reqwest Error: {req_err}"),
		}
	}
}
impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "status", content = "data", rename_all = "camelCase")]
pub(crate) enum DeResult<T> {
	Success(T),
	Error(Option<Box<str>>),
}

impl<T> From<DeResult<T>> for Result<T> {
	fn from(value: DeResult<T>) -> Self {
		match value {
			DeResult::Success(t) => Ok(t),
			DeResult::Error(e) => Err(Error::Api(e)),
		}
	}
}

/// Container for ranked and casual values
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RankedAndCasual<T> {
	pub ranked: T,
	pub casual: T,
}

/// Container for UUIDs and data of exactly two players
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwoUserData<T> {
	pub user_1_uuid: Uuid,
	pub user_1_data: T,
	pub user_2_uuid: Uuid,
	pub user_2_data: T,
}
impl<T> TwoUserData<T> {
	/// First user's UUID and data
	pub fn user_1(&self) -> (Uuid, &T) {
		(self.user_1_uuid, &self.user_1_data)
	}
	/// Second user's UUID and data
	pub fn user_2(&self) -> (Uuid, &T) {
		(self.user_2_uuid, &self.user_2_data)
	}
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for TwoUserData<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let mut entries = HashMap::<Uuid, T>::deserialize(deserializer)?
			.into_iter()
			.collect::<Vec<_>>();
		entries.sort_by_key(|(uuid, _)| *uuid);
		let [(user_1_uuid, user_1_data), (user_2_uuid, user_2_data)] = entries
			.try_into()
			.map_err(|err: Vec<_>| de::Error::invalid_length(err.len(), &"2"))?;
		Ok(TwoUserData {
			user_1_uuid,
			user_1_data,
			user_2_uuid,
			user_2_data,
		})
	}
}
#[cfg(feature = "serialize")]
impl<T: Serialize> Serialize for TwoUserData<T> {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut map = serializer.serialize_map(Some(2))?;
		map.serialize_entry(&self.user_1_uuid, &self.user_1_data)?;
		map.serialize_entry(&self.user_2_uuid, &self.user_2_data)?;
		map.end()
	}
}
