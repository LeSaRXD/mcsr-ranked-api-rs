use serde::Serialize;

#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{helpers::make_request, types::Season, Result};

use super::{identifier::UserIdentifier, info::UserInfo};

const BASE_URL: &str = "https://mcsrranked.com/api/users/{}";

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

impl UserIdentifier<'_> {
	/// GET the user by identifier using given `params`
	pub async fn get_user(&self, params: GetUserParams) -> Result<UserInfo> {
		make_request(BASE_URL, [&self.to_string()], Some(&params)).await
	}
}

#[cfg(feature = "blocking")]
impl UserIdentifier<'_> {
	/// Synchronously GET the user by identifier using given `params`
	pub fn get_user_blocking(&self, params: GetUserParams) -> Result<UserInfo> {
		make_request_blocking(BASE_URL, [&self.to_string()], Some(&params))
	}
}
