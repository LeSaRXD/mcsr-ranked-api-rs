use std::str::FromStr;

use chrono::{TimeZone, Utc};
use uuid::Uuid;

use crate::{
	leaderboard::game::{BestTimeInfo, BestTimeLeaderboard},
	types::Time,
	user::{SupporterTier, UserProfile},
};

#[test]
fn leaderboard() {
	const JSON: &str = r#"[{"rank":1,"season":1,"date":1685157577,"id":284288,"time":433388,"user":{"uuid":"08476f5847fc4daeba74a2544fc9d65b","nickname":"Zylenox","roleType":0,"eloRate":1523,"eloRank":90},"country":"us"},{"rank":2,"season":1,"date":1685696875,"id":300983,"time":457763,"user":{"uuid":"17e787d1d6374f818b294f2319db370d","nickname":"silverrruns","roleType":0,"eloRate":1818,"eloRank":15},"country":null}]"#;
	let leaderboard: BestTimeLeaderboard = serde_json::from_str(JSON).unwrap();

	assert_eq!(
		leaderboard[0],
		BestTimeInfo {
			rank: 1,
			season: 1,
			date: Utc.timestamp_opt(1685157577, 0).unwrap(),
			id: 284288,
			time: Time(433388),
			user: UserProfile::new(
				Uuid::from_str("08476f5847fc4daeba74a2544fc9d65b").unwrap(),
				"Zylenox",
				SupporterTier::None,
				Some(1523),
				Some(90),
				None,
			),
		}
	);
	assert_eq!(
		leaderboard[1],
		BestTimeInfo {
			rank: 2,
			season: 1,
			date: Utc.timestamp_opt(1685696875, 0).unwrap(),
			id: 300983,
			time: Time(457763),
			user: UserProfile::new(
				Uuid::from_str("17e787d1d6374f818b294f2319db370d").unwrap(),
				"silverrruns",
				SupporterTier::None,
				Some(1818),
				Some(15),
				None,
			),
		}
	);
}

mod params {
	use crate::leaderboard::game::requests::{BestTimeSeason, GetBestTimeLeaderboardParams};

	#[test]
	fn from() {
		assert_eq!(BestTimeSeason::from(0), BestTimeSeason::Current);

		assert_eq!(
			BestTimeSeason::from(1),
			BestTimeSeason::Specific(1.try_into().unwrap()),
		);

		assert_eq!(
			BestTimeSeason::from(255),
			BestTimeSeason::Specific(255.try_into().unwrap()),
		);

		assert_eq!(BestTimeSeason::from(None), BestTimeSeason::All);
	}

	#[test]
	fn empty() {
		let params = GetBestTimeLeaderboardParams {
			season: BestTimeSeason::All,
			distinct: false,
		};
		let params = serde_qs::to_string(&params).unwrap();
		assert_eq!(params, "");
	}

	#[test]
	fn only_season() {
		let params = GetBestTimeLeaderboardParams {
			season: BestTimeSeason::Current,
			distinct: false,
		};
		let params = serde_qs::to_string(&params).unwrap();
		assert_eq!(params, "season=0");
	}

	#[test]
	fn only_distinct() {
		let params = GetBestTimeLeaderboardParams {
			season: BestTimeSeason::All,
			distinct: true,
		};
		let params = serde_qs::to_string(&params).unwrap();
		assert_eq!(params, "distinct=true");
	}

	#[test]
	fn full() {
		let params = GetBestTimeLeaderboardParams {
			season: BestTimeSeason::Specific(2.try_into().unwrap()),
			distinct: true,
		};
		let params = serde_qs::to_string(&params).unwrap();
		assert_eq!(params, "season=2&distinct=true");
	}
}
