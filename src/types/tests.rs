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

	use crate::{types::DeReqResult, user::SupporterTier};

	fn result_from<T>(json: &str) -> DeReqResult<T>
	where
		for<'de> T: Deserialize<'de>,
	{
		serde_json::from_str(json).unwrap()
	}

	#[test]
	fn empty_error() {
		const JSON: &str = r#"{"status":"error","data":null}"#;
		let result = result_from::<Option<()>>(JSON);
		assert_eq!(result, DeReqResult::Error(None));
	}

	#[test]
	fn message_error() {
		const JSON: &str =
			r#"{"status":"error","data":"invalid `type` query. it must be >= 0 or <= 3"}"#;
		let result = result_from::<Option<()>>(JSON);
		assert_eq!(
			result,
			DeReqResult::Error(Some("invalid `type` query. it must be >= 0 or <= 3".into()))
		);
	}

	#[test]
	fn empty_ok() {
		const JSON: &str = r#"{"status": "success", "data": null}"#;
		let result = result_from::<Option<()>>(JSON);
		assert_eq!(result, DeReqResult::Success(None));
	}

	#[test]
	fn user_ok() {
		use crate::user::UserProfile;

		const JSON: &str = r#"{"status":"success","data":{"uuid":"7665f76f431b41c6b321bea16aff913b","nickname":"lowk3y_","roleType":0,"eloRate":1966,"eloRank":4}}"#;
		let result = result_from::<UserProfile>(JSON);
		assert_eq!(
			result,
			DeReqResult::Success(UserProfile::new(
				Uuid::from_str("7665f76f431b41c6b321bea16aff913b").unwrap(),
				"lowk3y_",
				SupporterTier::None,
				Some(1966),
				Some(4)
			))
		)
	}
}
