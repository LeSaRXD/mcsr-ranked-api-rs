use serde::Serialize;

#[cfg(feature = "matches")]
use crate::game::{requests::GetMatchesParams, MatchInfo};
#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{helpers::make_request, types::Season, user::identifier::UserIdentifier, Result};

use super::VersusInfo;

const BASE_URL: &str = "https://api.mcsrranked.com/users/{}/versus/{}";
const MATCHES_URL: &str = "https://api.mcsrranked.com/users/{}/versus/{}";

/// Parameters for [`VersusInfo::get`]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVersusInfoParams {
	pub season: Option<Season>,
}
impl GetVersusInfoParams {
	pub fn season(season: Season) -> Self {
		Self {
			season: Some(season),
		}
	}
}

impl VersusInfo {
	/// GET versus info between two players using `params`
	pub async fn get<'a>(
		user_1: &UserIdentifier<'a>,
		user_2: &UserIdentifier<'a>,
		params: Option<&GetVersusInfoParams>,
	) -> Result<Self> {
		make_request(BASE_URL, [&user_1.to_string(), &user_2.to_string()], params).await
	}
	#[cfg(feature = "matches")]
	/// GET versus matches between two players using `params`
	pub async fn get_matches<'a>(
		user_1: &UserIdentifier<'a>,
		user_2: &UserIdentifier<'a>,
		params: Option<&GetMatchesParams>,
	) -> Result<Vec<MatchInfo>> {
		make_request(
			MATCHES_URL,
			[&user_1.to_string(), &user_2.to_string()],
			params,
		)
		.await
	}
}

#[cfg(feature = "blocking")]
impl VersusInfo {
	/// Synchronously GET versus info between two players using `params`
	pub fn get_blocking<'a>(
		user_1: &UserIdentifier<'a>,
		user_2: &UserIdentifier<'a>,
		params: Option<&GetVersusInfoParams>,
	) -> Result<Self> {
		make_request_blocking(BASE_URL, [&user_1.to_string(), &user_2.to_string()], params)
	}
	#[cfg(feature = "matches")]
	/// Synchronounsly GET versus matches between two players using `params`
	pub fn get_matches_blocking<'a>(
		user_1: &UserIdentifier<'a>,
		user_2: &UserIdentifier<'a>,
		params: Option<&GetMatchesParams>,
	) -> Result<Vec<MatchInfo>> {
		make_request_blocking(
			MATCHES_URL,
			[&user_1.to_string(), &user_2.to_string()],
			params,
		)
	}
}
