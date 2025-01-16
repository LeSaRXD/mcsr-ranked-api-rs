use serde::Serialize;

#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{helpers::make_request, types::Season};

use super::EloLeaderboardInfo;

const BASE_URL: &str = "https://mcsrranked.com/api/leaderboard";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEloLeaderboardInfoParams {
	pub season: Option<Season>,
}
impl GetEloLeaderboardInfoParams {
	pub fn season(season: Season) -> Self {
		Self {
			season: Some(season),
		}
	}
}

impl EloLeaderboardInfo {
	/// GET the user leaderboard using given `params`
	pub async fn get(params: &Option<GetEloLeaderboardInfoParams>) -> crate::Result<Self> {
		make_request(BASE_URL, &[] as &[&str], params).await
	}
}

#[cfg(feature = "blocking")]
impl EloLeaderboardInfo {
	/// Synchronously GET the user leaderboard using given `params`
	pub fn get_blocking(params: &Option<GetEloLeaderboardInfoParams>) -> crate::Result<Self> {
		make_request_blocking(BASE_URL, &[] as &[&str], params)
	}
}
