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
pub struct LeaderboardSeasonOutcome {
	#[serde(rename = "eloRate")]
	pub elo: Elo,
	#[serde(rename = "eloRank")]
	pub rank: Rank,
	pub phase_point: PhasePoints,
}

/// User data specific to leaderboards
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardUser {
	#[serde(flatten)]
	pub profile: UserProfile,
	pub season_result: LeaderboardSeasonOutcome,
}
