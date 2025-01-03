use std::{
	error::Error,
	fmt::{self, Display},
	marker::PhantomData,
	str::FromStr,
};

use serde::{
	de::{self, MapAccess, Visitor},
	Deserialize, Deserializer,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
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
/// Result with this crate's own `Error` type
pub type ReqResult<T> = Result<T, ReqError>;

#[doc(hidden)]
/// Error returned by a request to the API
#[derive(Debug)]
pub enum ReqError {
	/// Ranked API error
	Api(Option<Box<str>>),
	/// Reqwest library error
	Reqwest(reqwest::Error),
}

impl PartialEq for ReqError {
	fn eq(&self, other: &Self) -> bool {
		use ReqError::*;

		match (self, other) {
			(Api(lhs), Api(rhs)) => lhs == rhs,
			(Reqwest(lhs), Reqwest(rhs)) => lhs.to_string() == rhs.to_string(),
			_ => false,
		}
	}
}
impl Eq for ReqError {}

impl From<reqwest::Error> for ReqError {
	fn from(value: reqwest::Error) -> Self {
		Self::Reqwest(value)
	}
}

impl Display for ReqError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use ReqError::*;

		match self {
			Api(Some(api_err)) => write!(f, "API Error: {}", api_err),
			Api(None) => f.write_str("API Error! (No message)"),
			Reqwest(req_err) => write!(f, "Reqwest Error: {}", req_err),
		}
	}
}
impl Error for ReqError {}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "status", content = "data", rename_all = "camelCase")]
pub(crate) enum DeReqResult<T> {
	Success(T),
	Error(Option<Box<str>>),
}

impl<T> From<DeReqResult<T>> for ReqResult<T> {
	fn from(value: DeReqResult<T>) -> Self {
		match value {
			DeReqResult::Success(t) => Ok(t),
			DeReqResult::Error(e) => Err(ReqError::Api(e)),
		}
	}
}

/// Container for ranked and casual values
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RankedAndCasual<T> {
	ranked: T,
	casual: T,
}
impl<T> RankedAndCasual<T> {
	/// Value for ranked
	pub fn ranked(&self) -> &T {
		&self.ranked
	}
	/// Value for casual
	pub fn casual(&self) -> &T {
		&self.casual
	}
}

/// Container for UUIDs and data of exactly two players
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwoUserData<T> {
	user_1_uuid: Uuid,
	user_1_data: T,
	user_2_uuid: Uuid,
	user_2_data: T,
}
impl<T> TwoUserData<T> {
	/// First user's UUID and data
	pub fn user_1(&self) -> (Uuid, &T) {
		(self.user_1_uuid, &self.user_1_data)
	}
	/// Second users's UUID and data
	pub fn user_2(&self) -> (Uuid, &T) {
		(self.user_2_uuid, &self.user_2_data)
	}
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for TwoUserData<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct TwoUserDataVisitor<T> {
			_phantom: PhantomData<T>,
		}

		impl<'de, T: Deserialize<'de>> Visitor<'de> for TwoUserDataVisitor<T> {
			type Value = TwoUserData<T>;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("struct TwoUserData")
			}

			fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
			where
				V: MapAccess<'de>,
			{
				let mut user_1 = None;
				let mut user_2 = None;

				while let Some(key) = map.next_key()? {
					if let Ok(uuid) = Uuid::from_str(key) {
						match (&user_1, &user_2) {
							(None, None) => user_1 = Some((uuid, map.next_value()?)),
							(Some(_), None) => user_2 = Some((uuid, map.next_value()?)),
							_ => return Err(de::Error::duplicate_field("user_2")),
						}
					}
				}

				let user_1 = user_1.ok_or_else(|| de::Error::missing_field("user_1"))?;
				let user_2 = user_2.ok_or_else(|| de::Error::missing_field("user_2"))?;

				Ok(TwoUserData {
					user_1_uuid: user_1.0,
					user_1_data: user_1.1,
					user_2_uuid: user_2.0,
					user_2_data: user_2.1,
				})
			}
		}

		deserializer.deserialize_map(TwoUserDataVisitor {
			_phantom: PhantomData::<T>,
		})
	}
}
