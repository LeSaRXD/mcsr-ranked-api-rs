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
	pub rank: Rank,
	pub season: Season,
	#[serde(with = "ts_seconds")]
	pub date: DateTime<Utc>,
	pub id: MatchId,
	pub time: Time,
	pub user: UserProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BestTimeLeaderboard(pub Box<[BestTimeInfo]>);

impl Deref for BestTimeLeaderboard {
	type Target = [BestTimeInfo];
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
