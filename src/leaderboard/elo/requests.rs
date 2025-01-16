#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{helpers::make_request, leaderboard::requests::GetLeaderboardInfoParams};

use super::EloLeaderboardInfo;

const BASE_URL: &str = "https://mcsrranked.com/api/leaderboard";

impl EloLeaderboardInfo {
	/// GET the user leaderboard using given `params`
	pub async fn get(params: &Option<GetLeaderboardInfoParams>) -> crate::Result<Self> {
		make_request(BASE_URL, &[] as &[&str], params).await
	}
}

#[cfg(feature = "blocking")]
impl EloLeaderboardInfo {
	/// Synchronously GET the user leaderboard using given `params`
	pub fn get_blocking(params: &Option<GetLeaderboardInfoParams>) -> crate::Result<Self> {
		make_request_blocking(BASE_URL, &[] as &[&str], params)
	}
}
