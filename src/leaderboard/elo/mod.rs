use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

pub mod requests;
#[cfg(test)]
mod tests;

use crate::types::Season;

use super::LeaderboardUser;

/// Info about a specific season
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonInfo {
	pub number: Season,
	#[serde(with = "ts_seconds")]
	pub ends_at: DateTime<Utc>,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EloLeaderboardInfo {
	pub season: SeasonInfo,
	pub users: Box<[LeaderboardUser]>,
}
