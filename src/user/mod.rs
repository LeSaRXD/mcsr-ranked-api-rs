use std::fmt::Debug;

use serde::Deserialize;
use serde_repr::Deserialize_repr;
use uuid::Uuid;

use crate::types::{Elo, Rank};

pub mod identifier;
pub mod info;
pub mod requests;
#[cfg(test)]
mod tests;

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
	country: Option<Box<str>>,
}

#[cfg(test)]
impl UserProfile {
	pub(crate) fn new<U>(
		uuid: U,
		name: &str,
		supporter_tier: SupporterTier,
		elo: Option<Elo>,
		rank: Option<Rank>,
		country: Option<&str>,
	) -> Self
	where
		U: TryInto<Uuid>,
		U::Error: Debug,
	{
		Self {
			uuid: uuid.try_into().expect("Expected a valid uuid"),
			nickname: name.into(),
			supporter_tier,
			elo,
			rank,
			country: country.map(Into::into),
		}
	}
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
