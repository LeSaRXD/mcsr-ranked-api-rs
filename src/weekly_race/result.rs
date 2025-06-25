use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[cfg(feature = "weekly_races")]
use crate::types::WeeklyRaceId;
use crate::types::{Rank, Time};

/// User's weekly race result
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyRaceResult {
	/// WeeklyRace ID, 1-indexed
	pub id: WeeklyRaceId,
	/// Best time
	pub time: Time,
	/// Completion rank, 1-indexed
	pub rank: Rank,
}
