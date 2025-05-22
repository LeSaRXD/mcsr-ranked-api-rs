use serde::Deserialize;

use crate::{
	types::{EloChange, RankedAndCasual, TwoUserData},
	user::UserProfile,
};

pub mod requests;
#[cfg(test)]
mod tests;
/// Results of two players playing against each other
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusOutcome {
	#[serde(flatten)]
	pub wins: TwoUserData<u32>,
	pub total: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusInfo {
	pub players: (UserProfile, UserProfile),
	pub results: RankedAndCasual<VersusOutcome>,
	pub changes: TwoUserData<EloChange>,
}
