use serde::Deserialize;

use crate::{
	types::{Elo, PhasePoints, Rank},
	user::UserProfile,
};

pub mod elo;
#[cfg(feature = "matches")]
pub mod game;
pub mod phase;
pub mod requests;

/// Season result specific to leaderboards
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardSeasonResult {
	#[serde(rename = "eloRate")]
	elo: Elo,
	#[serde(rename = "eloRank")]
	rank: Rank,
	phase_point: PhasePoints,
}
impl LeaderboardSeasonResult {
	/// Elo of the user
	pub fn elo(&self) -> Elo {
		self.elo
	}
	/// Leaderboard rank of the user
	pub fn rank(&self) -> Rank {
		self.rank
	}
	/// Phase points of the user
	pub fn phase_points(&self) -> PhasePoints {
		self.phase_point
	}
}

/// User data specific to leaderboards
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardUser {
	#[serde(flatten)]
	profile: UserProfile,
	season_result: LeaderboardSeasonResult,
}
impl LeaderboardUser {
	/// Profile of the user
	pub fn profile(&self) -> &UserProfile {
		&self.profile
	}
	/// Season result of the user
	pub fn season_result(&self) -> &LeaderboardSeasonResult {
		&self.season_result
	}
}
