use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;

#[cfg(feature = "weekly_races")]
use crate::types::WeeklyRaceId;
use crate::{
	helpers::string_u64,
	types::{MinecraftSeed, Rank, Time},
	user::UserProfile,
};

pub mod result;
#[cfg(test)]
pub mod tests;

/// Seeds for overworld, nether, end and RNG
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedInfo {
	#[serde(deserialize_with = "string_u64")]
	overworld: MinecraftSeed,
	#[serde(deserialize_with = "string_u64")]
	nether: MinecraftSeed,
	#[serde(deserialize_with = "string_u64")]
	the_end: MinecraftSeed,
	#[serde(deserialize_with = "string_u64")]
	rng: MinecraftSeed,
}
impl SeedInfo {
	/// Overworld dimension seed
	pub fn overworld(&self) -> MinecraftSeed {
		self.overworld
	}
	/// Nether dimension seed
	pub fn nether(&self) -> MinecraftSeed {
		self.nether
	}
	/// The end dimension seed
	pub fn end(&self) -> MinecraftSeed {
		self.the_end
	}
	/// RNG seed
	pub fn rng(&self) -> MinecraftSeed {
		self.rng
	}
}

/// Seeds for overworld, nether, end and RNG
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyRaceLeaderboardEntry {
	rank: Rank,
	player: UserProfile,
	time: Time,
	replay_exist: bool,
}
impl WeeklyRaceLeaderboardEntry {
	/// Entry's rank, 1-indexed
	pub fn rank(&self) -> Rank {
		self.rank
	}
	/// The user owning the entry
	pub fn player(&self) -> &UserProfile {
		&self.player
	}
	/// The final time
	pub fn time(&self) -> Time {
		self.time
	}
	/// Does the replay exist
	pub fn replay_exists(&self) -> bool {
		self.replay_exist
	}
}

/// Weekly race info
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyRaceInfo {
	id: WeeklyRaceId,
	seed: SeedInfo,
	#[serde(with = "ts_seconds")]
	ends_at: DateTime<Utc>,
	leaderboard: Box<[WeeklyRaceLeaderboardEntry]>,
}
impl WeeklyRaceInfo {
	/// Id of the race, 1-indexed
	pub fn id(&self) -> &WeeklyRaceId {
		&self.id
	}
	/// The seeds for different dimensions and RNG
	pub fn seeds(&self) -> &SeedInfo {
		&self.seed
	}
	/// The end date and time
	pub fn ends_at(&self) -> &DateTime<Utc> {
		&self.ends_at
	}
	/// The leaderboard for the race
	pub fn leaderboard(&self) -> &[WeeklyRaceLeaderboardEntry] {
		&self.leaderboard
	}
}
