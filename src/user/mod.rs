use role_type::SupporterTier;
use serde::Deserialize;
use uuid::Uuid;

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
	elo_rate: Option<u16>,
	elo_rank: Option<u32>,
}

impl UserProfile {
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
	pub fn elo(&self) -> Option<u16> {
		self.elo_rate
	}
	/// The user's leaderboard rank, 1-indexed
	pub fn rank(&self) -> Option<u32> {
		self.elo_rank
	}
}
