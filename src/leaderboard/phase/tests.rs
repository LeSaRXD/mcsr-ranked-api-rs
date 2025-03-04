use chrono::{TimeZone, Utc};

use crate::{
	leaderboard::{
		phase::{PhaseInfo, PhaseLeaderboardInfo},
		LeaderboardSeasonOutcome, LeaderboardUser,
	},
	user::{SupporterTier, UserProfile},
};

#[test]
fn leaderboard() {
	const JSON: &str = r#"{"phase":{"endsAt":1709769600,"number":2,"season":4},"users":[{"uuid":"7665f76f431b41c6b321bea16aff913b","nickname":"lowk3y_","roleType":0,"eloRate":1966,"eloRank":4,"seasonResult":{"eloRate":1966,"eloRank":4,"phasePoint":50},"country":null},{"uuid":"3c8757790ab0400b8b9e3936e0dd535b","nickname":"doogile","roleType":3,"eloRate":2175,"eloRank":1,"seasonResult":{"eloRate":2175,"eloRank":1,"phasePoint":40},"country":"us"},{"uuid":"70eb9286e3e24153a8b37c8f884f1292","nickname":"7rowl","roleType":0,"eloRate":1969,"eloRank":3,"seasonResult":{"eloRate":1969,"eloRank":3,"phasePoint":35},"country":null},{"uuid":"562a308be86c4ec09438387860e792cc","nickname":"Oxidiot","roleType":0,"eloRate":1942,"eloRank":8,"country":null,"seasonResult":{"eloRate":1942,"eloRank":8,"phasePoint":30}},{"uuid":"17e787d1d6374f818b294f2319db370d","nickname":"silverrruns","roleType":0,"eloRate":2002,"eloRank":2,"country":null,"seasonResult":{"eloRate":2002,"eloRank":2,"phasePoint":25}},{"uuid":"af22aaab9ee74596a3578bd6345d25b5","nickname":"Priffin","roleType":0,"eloRate":1955,"eloRank":5,"country":null,"seasonResult":{"eloRate":1955,"eloRank":5,"phasePoint":25}},{"uuid":"fa61606e8131484c8dee506d1ff9a8dc","nickname":"AutomattPL","roleType":3,"eloRate":1947,"eloRank":6,"country":null,"seasonResult":{"eloRate":1947,"eloRank":6,"phasePoint":25}},{"uuid":"aa0aee82f7a94591a076331d899f836c","nickname":"sacanagem_online","roleType":0,"eloRate":1439,"eloRank":150,"country":null,"seasonResult":{"eloRate":1439,"eloRank":150,"phasePoint":5}},{"uuid":"5a2cb29136eb46529adc03aa4583a2d2","nickname":"GradientGray","roleType":0,"eloRate":1412,"eloRank":180,"country":null,"seasonResult":{"eloRate":1412,"eloRank":180,"phasePoint":5}},{"uuid":"745a819973974fe1bb1608e57fd439b6","nickname":"centuriee","roleType":0,"eloRate":1412,"eloRank":180,"country":null,"seasonResult":{"eloRate":1412,"eloRank":180,"phasePoint":5}},{"uuid":"4c3bc64c9f0a4cd988cad7703d80379e","nickname":"ColeTM","roleType":0,"eloRate":1392,"eloRank":209,"country":null,"seasonResult":{"eloRate":1392,"eloRank":209,"phasePoint":5}}]}"#;
	let leaderboard: PhaseLeaderboardInfo = serde_json::from_str(JSON).unwrap();

	assert_eq!(
		leaderboard.phase,
		PhaseInfo {
			number: Some(2),
			ends_at: Some(Utc.timestamp_opt(1709769600, 0).unwrap()),
			season: 4
		}
	);
	assert_eq!(
		leaderboard.users[0],
		LeaderboardUser {
			profile: UserProfile::new(
				"7665f76f431b41c6b321bea16aff913b",
				"lowk3y_",
				SupporterTier::None,
				Some(1966),
				Some(4),
				None,
			),
			season_result: LeaderboardSeasonOutcome {
				elo: 1966,
				rank: 4,
				phase_point: 50
			}
		}
	)
}
