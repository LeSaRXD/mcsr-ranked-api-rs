use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

pub mod requests;
#[cfg(test)]
mod tests;

use crate::types::{Phase, Season};

use super::LeaderboardUser;

/// Info about a specific phase
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseInfo {
	/// Phase number
	pub number: Option<Phase>,
	#[serde(with = "ts_seconds_option")]
	/// The date the phase ends on
	pub ends_at: Option<DateTime<Utc>>,
	/// Season number
	pub season: Season,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseLeaderboardInfo {
	/// Phase info
	pub phase: PhaseInfo,
	/// Players' info
	pub users: Box<[LeaderboardUser]>,
}
impl PhaseLeaderboardInfo {
	/// Players' info
	pub fn users(&self) -> &[LeaderboardUser] {
		&self.users
	}
}
