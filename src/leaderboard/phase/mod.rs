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
	pub number: Option<Phase>,
	#[serde(with = "ts_seconds_option")]
	pub ends_at: Option<DateTime<Utc>>,
	pub season: Season,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseLeaderboardInfo {
	pub phase: PhaseInfo,
	pub users: Box<[LeaderboardUser]>,
}
impl PhaseLeaderboardInfo {
	/// Players' info
	pub fn users(&self) -> &[LeaderboardUser] {
		&self.users
	}
}
