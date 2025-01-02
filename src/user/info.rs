use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::UserProfile;
#[cfg(feature = "achievements")]
use crate::achievement::Achievement;
use crate::types::{Elo, Phase, PhasePoints, Rank, RankedAndCasual};
#[cfg(feature = "weekly_races")]
use crate::weekly_race::result::WeeklyRaceResult;

/// Displayed and total achievements of a user
#[cfg(feature = "achievements")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAchievements {
	display: Box<[Achievement]>,
	total: Box<[Achievement]>,
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTimestamps {
	#[serde(with = "ts_seconds")]
	first_online: DateTime<Utc>,
	#[serde(with = "ts_seconds")]
	last_online: DateTime<Utc>,
	#[serde(with = "ts_seconds")]
	last_ranked: DateTime<Utc>,
}
impl UserTimestamps {
	pub fn first_online(&self) -> DateTime<Utc> {
		self.first_online
	}
	pub fn last_online(&self) -> DateTime<Utc> {
		self.last_online
	}
	pub fn last_ranked(&self) -> DateTime<Utc> {
		self.last_ranked
	}
}

/// Single statistic in ranked and casual modes
pub type Stat = RankedAndCasual<Option<u64>>;

/// All of the user's statistics
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
	best_time: Stat,
	highest_win_streak: Stat,
	current_win_streak: Stat,
	played_matches: Stat,
	playtime: Stat,
	forfeits: Stat,
	completions: Stat,
	wins: Stat,
	#[serde(rename = "loses")]
	losses: Stat,
}
impl UserStats {
	pub fn best_time(&self) -> &Stat {
		&self.best_time
	}

	pub fn highest_win_streak(&self) -> &Stat {
		&self.highest_win_streak
	}

	pub fn current_win_streak(&self) -> &Stat {
		&self.current_win_streak
	}

	pub fn played_matches(&self) -> &Stat {
		&self.played_matches
	}

	pub fn playtime(&self) -> &Stat {
		&self.playtime
	}

	pub fn forfeits(&self) -> &Stat {
		&self.forfeits
	}

	pub fn completions(&self) -> &Stat {
		&self.completions
	}

	pub fn wins(&self) -> &Stat {
		&self.wins
	}

	pub fn losses(&self) -> &Stat {
		&self.losses
	}
}

/// All statistics for season and total
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatistics {
	season: UserStats,
	total: UserStats,
}
impl UserStatistics {
	pub fn season(&self) -> &UserStats {
		&self.season
	}

	pub fn total(&self) -> &UserStats {
		&self.total
	}
}

/// User's social connection
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConnection {
	id: Box<str>,
	name: Box<str>,
}
impl UserConnection {
	pub fn id(&self) -> &str {
		&self.id
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}

/// All of user's connections
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConnections {
	#[serde(default)]
	discord: Option<UserConnection>,
	#[serde(default)]
	twitch: Option<UserConnection>,
	#[serde(default)]
	youtube: Option<UserConnection>,
}
impl UserConnections {
	pub fn discord(&self) -> &Option<UserConnection> {
		&self.discord
	}

	pub fn twitch(&self) -> &Option<UserConnection> {
		&self.twitch
	}

	pub fn youtube(&self) -> &Option<UserConnection> {
		&self.youtube
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EloPointsInfo {
	#[serde(rename = "eloRate")]
	elo: Option<Elo>,
	#[serde(rename = "eloRank")]
	rank: Option<Rank>,
	#[serde(alias = "phasePoint", alias = "point")]
	points: PhasePoints,
}
impl EloPointsInfo {
	/// ELO
	pub fn elo(&self) -> Option<Elo> {
		self.elo
	}
	/// Leaderboard rank
	pub fn rank(&self) -> Option<Rank> {
		self.rank
	}
	/// Phase points
	pub fn points(&self) -> PhasePoints {
		self.points
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseInfo {
	phase: Phase,
	#[serde(flatten)]
	info: EloPointsInfo,
}

/// Season result
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSeasonResult {
	last: EloPointsInfo,
	highest: Option<Elo>,
	lowest: Option<Elo>,
	phases: Box<[PhaseInfo]>,
}
impl UserSeasonResult {
	/// Last season's ELO info
	pub fn last(&self) -> &EloPointsInfo {
		&self.last
	}
	/// Highest ELO
	pub fn highest(&self) -> &Option<Elo> {
		&self.highest
	}
	/// Lowest ELO
	pub fn lowest(&self) -> &Option<Elo> {
		&self.lowest
	}
	/// Phase info
	pub fn phases(&self) -> &[PhaseInfo] {
		&self.phases
	}
}

/// All of user's available data combined
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
	#[serde(flatten)]
	profile: UserProfile,
	#[cfg(feature = "achievements")]
	achievements: UserAchievements,
	timestamp: UserTimestamps,
	statistics: UserStatistics,
	connections: UserConnections,
	season_result: Option<UserSeasonResult>,
	#[cfg(feature = "weekly_races")]
	weekly_races: Box<[WeeklyRaceResult]>,
}
impl UserInfo {
	/// User's profile
	pub fn profile(&self) -> &UserProfile {
		&self.profile
	}
	#[cfg(feature = "achievements")]
	/// User's achievements
	pub fn achievements(&self) -> &UserAchievements {
		&self.achievements
	}
	/// User's first, last seen and last ranked timestamps
	pub fn timestamps(&self) -> &UserTimestamps {
		&self.timestamp
	}
	/// User's connections
	pub fn connections(&self) -> &UserConnections {
		&self.connections
	}
	/// User's elo and phase results
	pub fn season_result(&self) -> &Option<UserSeasonResult> {
		&self.season_result
	}
	#[cfg(feature = "weekly_races")]
	/// User's weekly race stats
	pub fn weekly_races(&self) -> &[WeeklyRaceResult] {
		&self.weekly_races
	}
}
