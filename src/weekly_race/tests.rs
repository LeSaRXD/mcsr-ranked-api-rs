use chrono::{TimeZone, Utc};

use crate::{
	types::Time,
	user::{SupporterTier, UserProfile},
	weekly_race::{
		result::WeeklyRaceResult, WeeklyRaceLeaderboardInfo, WeeklyRaceLeaderboardRecord,
		WeeklyRaceSeedInfo,
	},
};

#[test]
fn weekly_race() {
	const JSON: &str = r#"{"id":13,"seed":{"overworld":"142605421743383832","nether":"142605421743383832","theEnd":"142605421743383832","rng":"142605421743383832"},"endsAt":1735516800,"leaderboard":[{"rank":1,"player":{"uuid":"4aed1e5e8f5c44e2bc0666e0c03781af","nickname":"nEmerald","roleType":0,"eloRate":1512,"eloRank":92,"country":null},"time":324323,"replayExist":true},{"rank":2,"player":{"uuid":"92b63a39b36a445fa94c77ae212dcea3","nickname":"bing_pigs","roleType":0,"eloRate":1512,"eloRank":92,"country":null},"time":345664,"replayExist":true},{"rank":3,"player":{"uuid":"5cd115f0ec1240659db152406c0984a3","nickname":"yjako","roleType":0,"eloRate":1512,"eloRank":92,"country":null},"time":354563,"replayExist":false}]}"#;
	let race: WeeklyRaceLeaderboardInfo = serde_json::from_str(JSON).unwrap();
	assert_eq!(race.id, 13);
	assert_eq!(
		race.seed,
		WeeklyRaceSeedInfo {
			overworld: 142605421743383832,
			nether: 142605421743383832,
			the_end: 142605421743383832,
			rng: 142605421743383832,
		}
	);
	assert_eq!(race.ends_at, Utc.timestamp_opt(1735516800, 0).unwrap());
	assert_eq!(
		race.leaderboard(),
		[
			WeeklyRaceLeaderboardRecord {
				rank: 1,
				player: UserProfile::new(
					"4aed1e5e8f5c44e2bc0666e0c03781af",
					"nEmerald",
					SupporterTier::None,
					Some(1512),
					Some(92),
					None,
				),
				time: Time(324323),
				replay_exists: true,
			},
			WeeklyRaceLeaderboardRecord {
				rank: 2,
				player: UserProfile::new(
					"92b63a39b36a445fa94c77ae212dcea3",
					"bing_pigs",
					SupporterTier::None,
					Some(1512),
					Some(92),
					None,
				),
				time: Time(345664),
				replay_exists: true,
			},
			WeeklyRaceLeaderboardRecord {
				rank: 3,
				player: UserProfile::new(
					"5cd115f0ec1240659db152406c0984a3",
					"yjako",
					SupporterTier::None,
					Some(1512),
					Some(92),
					None,
				),
				time: Time(354563),
				replay_exists: false,
			},
		]
	);

	#[cfg(feature = "serialize")]
	{
		let re_deserialized: WeeklyRaceLeaderboardInfo =
			serde_json::from_str(&serde_json::to_string(&race).unwrap()).unwrap();
		assert_eq!(re_deserialized, race);
	}
}

#[test]
fn weekly_race_result() {
	const JSON: &str = r#"{"id":1,"time":489237,"rank":8}"#;
	let result: WeeklyRaceResult = serde_json::from_str(JSON).unwrap();
	assert_eq!(result.id, 1);
	assert_eq!(result.time, Time(489237));
	assert_eq!(result.rank, 8);

	#[cfg(feature = "serialize")]
	{
		let re_deserialized: WeeklyRaceResult =
			serde_json::from_str(&serde_json::to_string(&result).unwrap()).unwrap();
		assert_eq!(re_deserialized, result);
	}
}
