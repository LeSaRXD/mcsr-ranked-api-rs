use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[cfg(feature = "weekly_races")]
use crate::types::WeeklyRaceId;
use crate::{
	helpers::string_u64,
	types::{MinecraftSeed, Rank, Time},
	user::UserProfile,
};

pub mod requests;
pub mod result;
#[cfg(test)]
pub mod tests;

/// Seeds for overworld, nether, end and RNG
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyRaceSeedInfo {
	#[serde(with = "string_u64")]
	pub overworld: MinecraftSeed,
	#[serde(with = "string_u64")]
	pub nether: MinecraftSeed,
	#[serde(with = "string_u64")]
	pub the_end: MinecraftSeed,
	#[serde(with = "string_u64")]
	pub rng: MinecraftSeed,
}

/// Seeds for overworld, nether, end and RNG
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyRaceLeaderboardRecord {
	/// Entry's rank, 1-indexed
	pub rank: Rank,
	/// The user the entry belongs to
	pub player: UserProfile,
	/// The final time
	pub time: Time,
	/// Whether the replay exists
	#[serde(rename = "replayExist")]
	pub replay_exists: bool,
}

/// Weekly race info
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyRaceLeaderboardInfo {
	/// Id of the race, 1-indexed
	pub id: WeeklyRaceId,
	/// The seeds for different dimensions and RNG
	pub seed: WeeklyRaceSeedInfo,
	#[serde(with = "ts_seconds")]
	/// The end date and time
	pub ends_at: DateTime<Utc>,
	/// The leaderboard for the race
	pub leaderboard: Box<[WeeklyRaceLeaderboardRecord]>,
}
impl WeeklyRaceLeaderboardInfo {
	/// The leaderboard entries as a slice
	pub fn leaderboard(&self) -> &[WeeklyRaceLeaderboardRecord] {
		&self.leaderboard
	}
}
