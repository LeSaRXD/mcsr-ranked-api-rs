use chrono::DateTime;
use chrono::{serde::ts_seconds, Utc};
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::types::Time;
use crate::types::{Elo, EloChange, MatchId, Rank, Season};
use crate::user::UserProfile;

pub mod requests;
#[cfg(test)]
mod tests;
pub mod versus;

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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MatchSeedInfo {
	pub id: Option<Box<str>>,
	pub overworld: Option<OverworldType>,
	pub bastion: Option<BastionType>,
	pub variations: Box<[Box<str>]>,
}
impl MatchSeedInfo {
	/// Id of the seed (not the seed itself)
	pub fn id(&self) -> Option<&str> {
		self.id.as_ref().map(AsRef::as_ref)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MatchType {
	Causal = 1,
	Ranked = 2,
	Private = 3,
	Event = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchOutcome {
	#[serde(rename = "uuid")]
	pub winner_uuid: Option<Uuid>,
	pub time: Time,
}

/// Match leaderboard ranking
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchRank {
	pub season: Option<Rank>,
	pub all_time: Option<Rank>,
}

/// Match contestant's elo update
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchEloUpdate {
	#[serde(rename = "uuid")]
	pub player_uuid: Uuid,
	pub change: Option<EloChange>,
	#[serde(rename = "eloRate")]
	pub elo: Option<Elo>,
}

/// Seed type (overworld)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OverworldType {
	Village,
	BuriedTreasure,
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
	pub player_uuid: Uuid,
	pub time: Time,
}

/// Match timeline event
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineEvent {
	#[serde(rename = "uuid")]
	pub player_uuid: Uuid,
	pub time: Time,
	#[serde(rename = "type")]
	pub id: Box<str>,
}
impl MatchTimelineEvent {
	pub fn id(&self) -> &str {
		&self.id
	}
}

/// Match info
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
	pub id: MatchId,
	#[serde(rename = "type")]
	pub kind: MatchType,
	pub season: Season,
	pub category: Option<MatchCategory>,
	#[serde(with = "ts_seconds")]
	pub date: DateTime<Utc>,
	pub players: Box<[UserProfile]>,
	pub spectators: Box<[UserProfile]>,
	pub seed: Option<MatchSeedInfo>,
	pub result: MatchOutcome,
	pub forfeited: bool,
	pub decayed: bool,
	pub rank: MatchRank,
	#[serde(rename = "changes")]
	pub elo_updates: Box<[MatchEloUpdate]>,
}
impl MatchInfo {
	/// Users participating in the match
	pub fn players(&self) -> &[UserProfile] {
		&self.players
	}
	/// Users spectating the match
	pub fn spectators(&self) -> &[UserProfile] {
		&self.spectators
	}
	// The updates to the participants' ELOs
	pub fn elo_updates(&self) -> &[MatchEloUpdate] {
		&self.elo_updates
	}
}

/// Advanced (full) match info
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedMatchInfo {
	#[serde(flatten)]
	info: MatchInfo,
	completions: Box<[MatchCompletion]>,
	#[serde(rename = "timelines")]
	timeline_events: Box<[MatchTimelineEvent]>,
	#[serde(rename = "replayExist")]
	replay_exists: bool,
}
impl AdvancedMatchInfo {
	/// The completions info of the match
	pub fn completions(&self) -> &[MatchCompletion] {
		&self.completions
	}
	/// The events (achievements) timeline
	pub fn timeline_events(&self) -> &[MatchTimelineEvent] {
		&self.timeline_events
	}
}
