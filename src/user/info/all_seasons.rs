use std::collections::HashMap;

use serde::{de::Unexpected, Deserialize, Deserializer};

use crate::{
	types::Season,
	user::{info::UserSeasonOutcome, UserProfile},
};

/// The user info containing the results of all seasons
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllSeasonUserInfo {
	#[serde(flatten)]
	profile: UserProfile,
	#[serde(deserialize_with = "de_season_results")]
	season_results: Box<[Option<UserSeasonOutcome>]>,
}
impl AllSeasonUserInfo {
	/// The profile of the user
	pub fn profile(&self) -> &UserProfile {
		&self.profile
	}
	/// All of the seasons' results
	pub fn season_results(&self) -> &[Option<UserSeasonOutcome>] {
		&self.season_results
	}
	/// Specific season's results
	pub fn result(&self, season: Season) -> Option<&UserSeasonOutcome> {
		self.season_results
			.get(season as usize)
			.and_then(Option::as_ref)
	}
}

fn de_season_results<'de, D>(deserializer: D) -> Result<Box<[Option<UserSeasonOutcome>]>, D::Error>
where
	D: Deserializer<'de>,
{
	let map = HashMap::<String, UserSeasonOutcome>::deserialize(deserializer)?;

	let mut max_idx = 0usize;
	let idx_info_pairs = map
		.into_iter()
		.map(|(key, info)| match key.parse::<usize>() {
			Ok(idx) => {
				if idx > max_idx {
					max_idx = idx;
				}
				Ok((idx, info))
			}
			Err(_) => Err(serde::de::Error::invalid_type(
				Unexpected::Str(&key),
				&"index usize",
			)),
		})
		.collect::<Result<Vec<_>, _>>()?;

	let mut res = vec![None; max_idx + 1];
	for (idx, info) in idx_info_pairs {
		res[idx] = Some(info);
	}
	Ok(res.into_boxed_slice())
}
