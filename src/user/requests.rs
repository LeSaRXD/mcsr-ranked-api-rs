use serde::Serialize;

use crate::{
	helpers::construct_url,
	types::{DeReqResult, Season},
};

use super::{identifier::UserIdentifier, info::UserInfo};

const BASE_URL: &str = "https://mcsrranked.com/api/users/{}";

/// Parameters for [UserIdentifier::get_user]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetUserParams {
	/// Season to get the user info for
	///
	/// Default is the current season
	pub season: Option<Season>,
}
impl GetUserParams {
	pub fn new(season: Season) -> Self {
		Self {
			season: Some(season),
		}
	}
}

impl UserIdentifier<'_> {
	/// GET the user by identifier using given `params`
	pub async fn get_user(&self, params: &Option<GetUserParams>) -> crate::Result<UserInfo> {
		let url = construct_url(BASE_URL, [&self.to_string()], params);
		reqwest::get(url.as_ref())
			.await?
			.json::<DeReqResult<UserInfo>>()
			.await?
			.into()
	}
}

#[cfg(feature = "blocking")]
impl UserIdentifier<'_> {
	/// Synchronously GET the user by identifier using given `params`
	pub fn get_user_blocking(&self, params: &Option<GetUserParams>) -> crate::Result<UserInfo> {
		let url = construct_url(BASE_URL, [&self.to_string()], params);
		reqwest::blocking::get(url.as_ref())?
			.json::<DeReqResult<UserInfo>>()?
			.into()
	}
}
