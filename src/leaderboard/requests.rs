use serde::Serialize;

use crate::types::Season;

/// Parameters for [`super::elo::EloLeaderboardInfo::get`] and [`super::phase::PhaseLeaderboardInfo::get`]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeaderboardInfoParams {
	pub season: Option<Season>,
}
impl GetLeaderboardInfoParams {
	pub fn season(season: Season) -> Self {
		Self {
			season: Some(season),
		}
	}
}
