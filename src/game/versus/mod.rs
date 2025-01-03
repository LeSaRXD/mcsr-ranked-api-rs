use serde::Deserialize;

use crate::{
	types::{EloChange, RankedAndCasual, TwoUserData},
	user::UserProfile,
};

#[cfg(test)]
mod tests;

/// Results of two players playing against each other
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusResult {
	#[serde(flatten)]
	wins: TwoUserData<u32>,
	total: u32,
}
impl VersusResult {
	/// Wins for both players
	pub fn wins(&self) -> &TwoUserData<u32> {
		&self.wins
	}
	/// Total versus matches
	pub fn total(&self) -> u32 {
		self.total
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusInfo {
	players: (UserProfile, UserProfile),
	results: RankedAndCasual<VersusResult>,
	changes: TwoUserData<EloChange>,
}
impl VersusInfo {
	/// Players who versed each other
	pub fn players(&self) -> &(UserProfile, UserProfile) {
		&self.players
	}
	/// Results of both ranked and casual versus matches
	pub fn results(&self) -> &RankedAndCasual<VersusResult> {
		&self.results
	}
	/// Changes in ELO of both players
	pub fn changes(&self) -> &TwoUserData<EloChange> {
		&self.changes
	}
}
