use serde::Deserialize;

#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
	types::{EloChange, RankedAndCasual, TwoUserData},
	user::UserProfile,
};

pub mod requests;
#[cfg(test)]
mod tests;

/// Results of two players playing against each other
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusOutcome {
	#[serde(flatten)]
	pub wins: TwoUserData<u32>,
	pub total: u32,
}

/// Information about two players going against each other
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusInfo {
	// The two players
	pub players: (UserProfile, UserProfile),
	/// The results of the games played against each other
	pub results: RankedAndCasual<VersusOutcome>,
	/// The changes in ELO
	pub changes: TwoUserData<EloChange>,
}
