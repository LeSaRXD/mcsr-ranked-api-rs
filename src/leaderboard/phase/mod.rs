use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::Deserialize;

pub mod requests;
#[cfg(test)]
mod tests;

use crate::types::{Phase, Season};

use super::LeaderboardUser;

/// Info about a specific phase
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseInfo {
	number: Option<Phase>,
	#[serde(with = "ts_seconds_option")]
	ends_at: Option<DateTime<Utc>>,
	season: Season,
}
impl PhaseInfo {
	/// Phase number, None if target season is not current season
	pub fn number(&self) -> Option<Phase> {
		self.number
	}
	/// When the phase ends, None if already ended
	pub fn ends_at(&self) -> Option<DateTime<Utc>> {
		self.ends_at
	}
	/// What season the phase is in
	pub fn season(&self) -> Season {
		self.season
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseLeaderboardInfo {
	phase: PhaseInfo,
	users: Box<[LeaderboardUser]>,
}
impl PhaseLeaderboardInfo {
	/// Phase info for the leaderboard
	pub fn phase(&self) -> &PhaseInfo {
		&self.phase
	}
	/// Players' info
	pub fn users(&self) -> &[LeaderboardUser] {
		&self.users
	}
}
