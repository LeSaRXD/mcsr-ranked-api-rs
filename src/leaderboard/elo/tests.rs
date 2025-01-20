use std::str::FromStr;

use chrono::{TimeZone, Utc};
use uuid::Uuid;

use super::{EloLeaderboardInfo, SeasonInfo};
use crate::{
	leaderboard::LeaderboardSeasonOutcome,
	user::{SupporterTier, UserProfile},
};

#[test]
fn leaderboard() {
	const JSON: &str = r#"{"season":{"endsAt":1712448000,"number":4},"users":[{"uuid":"3c8757790ab0400b8b9e3936e0dd535b","nickname":"doogile","roleType":3,"eloRate":2175,"eloRank":1,"seasonResult":{"eloRate":2175,"eloRank":1,"phasePoint":40}},{"uuid":"17e787d1d6374f818b294f2319db370d","nickname":"silverrruns","roleType":0,"eloRate":2002,"eloRank":2,"seasonResult":{"eloRate":2002,"eloRank":2,"phasePoint":25}},{"uuid":"70eb9286e3e24153a8b37c8f884f1292","nickname":"7rowl","roleType":0,"eloRate":1969,"eloRank":3,"seasonResult":{"eloRate":1969,"eloRank":3,"phasePoint":35}},{"uuid":"7665f76f431b41c6b321bea16aff913b","nickname":"lowk3y_","roleType":0,"eloRate":1966,"eloRank":4,"seasonResult":{"eloRate":1966,"eloRank":4,"phasePoint":50}},{"uuid":"af22aaab9ee74596a3578bd6345d25b5","nickname":"Priffin","roleType":0,"eloRate":1955,"eloRank":5,"seasonResult":{"eloRate":1955,"eloRank":5,"phasePoint":25}},{"uuid":"a29a2e3d1ed649f8b122de8ddad2668a","nickname":"Jud0zwerg","roleType":0,"eloRate":1446,"eloRank":147,"seasonResult":{"eloRate":1446,"eloRank":147,"phasePoint":0}},{"uuid":"0388b80ebe6c4216b4a8305c0cd27894","nickname":"tommorerow","roleType":1,"eloRate":1445,"eloRank":148,"seasonResult":{"eloRate":1445,"eloRank":148,"phasePoint":5}},{"uuid":"8021b1eb133346c3b0b88d19c5be9188","nickname":"gabboooz","roleType":0,"eloRate":1443,"eloRank":149,"seasonResult":{"eloRate":1443,"eloRank":149,"phasePoint":0}},{"uuid":"aa0aee82f7a94591a076331d899f836c","nickname":"sacanagem_online","roleType":0,"eloRate":1439,"eloRank":150,"seasonResult":{"eloRate":1439,"eloRank":150,"phasePoint":5}},{"uuid":"c7802cb7c30c47aabc1a7ec790ff2260","nickname":"iKme_","roleType":0,"eloRate":1439,"eloRank":150,"seasonResult":{"eloRate":1439,"eloRank":150,"phasePoint":0}}]}"#;
	let leaderboard: EloLeaderboardInfo = serde_json::from_str(JSON).unwrap();

	assert_eq!(
		leaderboard.season,
		SeasonInfo {
			number: 4,
			ends_at: Utc.timestamp_opt(1712448000, 0).unwrap(),
		}
	);
	assert_eq!(
		leaderboard.users[0].profile,
		UserProfile::new(
			Uuid::from_str("3c8757790ab0400b8b9e3936e0dd535b").unwrap(),
			"doogile",
			SupporterTier::Diamond,
			Some(2175),
			Some(1)
		),
	);
	assert_eq!(
		leaderboard.users[0].season_result,
		LeaderboardSeasonOutcome {
			elo: 2175,
			rank: 1,
			phase_point: 40,
		}
	)
}
