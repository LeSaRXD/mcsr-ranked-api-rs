use role_type::SupporterTier;
use serde::Deserialize;
use uuid::Uuid;

use crate::{Elo, Rank};

pub mod identifier;
pub mod role_type;
#[cfg(test)]
mod tests;

/// A user's profile
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
	uuid: Uuid,
	nickname: Box<str>,
	role_type: SupporterTier,
	elo_rate: Option<Elo>,
	elo_rank: Option<Rank>,
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
			role_type,
			elo_rate,
			elo_rank,
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
		self.role_type
	}
	/// The user's ELO
	pub fn elo(&self) -> Option<Elo> {
		self.elo_rate
	}
	/// The user's leaderboard rank, 1-indexed
	pub fn rank(&self) -> Option<Rank> {
		self.elo_rank
	}
}
