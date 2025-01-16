use serde::Serialize;

use crate::types::Season;

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
