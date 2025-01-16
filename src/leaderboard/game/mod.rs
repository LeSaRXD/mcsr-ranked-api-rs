use std::ops::Deref;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;

use crate::{
	types::{MatchId, Rank, Season, Time},
	user::UserProfile,
};

pub mod requests;
#[cfg(test)]
mod tests;

/// Info about a specific best time
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestTimeInfo {
	rank: Rank,
	season: Season,
	#[serde(with = "ts_seconds")]
	date: DateTime<Utc>,
	id: MatchId,
	time: Time,
	user: UserProfile,
}
impl BestTimeInfo {
	/// Rank of the best time
	pub fn rank(&self) -> Rank {
		self.rank
	}
	/// Season when the best time was set
	pub fn season(&self) -> Season {
		self.season
	}
	/// Date when the best time was set
	pub fn date(&self) -> DateTime<Utc> {
		self.date
	}
	/// Match id of the best time
	pub fn match_id(&self) -> MatchId {
		self.id
	}
	/// Final time
	pub fn time(&self) -> Time {
		self.time
	}
	/// Profile of the user who set the best time
	pub fn user(&self) -> &UserProfile {
		&self.user
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BestTimeLeaderboard(pub Box<[BestTimeInfo]>);
impl Deref for BestTimeLeaderboard {
	type Target = [BestTimeInfo];
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
