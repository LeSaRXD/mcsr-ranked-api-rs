use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;

pub mod requests;
#[cfg(test)]
mod tests;

use crate::types::Season;

use super::LeaderboardUser;

/// Info about a specific season
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonInfo {
	number: Season,
	#[serde(with = "ts_seconds")]
	ends_at: DateTime<Utc>,
}
impl SeasonInfo {
	/// Season number
	pub fn number(&self) -> Season {
		self.number
	}
	/// When the season ends
	pub fn ends_at(&self) -> DateTime<Utc> {
		self.ends_at
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EloLeaderboardInfo {
	season: SeasonInfo,
	users: Box<[LeaderboardUser]>,
}
impl EloLeaderboardInfo {
	/// Season info for the leaderboard
	pub fn season(&self) -> &SeasonInfo {
		&self.season
	}

	/// Players' info
	pub fn users(&self) -> &[LeaderboardUser] {
		&self.users
	}
}
