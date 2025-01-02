use serde::Deserialize;

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
