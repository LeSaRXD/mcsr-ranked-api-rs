use serde::Serialize;

use crate::{
	helpers::{make_request, make_request_blocking},
	pagination::Pagination,
	types::Season,
	user::identifier::UserIdentifier,
};

use super::{MatchInfo, MatchType};

const USER_URL: &str = "https://mcsrranked.com/api/users/{}/matches";

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetUserMatchesParams {
	#[serde(flatten)]
	pub pagination: Pagination,
	#[serde(rename = "type")]
	pub kind: Option<MatchType>,
	pub season: Option<Season>,
	pub exclude_decay: bool,
}
impl From<Pagination> for GetUserMatchesParams {
	fn from(pagination: Pagination) -> Self {
		Self {
			pagination,
			..Default::default()
		}
	}
}

impl<'a> UserIdentifier<'a> {
	/// GET the user's matches by identifier using given `params`
	pub async fn get_user_matches(
		&self,
		params: &Option<GetUserMatchesParams>,
	) -> crate::Result<Box<[MatchInfo]>> {
		make_request(USER_URL, [&self.to_string()], params).await
	}
}

#[cfg(feature = "blocking")]
impl<'a> UserIdentifier<'a> {
	/// Synchronously GET the user's matches by identifier using given `params`
	pub fn get_user_matches_blocking(
		&self,
		params: &Option<GetUserMatchesParams>,
	) -> crate::Result<Box<[MatchInfo]>> {
		make_request_blocking(USER_URL, [&self.to_string()], params)
	}
}
