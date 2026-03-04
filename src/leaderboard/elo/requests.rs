#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{Result, helpers::make_request, leaderboard::requests::GetLeaderboardInfoParams};

use super::EloLeaderboardInfo;

const BASE_URL: &str = "https://api.mcsrranked.com/leaderboard";

impl EloLeaderboardInfo {
	/// GET the user leaderboard using given `params`
	pub async fn get<'a>(params: impl Into<Option<&'a GetLeaderboardInfoParams>>) -> Result<Self> {
		make_request(BASE_URL, &[] as &[&str], params.into()).await
	}
}

#[cfg(feature = "blocking")]
impl EloLeaderboardInfo {
	/// Synchronously GET the user leaderboard using given `params`
	pub fn get_blocking<'a>(
		params: impl Into<Option<&'a GetLeaderboardInfoParams>>,
	) -> Result<Self> {
		make_request_blocking(BASE_URL, &[] as &[&str], params.into())
	}
}
