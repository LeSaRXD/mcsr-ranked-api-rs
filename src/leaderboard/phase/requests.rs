#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{helpers::make_request, leaderboard::requests::GetLeaderboardInfoParams, Result};

use super::PhaseLeaderboardInfo;

const BASE_URL: &str = "https://api.mcsrranked.com/phase-leaderboard";

impl PhaseLeaderboardInfo {
	/// GET the phase leaderboard using given `params`
	pub async fn get(params: Option<&GetLeaderboardInfoParams>) -> Result<Self> {
		make_request(BASE_URL, &[] as &[&str], params).await
	}
}

#[cfg(feature = "blocking")]
impl PhaseLeaderboardInfo {
	/// Synchronously GET the phase leaderboard using given `params`
	pub fn get_blocking(params: Option<&GetLeaderboardInfoParams>) -> Result<Self> {
		make_request_blocking(BASE_URL, &[] as &[&str], params)
	}
}
