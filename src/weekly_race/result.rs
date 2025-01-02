use serde::Deserialize;

use crate::types::{Rank, Time, WeeklyRaceId};

/// User's weekly race result
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyRaceResult {
	id: WeeklyRaceId,
	time: Time,
	rank: Rank,
}
impl WeeklyRaceResult {
	/// Race ID, 1-indexed
	pub fn id(&self) -> WeeklyRaceId {
		self.id
	}
	/// Best time
	pub fn time(&self) -> Time {
		self.time
	}
	/// Completion rank, 1-indexed
	pub fn rank(&self) -> Rank {
		self.rank
	}
}
