use std::{num::NonZero, ops::Not};

use serde::Serialize;

#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{helpers::make_request, types::Season, Result};

use super::BestTimeLeaderboard;

const BASE_URL: &str = "https://api.mcsrranked.com/record-leaderboard";

/// Season(s) parameter to fetch best times
///
/// - `All`: best times of all time
/// - `Current`: best times of the current season
/// - `Specific(season)`: best times of a specific `season`
///
/// Note: for better ergonomics, its recommended to construct
/// this enum using `From<Season>` or `From<Option<Season>>`
/// to avoid unwrapping `NonZero`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BestTimeSeason {
	All,
	Current,
	Specific(NonZero<Season>),
}
impl BestTimeSeason {
	fn is_all(&self) -> bool {
		matches!(self, BestTimeSeason::All)
	}
}
impl Serialize for BestTimeSeason {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		use BestTimeSeason as BTS;

		serde::Serialize::serialize(
			&match self {
				BTS::All => None,
				BTS::Current => Some(0),
				BTS::Specific(season) => Some(season.get()),
			},
			serializer,
		)
	}
}
impl From<Season> for BestTimeSeason {
	fn from(value: Season) -> Self {
		use BestTimeSeason as BTS;

		match NonZero::new(value) {
			None => BTS::Current,
			Some(v) => BTS::Specific(v),
		}
	}
}
impl From<Option<Season>> for BestTimeSeason {
	fn from(value: Option<Season>) -> Self {
		use BestTimeSeason as BTS;

		match value {
			None => BTS::All,
			Some(season) => Self::from(season),
		}
	}
}

/// Parameters for [`super::BestTimeLeaderboard::get`]
///
/// Note: this struct supports the builder pattern
///
/// # Examples
/// ```
/// use mcsr_ranked_api::leaderboard::game::requests::GetBestTimeLeaderboardParams;
/// let params = GetBestTimeLeaderboardParams::default()
///     .season(0)
///     .distinct(true);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetBestTimeLeaderboardParams {
	/// Which season to get best times from
	#[serde(skip_serializing_if = "BestTimeSeason::is_all")]
	pub season: BestTimeSeason,
	/// Only fetch distinct players' records
	#[serde(skip_serializing_if = "Not::not")]
	pub distinct: bool,
}
impl Default for GetBestTimeLeaderboardParams {
	fn default() -> Self {
		Self {
			season: BestTimeSeason::All,
			distinct: false,
		}
	}
}
impl GetBestTimeLeaderboardParams {
	/// Construct new params from `season` and `distinct`
	pub fn new(season: impl Into<BestTimeSeason>, distinct: bool) -> Self {
		Self {
			season: season.into(),
			distinct,
		}
	}
	/// Set the `season` field
	pub fn season(mut self, season: impl Into<BestTimeSeason>) -> Self {
		self.season = season.into();
		self
	}
	/// Set the `distinct` field
	pub fn distinct(mut self, distinct: bool) -> Self {
		self.distinct = distinct;
		self
	}
}

impl BestTimeLeaderboard {
	/// GET the best time leaderboard using given `params`
	pub async fn get(params: Option<&GetBestTimeLeaderboardParams>) -> Result<Self> {
		make_request(BASE_URL, &[] as &[&str], params).await
	}
}

#[cfg(feature = "blocking")]
impl BestTimeLeaderboard {
	/// Synchronously GET the best time leaderboard using given `params`
	pub fn get_blocking(params: Option<&GetBestTimeLeaderboardParams>) -> Result<Self> {
		make_request_blocking(BASE_URL, &[] as &[&str], params)
	}
}
