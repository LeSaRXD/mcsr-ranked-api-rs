use serde::Serialize;

#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{Result, helpers::make_request, types::Season};

use super::{
	identifier::UserIdentifier,
	info::{UserInfo, all_seasons::AllSeasonUserInfo},
};

const BASE_URL: &str = "https://api.mcsrranked.com/users/{}";
const ALL_SEASONS_URL: &str = "https://api.mcsrranked.com/users/{}/seasons";

/// Parameters for [`UserIdentifier::get_user`]
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

impl<'a> UserIdentifier<'a> {
	/// GET the user by identifier using given `params`
	pub async fn get_user(&self, params: impl Into<Option<&'a GetUserParams>>) -> Result<UserInfo> {
		make_request(BASE_URL, [&self.to_string()], params.into()).await
	}
	/// GET the user's info with data from all seasons
	pub async fn get_user_all_seasons(&self) -> Result<AllSeasonUserInfo> {
		make_request(ALL_SEASONS_URL, &[&self.to_string()], None::<&()>).await
	}
}

#[cfg(feature = "blocking")]
impl<'a> UserIdentifier<'a> {
	/// Synchronously GET the user by identifier using given `params`
	pub fn get_user_blocking(
		&self,
		params: impl Into<Option<&'a GetUserParams>>,
	) -> Result<UserInfo> {
		make_request_blocking(BASE_URL, [&self.to_string()], params.into())
	}
	/// Synchronously GET the user's info with data from all seasons
	pub fn get_user_all_seasons_blocking(&self) -> Result<AllSeasonUserInfo> {
		make_request_blocking(ALL_SEASONS_URL, &[&self.to_string()], None::<&()>)
	}
}
