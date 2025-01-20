use serde::Serialize;
use std::ops::Not;

#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{
	helpers::make_request,
	pagination::Pagination,
	types::{MatchId, Season},
	user::identifier::UserIdentifier,
	Result,
};

use super::{AdvancedMatchInfo, MatchInfo, MatchType};

const BASE_URL: &str = "https://mcsrranked.com/api/matches/{}";

impl AdvancedMatchInfo {
	pub async fn get_by_id(id: MatchId) -> Result<Self> {
		make_request(BASE_URL, [&id.to_string()], None::<&()>).await
	}
}

#[cfg(feature = "blocking")]
impl AdvancedMatchInfo {
	pub fn get_by_id_blocking(id: MatchId) -> Result<Self> {
		make_request_blocking(BASE_URL, [&id.to_string()], None::<&()>)
	}
}

const USER_URL: &str = "https://mcsrranked.com/api/users/{}/matches";

/// Parameters for [`UserIdentifier::get_user_matches`]
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
		params: Option<&GetUserMatchesParams>,
	) -> Result<Box<[MatchInfo]>> {
		make_request(USER_URL, [&self.to_string()], params).await
	}
}

#[cfg(feature = "blocking")]
impl UserIdentifier<'_> {
	/// Synchronously GET the user's matches by identifier using given `params`
	pub fn get_user_matches_blocking(
		&self,
		params: Option<&GetUserMatchesParams>,
	) -> Result<Box<[MatchInfo]>> {
		make_request_blocking(USER_URL, [&self.to_string()], params)
	}
}

const RECENT_URL: &str = "https://mcsrranked.com/api/matches/";

/// Parameters for [`MatchInfo::get_recent`]
///
/// /// Note: this struct supports the builder pattern
///
/// # Examples
/// ```
/// use mcsr_ranked_api::game::requests::GetRecentMatchesParams;
/// use mcsr_ranked_api::game::MatchType;
/// let params = GetRecentMatchesParams::default()
///     .season(0)
///     .include_decay(true)
///     .kind(MatchType::Private);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetRecentMatchesParams<'a> {
	#[serde(flatten)]
	pub pagination: Option<Pagination>,
	#[serde(rename = "type")]
	pub kind: Option<MatchType>,
	pub tag: Option<&'a str>,
	pub season: Option<Season>,
	#[serde(rename = "includedecay")]
	#[serde(skip_serializing_if = "Not::not")]
	pub include_decay: bool,
}
impl From<Pagination> for GetRecentMatchesParams<'_> {
	fn from(value: Pagination) -> Self {
		Self {
			pagination: Some(value),
			..Default::default()
		}
	}
}
impl<'a> GetRecentMatchesParams<'a> {
	/// Set the `pagination` field
	///
	/// Note: it is better to use `From<Pagination>`
	pub fn pagination(mut self, pagination: Pagination) -> Self {
		self.pagination = Some(pagination);
		self
	}
	/// Set the `kind` field
	pub fn kind(mut self, kind: MatchType) -> Self {
		self.kind = Some(kind);
		self
	}
	/// Set the `tag` field
	pub fn tag(mut self, tag: &'a str) -> Self {
		self.tag = Some(tag);
		self
	}
	/// Set the `season` field
	pub fn season(mut self, season: Season) -> Self {
		self.season = Some(season);
		self
	}
	/// Set the `include_decay` field
	pub fn include_decay(mut self, include_decay: bool) -> Self {
		self.include_decay = include_decay;
		self
	}
}

impl MatchInfo {
	/// GET recent matches given `params`
	pub async fn get_recent(params: Option<&GetRecentMatchesParams<'_>>) -> Result<Box<[Self]>> {
		make_request(RECENT_URL, &[] as &[&str], params).await
	}
}

#[cfg(feature = "blocking")]
impl MatchInfo {
	/// Synchronously GET recent matches given `params`
	pub fn get_recent_blocking(params: Option<&GetRecentMatchesParams<'_>>) -> Result<Box<[Self]>> {
		make_request_blocking(RECENT_URL, &[] as &[&str], params)
	}
}
