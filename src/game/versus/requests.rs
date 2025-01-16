use serde::Serialize;

#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{helpers::make_request, types::Season, user::identifier::UserIdentifier};

use super::VersusInfo;

const BASE_URL: &str = "https://mcsrranked.com/api/users/{}/versus/{}";

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
	/// GET versus matches between two players using `params`
	pub async fn get_by_ids<'a>(
		user_1: &UserIdentifier<'a>,
		user_2: &UserIdentifier<'a>,
		params: Option<&GetVersusInfoParams>,
	) -> crate::Result<Self> {
		make_request(BASE_URL, [&user_1.to_string(), &user_2.to_string()], params).await
	}
}

#[cfg(feature = "blocking")]
impl VersusInfo {
	/// Synchronously GET versus matches between two players using `params`
	pub fn get_by_ids_blocking<'a>(
		user_1: &UserIdentifier<'a>,
		user_2: &UserIdentifier<'a>,
		params: Option<&GetVersusInfoParams>,
	) -> crate::Result<Self> {
		make_request_blocking(BASE_URL, [&user_1.to_string(), &user_2.to_string()], params)
	}
}
