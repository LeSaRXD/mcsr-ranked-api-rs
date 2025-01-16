use serde::Serialize;

#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{
	helpers::make_request,
	pagination::Pagination,
	types::{MatchId, Season},
	user::identifier::UserIdentifier,
};

use super::{AdvancedMatchInfo, MatchInfo, MatchType};

const BASE_URL: &str = "https://mcsrranked.com/api/matches/{}";

impl AdvancedMatchInfo {
	pub async fn get_by_id(id: MatchId) -> crate::Result<Self> {
		make_request(BASE_URL, [&id.to_string()], &None::<()>).await
	}
}

#[cfg(feature = "blocking")]
impl AdvancedMatchInfo {
	pub fn get_by_id_blocking(id: MatchId) -> crate::Result<Self> {
		make_request_blocking(BASE_URL, [&id.to_string()], &None::<()>)
	}
}

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

impl UserIdentifier<'_> {
	/// GET the user's matches by identifier using given `params`
	pub async fn get_user_matches(
		&self,
		params: &Option<GetUserMatchesParams>,
	) -> crate::Result<Box<[MatchInfo]>> {
		make_request(USER_URL, [&self.to_string()], params).await
	}
}

#[cfg(feature = "blocking")]
impl UserIdentifier<'_> {
	/// Synchronously GET the user's matches by identifier using given `params`
	pub fn get_user_matches_blocking(
		&self,
		params: &Option<GetUserMatchesParams>,
	) -> crate::Result<Box<[MatchInfo]>> {
		make_request_blocking(USER_URL, [&self.to_string()], params)
	}
}
