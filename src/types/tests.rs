use crate::types::RankedAndCasual;

mod time {
	use crate::types::Time;

	#[test]
	fn time() {
		let time = Time::new(3_923_124);
		assert_eq!(time.millis(), 124);
		assert_eq!(time.seconds(), 23);
		assert_eq!(time.minutes(), 5);
		assert_eq!(time.hours(), 1);
	}
}

mod api_result {
	use std::str::FromStr;

	use serde::Deserialize;
	use uuid::Uuid;

	use crate::{types::DeResult, user::SupporterTier};

	fn result_from<T>(json: &str) -> DeResult<T>
	where
		for<'de> T: Deserialize<'de>,
	{
		serde_json::from_str(json).unwrap()
	}

	#[test]
	fn empty_error() {
		const JSON: &str = r#"{"status":"error","data":null}"#;
		let result = result_from::<Option<()>>(JSON);
		assert_eq!(result, DeResult::Error(None));
	}

	#[test]
	fn message_error() {
		const JSON: &str =
			r#"{"status":"error","data":"invalid `type` query. it must be >= 0 or <= 3"}"#;
		let result = result_from::<Option<()>>(JSON);
		assert_eq!(
			result,
			DeResult::Error(Some("invalid `type` query. it must be >= 0 or <= 3".into()))
		);
	}

	#[test]
	fn empty_ok() {
		const JSON: &str = r#"{"status": "success", "data": null}"#;
		let result = result_from::<Option<()>>(JSON);
		assert_eq!(result, DeResult::Success(None));
	}

	#[test]
	fn user_ok() {
		use crate::user::UserProfile;

		const JSON: &str = r#"{"status":"success","data":{"uuid":"7665f76f431b41c6b321bea16aff913b","nickname":"lowk3y_","roleType":0,"eloRate":1966,"eloRank":4}}"#;
		let result = result_from::<UserProfile>(JSON);
		assert_eq!(
			result,
			DeResult::Success(UserProfile::new(
				Uuid::from_str("7665f76f431b41c6b321bea16aff913b").unwrap(),
				"lowk3y_",
				SupporterTier::None,
				Some(1966),
				Some(4)
			))
		)
	}
}

mod two_user_data {
	use std::str::FromStr;

	use serde::Deserialize;
	use uuid::Uuid;

	use crate::types::TwoUserData;

	#[test]
	fn simple() {
		const JSON: &str =
			r#"{"a0c06d33c69941d09b22e0c98c4233fd":2,"af22aaab9ee74596a3578bd6345d25b5":1}"#;
		let data: TwoUserData<u32> = serde_json::from_str(JSON).unwrap();

		assert_eq!(
			data.user_1(),
			(
				Uuid::from_str("a0c06d33c69941d09b22e0c98c4233fd").unwrap(),
				&2
			),
		);
		assert_eq!(
			data.user_2(),
			(
				Uuid::from_str("af22aaab9ee74596a3578bd6345d25b5").unwrap(),
				&1
			),
		);
	}

	#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
	struct TestFlatten {
		#[serde(flatten)]
		two_user: TwoUserData<u32>,
		total: u32,
	}

	#[test]
	fn with_flatten() {
		const JSON: &str = r#"{"a0c06d33c69941d09b22e0c98c4233fd":2,"af22aaab9ee74596a3578bd6345d25b5":1,"total":3}"#;
		let data: TestFlatten = serde_json::from_str(JSON).unwrap();
		assert_eq!(
			data.two_user.user_1(),
			(
				Uuid::from_str("a0c06d33c69941d09b22e0c98c4233fd").unwrap(),
				&2
			)
		);
		assert_eq!(
			data.two_user.user_2(),
			(
				Uuid::from_str("af22aaab9ee74596a3578bd6345d25b5").unwrap(),
				&1
			)
		);
		assert_eq!(data.total, 3);
	}
}

#[test]
fn ranked_and_casual() {
	const JSON: &str = r#"{"ranked":2,"casual":1}"#;
	let data: RankedAndCasual<u32> = serde_json::from_str(JSON).unwrap();

	assert_eq!(data.ranked(), &2);
	assert_eq!(data.casual(), &1);
}
