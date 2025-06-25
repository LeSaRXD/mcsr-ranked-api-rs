use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

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
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardUser {
	#[serde(flatten)]
	pub profile: UserProfile,
	pub season_result: LeaderboardSeasonOutcome,
}
