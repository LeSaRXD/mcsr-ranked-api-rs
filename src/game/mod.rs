use chrono::DateTime;
use chrono::{serde::ts_seconds, Utc};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use uuid::Uuid;

use crate::types::Time;
use crate::types::{Elo, EloChange, MatchId, Rank, Season};
use crate::user::UserProfile;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MatchCategory {
	Any,
	Custom,
	High,
	KillAllBosses,
	KillWither,
	KillElderGuardian,
	AllAdvancements,
	Half,
	PoglootQuater,
	HowDidWeGetHere,
	HeroOfTheVillage,
	Arbalistic,
	CoverMeInDebris,
	EnterNether,
	EnterEnd,
	AllSwords,
	AllMinerals,
	#[serde(rename = "FULL_IA_15_LVL")]
	FullIa15Lvl,
	AllWorkstations,
	FullInv,
	StackOfLimeWool,
	AllPortals,
	AllBlocks,
	MineAChunk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u8)]
pub enum MatchType {
	Causal = 1,
	Ranked = 2,
	Private = 3,
	Event = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchResult {
	#[serde(rename = "uuid")]
	winner_uuid: Uuid,
	time: Time,
}
impl MatchResult {
	pub fn winner_uuid(&self) -> Uuid {
		self.winner_uuid
	}
	pub fn time(&self) -> Time {
		self.time
	}
}

/// Match leaderboard ranking
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchRank {
	season: Option<Rank>,
	all_time: Option<Rank>,
}
impl MatchRank {
	/// Seasonal ranking of the match, 1-indexed
	pub fn season(&self) -> Option<Rank> {
		self.season
	}
	/// All-time ranking of the match, 1-indexed
	pub fn all_time(&self) -> Option<Rank> {
		self.all_time
	}
}

/// Match contestant's elo update
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchEloUpdate {
	#[serde(rename = "uuid")]
	player_uuid: Uuid,
	change: Option<EloChange>,
	#[serde(rename = "eloRate")]
	elo: Option<Elo>,
}
impl MatchEloUpdate {
	/// Player (whose ELO changed) miencraft UUID
	pub fn player_uuid(&self) -> Uuid {
		self.player_uuid
	}
	/// Elo change (delta)
	pub fn elo_change(&self) -> Option<EloChange> {
		self.change
	}
	/// Current elo
	pub fn elo(&self) -> Option<Elo> {
		self.elo
	}
}

/// Seed type (overworld)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeedType {
	Village,
	BurriedTreasure,
	Shipwreck,
	RuinedPortal,
	DesertTemple,
}

/// Bastion type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BastionType {
	Housing,
	Treasure,
	Bridge,
	Stables,
}

/// Match completion info
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchCompletion {
	#[serde(rename = "uuid")]
	player_uuid: Uuid,
	time: Time,
}
impl MatchCompletion {
	/// UUID of the player who completed the match
	pub fn player_uuid(&self) -> Uuid {
		self.player_uuid
	}
	/// Completion time
	pub fn time(&self) -> Time {
		self.time
	}
}

/// Match timeline event
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineEvent {
	#[serde(rename = "uuid")]
	player_uuid: Uuid,
	time: Time,
	#[serde(rename = "type")]
	id: Box<str>,
}
impl MatchTimelineEvent {
	pub fn player_uuid(&self) -> Uuid {
		self.player_uuid
	}
	pub fn time(&self) -> Time {
		self.time
	}
	pub fn id(&self) -> &str {
		&self.id
	}
}

/// Match info
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
	id: MatchId,
	#[serde(rename = "type")]
	kind: MatchType,
	season: Season,
	category: MatchCategory,
	#[serde(with = "ts_seconds")]
	date: DateTime<Utc>,
	players: Box<[UserProfile]>,
	spectators: Box<[UserProfile]>,
	result: MatchResult,
	forfeited: bool,
	decayed: bool,
	rank: MatchRank,
	changes: Box<[MatchEloUpdate]>,
	seed_type: SeedType,
	bastion_type: BastionType,
	completions: Box<[MatchCompletion]>,
	#[serde(default)]
	timelines: Option<Box<[MatchTimelineEvent]>>,
	replay_exist: bool,
}

impl MatchInfo {
	pub fn id(&self) -> MatchId {
		self.id
	}
	pub fn kind(&self) -> MatchType {
		self.kind
	}
	pub fn season(&self) -> Season {
		self.season
	}
	pub fn category(&self) -> MatchCategory {
		self.category
	}
	pub fn date(&self) -> DateTime<Utc> {
		self.date
	}
	pub fn players(&self) -> &[UserProfile] {
		&self.players
	}
	pub fn spectators(&self) -> &[UserProfile] {
		&self.spectators
	}
	pub fn result(&self) -> &MatchResult {
		&self.result
	}
	pub fn forfeited(&self) -> bool {
		self.forfeited
	}
	pub fn decayed(&self) -> bool {
		self.decayed
	}
	pub fn rank(&self) -> &MatchRank {
		&self.rank
	}
	pub fn elo_updates(&self) -> &[MatchEloUpdate] {
		&self.changes
	}
	pub fn seed_type(&self) -> SeedType {
		self.seed_type
	}
	pub fn bastion_type(&self) -> BastionType {
		self.bastion_type
	}
	pub fn completions(&self) -> &[MatchCompletion] {
		&self.completions
	}
	pub fn timeline_events(&self) -> Option<&[MatchTimelineEvent]> {
		match &self.timelines {
			Some(events) => Some(events),
			None => None,
		}
	}
	pub fn replay_exists(&self) -> bool {
		self.replay_exist
	}
}
