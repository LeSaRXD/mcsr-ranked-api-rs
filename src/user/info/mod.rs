pub mod all_seasons;

use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use super::UserProfile;
#[cfg(feature = "achievements")]
use crate::achievement::Achievement;
use crate::types::{Elo, Phase, PhasePoints, Rank, RankedAndCasual};
#[cfg(feature = "weekly_races")]
use crate::weekly_race::result::WeeklyRaceResult;

/// Displayed and total achievements of a user
#[cfg(feature = "achievements")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct UserAchievements {
	/// Achievements the user chose to display on their profile
	pub display: Box<[Achievement]>,
	/// All of the user's achievements
	pub total: Box<[Achievement]>,
}
#[cfg(feature = "achievements")]
impl UserAchievements {
	/// Achievements the user chose to display on their profile
	pub fn displayed(&self) -> &[Achievement] {
		&self.display
	}
	/// All of the user's achievements
	pub fn total(&self) -> &[Achievement] {
		&self.total
	}
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTimestamps {
	#[serde(with = "ts_seconds")]
	pub first_online: DateTime<Utc>,
	#[serde(with = "ts_seconds")]
	pub last_online: DateTime<Utc>,
	#[serde(with = "ts_seconds")]
	pub last_ranked: DateTime<Utc>,
	#[serde(with = "ts_seconds_option")]
	pub next_decay: Option<DateTime<Utc>>,
}

/// All of the user's statistics
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
	pub best_time: RankedAndCasual<Option<u64>>,
	pub highest_win_streak: RankedAndCasual,
	pub current_win_streak: RankedAndCasual,
	pub played_matches: RankedAndCasual,
	pub playtime: RankedAndCasual,
	pub completion_time: RankedAndCasual,
	pub forfeits: RankedAndCasual,
	pub completions: RankedAndCasual,
	pub wins: RankedAndCasual,
	#[serde(rename = "loses")]
	pub losses: RankedAndCasual,
}

/// All statistics for season and total
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatistics {
	pub season: UserStats,
	pub total: UserStats,
}

/// User's social connection
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConnection {
	pub id: Box<str>,
	pub name: Box<str>,
}

/// All of user's connections
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConnections {
	#[serde(default)]
	pub discord: Option<UserConnection>,
	#[serde(default)]
	pub twitch: Option<UserConnection>,
	#[serde(default)]
	pub youtube: Option<UserConnection>,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EloPointsInfo {
	#[serde(rename = "eloRate")]
	pub elo: Option<Elo>,
	#[serde(rename = "eloRank")]
	pub rank: Option<Rank>,
	#[serde(alias = "phasePoint", alias = "point")]
	pub points: PhasePoints,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseInfo {
	pub phase: Phase,
	#[serde(flatten)]
	pub info: EloPointsInfo,
}
#[cfg(test)]
impl PhaseInfo {
	pub(crate) fn new(phase: Phase, elo: Elo, rank: Rank, points: PhasePoints) -> Self {
		Self {
			phase,
			info: EloPointsInfo {
				elo: Some(elo),
				rank: Some(rank),
				points,
			},
		}
	}
}

/// Season result
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSeasonOutcome {
	pub last: EloPointsInfo,
	pub highest: Option<Elo>,
	pub lowest: Option<Elo>,
	pub phases: Box<[PhaseInfo]>,
}
#[cfg(test)]
impl UserSeasonOutcome {
	pub(crate) fn new(
		last_elo: Elo,
		last_rank: Rank,
		last_points: PhasePoints,
		highest_elo: Elo,
		lowest_elo: Elo,
		phases: impl IntoIterator<Item = PhaseInfo>,
	) -> Self {
		Self {
			last: EloPointsInfo {
				elo: Some(last_elo),
				rank: Some(last_rank),
				points: last_points,
			},
			highest: Some(highest_elo),
			lowest: Some(lowest_elo),
			phases: phases.into_iter().collect(),
		}
	}
}

impl UserSeasonOutcome {
	/// Every phase info
	pub fn phases(&self) -> &[PhaseInfo] {
		&self.phases
	}
}

/// All of user's data combined
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
	#[serde(flatten)]
	pub profile: UserProfile,
	#[cfg(feature = "achievements")]
	pub achievements: UserAchievements,
	#[serde(rename = "timestamp")]
	pub timestamps: UserTimestamps,
	pub statistics: UserStatistics,
	pub connections: UserConnections,
	pub season_result: Option<UserSeasonOutcome>,
	#[cfg(feature = "weekly_races")]
	pub weekly_races: Box<[WeeklyRaceResult]>,
}
impl UserInfo {
	/// User's weekly race stats
	pub fn weekly_races(&self) -> &[WeeklyRaceResult] {
		&self.weekly_races
	}
}
