use chrono::{TimeZone, Utc};
use serde::de::{Error, Unexpected};

use super::{Achievement, AchievementData};

fn achievment_from(json: &str) -> Achievement {
	serde_json::from_str(json).unwrap()
}
fn achievment_err_from(json: &str) -> serde_json::Error {
	serde_json::from_str::<Achievement>(json).unwrap_err()
}

#[test]
fn best_time() {
	const JSON: &str =
		r#"{"id":"bestTime","date":1706839603,"data":[],"level":10,"goal":419999,"value":null}"#;
	assert_eq!(
		achievment_from(JSON),
		Achievement {
			date: Utc.timestamp_opt(1706839603, 0).unwrap(),
			data: AchievementData::BestTime,
			level: 10,
			goal: Some(419999),
			value: None,
		}
	);
}

#[test]
fn highest_win_streak() {
	const JSON: &str =
		r#"{"id":"highestWinStreak","date":1706839603,"data":[],"level":7,"goal":25,"value":null}"#;
	assert_eq!(
		achievment_from(JSON),
		Achievement {
			date: Utc.timestamp_opt(1706839603, 0).unwrap(),
			data: AchievementData::HighestWinStreak,
			level: 7,
			goal: Some(25),
			value: None,
		}
	);
}

#[test]
fn played_matches() {
	const JSON: &str =
		r#"{"id":"playedMatches","date":1706839603,"data":[],"level":10,"goal":5000,"value":null}"#;
	assert_eq!(
		achievment_from(JSON),
		Achievement {
			date: Utc.timestamp_opt(1706839603, 0).unwrap(),
			data: AchievementData::PlayedMatches,
			level: 10,
			goal: Some(5000),
			value: None,
		}
	);
}

#[test]
fn playtime() {
	const JSON: &str =
		r#"{"id":"playtime","date":1706839603,"data":[],"level":7,"goal":1800000000,"value":null}"#;
	assert_eq!(
		achievment_from(JSON),
		Achievement {
			date: Utc.timestamp_opt(1706839603, 0).unwrap(),
			data: AchievementData::Playtime,
			level: 7,
			goal: Some(1800000000),
			value: None,
		}
	);
}

#[test]
fn wins() {
	const JSON: &str =
		r#"{"id":"wins","date":1706839603,"data":[],"level":9,"goal":2000,"value":null}"#;
	assert_eq!(
		achievment_from(JSON),
		Achievement {
			date: Utc.timestamp_opt(1706839603, 0).unwrap(),
			data: AchievementData::Wins,
			level: 9,
			goal: Some(2000),
			value: None,
		}
	);
}

#[test]
fn season_result_ok() {
	const JSON: &str =
		r#"{"id":"seasonResult","date":1724198414,"data":["5","2"],"level":2,"value":null}"#;
	assert_eq!(
		achievment_from(JSON),
		Achievement {
			date: Utc.timestamp_opt(1724198414, 0).unwrap(),
			data: AchievementData::SeasonOutcome { season: 5, rank: 2 },
			level: 2,
			goal: None,
			value: None,
		}
	);
}

#[test]
fn season_result_err_length() {
	const JSON_ERR_0: &str =
		r#"{"id":"seasonResult","date":1724198414,"data":[],"level":2,"value":null}"#;
	let custom_serde_json = serde_json::Error::invalid_length(0, &"2");
	assert_eq!(
		achievment_err_from(JSON_ERR_0).to_string(),
		custom_serde_json.to_string(),
	);

	const JSON_ERR_1: &str =
		r#"{"id":"seasonResult","date":1724198414,"data":["5"],"level":2,"value":null}"#;
	let custom_serde_json = serde_json::Error::invalid_length(1, &"2");
	assert_eq!(
		achievment_err_from(JSON_ERR_1).to_string(),
		custom_serde_json.to_string(),
	);

	const JSON_ERR_2: &str =
		r#"{"id":"seasonResult","date":1724198414,"data":["5","2","???"],"level":2,"value":null}"#;
	let custom_serde_json = serde_json::Error::invalid_length(3, &"2");
	assert_eq!(
		achievment_err_from(JSON_ERR_2).to_string(),
		custom_serde_json.to_string(),
	);
}

#[test]
fn season_result_err_type() {
	const JSON_ERR_SEASON: &str =
		r#"{"id":"seasonResult","date":1724198414,"data":["season","2"],"level":2,"value":null}"#;
	let custom_serde_json =
		serde_json::Error::invalid_type(Unexpected::Str("season"), &"season number");
	assert_eq!(
		achievment_err_from(JSON_ERR_SEASON).to_string(),
		custom_serde_json.to_string(),
	);

	const JSON_ERR_RANK: &str =
		r#"{"id":"seasonResult","date":1724198414,"data":["1","rank"],"level":2,"value":null}"#;
	let custom_serde_json =
		serde_json::Error::invalid_type(Unexpected::Str("rank"), &"rank number");
	assert_eq!(
		achievment_err_from(JSON_ERR_RANK).to_string(),
		custom_serde_json.to_string(),
	);
}
