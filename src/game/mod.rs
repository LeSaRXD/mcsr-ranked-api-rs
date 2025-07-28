use chrono::DateTime;
use chrono::{serde::ts_seconds, Utc};
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::types::Time;
use crate::types::{Elo, EloChange, MatchId, Rank, Season};
use crate::user::UserProfile;
#[cfg(feature = "variations")]
use crate::variations::Variation;

pub mod requests;
#[cfg(test)]
mod tests;
pub mod versus;

#[cfg_attr(feature = "serialize", derive(Serialize))]
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

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MatchSeedInfo {
	/// Id of the seed (not the seed itself)
	pub id: Option<Box<str>>,
	pub overworld: Option<OverworldType>,
	#[serde(rename = "nether")]
	pub bastion: Option<BastionType>,
	/// The heights of the 4 zero-cyclable end towers
	#[serde(default)]
	pub end_towers: Option<[u8; 4]>,
	#[serde(default)]
	#[cfg(feature = "variations")]
	pub variations: Box<[Variation]>,
	#[cfg(not(feature = "variations"))]
	pub variations: Box<[Box<str>]>,
}
impl MatchSeedInfo {
	/// Id of the seed (not the seed itself)
	pub fn id(&self) -> Option<&str> {
		self.id.as_ref().map(AsRef::as_ref)
	}
	#[cfg(not(feature = "variations"))]
	pub fn variations(&self) -> &[Box<str>] {
		self.variations.as_ref()
	}
	#[cfg(feature = "variations")]
	pub fn variations(&self) -> &[Variation] {
		self.variations.as_ref()
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

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchOutcome {
	#[serde(rename = "uuid")]
	pub winner_uuid: Option<Uuid>,
	pub time: Time,
}

/// Match leaderboard ranking
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchRank {
	pub season: Option<Rank>,
	pub all_time: Option<Rank>,
}

/// Match contestant's elo update
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BastionType {
	Housing,
	Treasure,
	Bridge,
	Stables,
}

/// Match completion info
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchCompletion {
	#[serde(rename = "uuid")]
	pub player_uuid: Uuid,
	pub time: Time,
}

/// Match timeline event
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
	/// Get the overworld type of the match
	pub fn overworld_type(&self) -> Option<OverworldType> {
		self.seed.as_ref().and_then(|s| s.overworld)
	}
	/// Get the bastion type of the match
	pub fn bastion_type(&self) -> Option<BastionType> {
		self.seed.as_ref().and_then(|s| s.bastion)
	}
}

/// Advanced (full) match info
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedMatchInfo {
	/// The base info about the match
	#[serde(flatten)]
	info: MatchInfo,
	/// Which players completed the match
	completions: Box<[MatchCompletion]>,
	#[serde(rename = "timelines")]
	/// Advancements and other events that happened during the match
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
