use std::fmt::Debug;

use serde::Deserialize;
use serde_repr::Deserialize_repr;
use uuid::Uuid;
#[cfg(feature = "serialize")]
use {serde::Serialize, serde_repr::Serialize_repr};

use crate::types::{Elo, Rank};

pub mod identifier;
pub mod info;
pub mod requests;
#[cfg(test)]
mod tests;

#[cfg_attr(feature = "serialize", derive(Serialize_repr))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr)]
#[repr(u8)]
pub enum SupporterTier {
	None = 0,
	Stone = 1,
	Iron = 2,
	Diamond = 3,
}

/// A user's profile
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
	pub uuid: Uuid,
	pub nickname: Box<str>,
	#[serde(rename = "roleType")]
	pub supporter_tier: SupporterTier,
	#[serde(rename = "eloRate")]
	pub elo: Option<Elo>,
	#[serde(rename = "eloRank")]
	pub rank: Option<Rank>,
	pub country: Option<Box<str>>,
}
