use super::VersusInfo;
use crate::user::{SupporterTier, UserProfile};

use uuid::uuid;

#[test]
fn versus() {
	const JSON: &str = r#"{"players":[{"uuid":"a0c06d33c69941d09b22e0c98c4233fd","nickname":"jamyreaf","roleType":0,"eloRate":1637,"eloRank":44,"country":null},{"uuid":"af22aaab9ee74596a3578bd6345d25b5","nickname":"Priffin","roleType":0,"eloRate":1637,"eloRank":44,"country":null}],"results":{"ranked":{"total":3,"a0c06d33c69941d09b22e0c98c4233fd":2,"af22aaab9ee74596a3578bd6345d25b5":1},"casual":{"total":0,"a0c06d33c69941d09b22e0c98c4233fd":0,"af22aaab9ee74596a3578bd6345d25b5":0}},"changes":{"a0c06d33c69941d09b22e0c98c4233fd":33,"af22aaab9ee74596a3578bd6345d25b5":-33}}"#;
	let versus: VersusInfo = serde_json::from_str(JSON).unwrap();
	assert_eq!(
		versus.players.0,
		UserProfile::new(
			"a0c06d33c69941d09b22e0c98c4233fd",
			"jamyreaf",
			SupporterTier::None,
			Some(1637),
			Some(44),
			None,
		)
	);
	assert_eq!(
		versus.players.1,
		UserProfile::new(
			"af22aaab9ee74596a3578bd6345d25b5",
			"Priffin",
			SupporterTier::None,
			Some(1637),
			Some(44),
			None,
		)
	);

	assert_eq!(versus.results.ranked.total, 3);
	assert_eq!(
		versus.results.ranked.wins.user_1(),
		(uuid!("a0c06d33c69941d09b22e0c98c4233fd"), &2),
	);
	assert_eq!(
		versus.results.ranked.wins.user_2(),
		(uuid!("af22aaab9ee74596a3578bd6345d25b5"), &1),
	);

	assert_eq!(versus.results.casual.total, 0);
	assert_eq!(
		versus.results.casual.wins.user_1(),
		(uuid!("a0c06d33c69941d09b22e0c98c4233fd"), &0),
	);
	assert_eq!(
		versus.results.casual.wins.user_2(),
		(uuid!("af22aaab9ee74596a3578bd6345d25b5"), &0),
	);

	assert_eq!(
		versus.changes.user_1(),
		(uuid!("a0c06d33c69941d09b22e0c98c4233fd"), &33),
	);
	assert_eq!(
		versus.changes.user_2(),
		(uuid!("af22aaab9ee74596a3578bd6345d25b5"), &-33),
	);

	#[cfg(feature = "serialize")]
	{
		println!("{}", serde_json::to_string(&versus).unwrap());
		let re_deserialized: VersusInfo =
			serde_json::from_str(&serde_json::to_string(&versus).unwrap()).unwrap();
		assert_eq!(
			re_deserialized, versus,
			"Re-deserialized info does not match initial info"
		);
	}
}
