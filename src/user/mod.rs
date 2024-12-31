use serde::Deserialize;
use uuid::Uuid;

use crate::types::{Elo, Rank};

pub mod identifier;
#[cfg(test)]
mod tests;
use serde_repr::Deserialize_repr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr)]
#[repr(u8)]
pub enum SupporterTier {
	None = 0,
	Stone = 1,
	Iron = 2,
	Diamond = 3,
}

/// A user's profile
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
	uuid: Uuid,
	nickname: Box<str>,
	#[serde(rename = "roleType")]
	supporter_tier: SupporterTier,
	#[serde(rename = "eloRate")]
	elo: Option<Elo>,
	#[serde(rename = "eloRank")]
	rank: Option<Rank>,
}

impl UserProfile {
	#[cfg(test)]
	pub(crate) fn new(
		uuid: Uuid,
		name: &str,
		role_type: SupporterTier,
		elo_rate: Option<Elo>,
		elo_rank: Option<Rank>,
	) -> Self {
		Self {
			uuid,
			nickname: name.into(),
			supporter_tier: role_type,
			elo: elo_rate,
			rank: elo_rank,
		}
	}
	/// The user's minecraft UUID
	pub fn uuid(&self) -> Uuid {
		self.uuid
	}
	/// The user's minecraft IGN
	pub fn nickname(&self) -> &str {
		&self.nickname
	}
	/// The user's supporter tier
	pub fn supporter_tier(&self) -> SupporterTier {
		self.supporter_tier
	}
	/// The user's ELO
	pub fn elo(&self) -> Option<Elo> {
		self.elo
	}
	/// The user's leaderboard rank, 1-indexed
	pub fn rank(&self) -> Option<Rank> {
		self.rank
	}
}
