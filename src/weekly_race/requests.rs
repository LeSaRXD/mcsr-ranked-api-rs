#[cfg(feature = "blocking")]
use crate::helpers::make_request_blocking;
use crate::{helpers::make_request, types::WeeklyRaceId};

use super::WeeklyRaceLeaderboardInfo;

const BASE_URL: &str = "https://mcsrranked.com/api/weekly-race/{}";

impl WeeklyRaceLeaderboardInfo {
	/// GET the weekly race leaderboard using given weekly `race_id`
	pub async fn get_by_id(race_id: WeeklyRaceId) -> crate::Result<Self> {
		make_request(BASE_URL, [&race_id.to_string()], None::<&()>).await
	}

	/// GET the current weekly race leaderboard
	pub async fn get_current() -> crate::Result<Self> {
		make_request(BASE_URL, &[] as &[&str], None::<&()>).await
	}
}

#[cfg(feature = "blocking")]
impl WeeklyRaceLeaderboardInfo {
	/// Synchronously GET the weekly race leaderboard using given weekly `race_id`
	pub fn get_by_id_blocking(race_id: WeeklyRaceId) -> crate::Result<Self> {
		make_request_blocking(BASE_URL, [&race_id.to_string()], None::<&()>)
	}

	/// Synchronously GET the current weekly race leaderboard
	pub fn get_current_blocking() -> crate::Result<Self> {
		make_request_blocking(BASE_URL, &[] as &[&str], None::<&()>)
	}
}
