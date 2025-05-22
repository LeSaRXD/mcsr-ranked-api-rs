use uuid::Uuid;

use crate::user::{
	identifier::UserIdentifier,
	info::{all_seasons::AllSeasonUserInfo, PhaseInfo, UserSeasonOutcome},
	SupporterTier, UserProfile,
};

#[test]
fn user_profile() {
	const DOOGILE: &str = r#"{"uuid":"3c8757790ab0400b8b9e3936e0dd535b","nickname":"doogile","roleType":3,"eloRate":1804,"eloRank":33,"country":"us"}"#;
	const LAYSAR: &str = r#"{"uuid":"79635c3dbf634a228bf44544cc7c0d27","nickname":"LaysarOwO","roleType":0,"eloRate":1226,"eloRank":333,"country":null}"#;

	let doogile: UserProfile = serde_json::from_str(DOOGILE).unwrap();
	let laysar: UserProfile = serde_json::from_str(LAYSAR).unwrap();

	assert_eq!(
		doogile,
		UserProfile::new(
			"3c8757790ab0400b8b9e3936e0dd535b",
			"doogile",
			SupporterTier::Diamond,
			Some(1804),
			Some(33),
			Some("us"),
		),
	);
	assert_eq!(
		laysar,
		UserProfile::new(
			"79635c3dbf634a228bf44544cc7c0d27",
			"LaysarOwO",
			SupporterTier::None,
			Some(1226),
			Some(333),
			None,
		)
	);
}

#[test]
fn all_seasons_info() {
	const JSON: &str = r#"{"uuid":"9a8e24df4c8549d696a6951da84fa5c4","nickname":"Feinberg","roleType":3,"eloRate":2047,"eloRank":5,"country":"us","seasonResults":{"6":{"last":{"eloRate":2276,"eloRank":8,"phasePoint":125},"highest":2291,"lowest":2276,"phases":[{"phase":2,"eloRate":2089,"eloRank":6,"point":30},{"phase":3,"eloRate":2248,"eloRank":4,"point":50},{"phase":4,"eloRate":2276,"eloRank":8,"point":45}]},"7":{"last":{"eloRate":2047,"eloRank":5,"phasePoint":20},"highest":2110,"lowest":1539,"phases":[{"phase":1,"eloRate":1871,"eloRank":9,"point":20}]}}}"#;
	let info: AllSeasonUserInfo = serde_json::from_str(JSON).unwrap();

	assert_eq!(
		info.profile,
		UserProfile::new(
			"9a8e24df4c8549d696a6951da84fa5c4",
			"Feinberg",
			SupporterTier::Diamond,
			Some(2047),
			Some(5),
			Some("us"),
		)
	);

	assert_eq!(
		info.season_results(),
		&[
			None,
			None,
			None,
			None,
			None,
			None,
			Some(UserSeasonOutcome::new(
				2276,
				8,
				125,
				2291,
				2276,
				[
					PhaseInfo::new(2, 2089, 6, 30),
					PhaseInfo::new(3, 2248, 4, 50),
					PhaseInfo::new(4, 2276, 8, 45),
				]
			)),
			Some(UserSeasonOutcome::new(
				2047,
				5,
				20,
				2110,
				1539,
				[PhaseInfo::new(1, 1871, 9, 20),]
			))
		]
	)
}

#[test]
fn identifier() {
	let uuid = UserIdentifier::Uuid(Uuid::nil());
	assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");
	assert_eq!(
		serde_json::to_string(&uuid).unwrap(),
		r#""00000000-0000-0000-0000-000000000000""#
	);

	let nickname = UserIdentifier::Nickname("LaysarOwO");
	assert_eq!(nickname.to_string(), "LaysarOwO");
	assert_eq!(serde_json::to_string(&nickname).unwrap(), r#""LaysarOwO""#);

	let nickname = UserIdentifier::DiscordId(519081871766978572);
	assert_eq!(nickname.to_string(), "discord.519081871766978572");
	assert_eq!(
		serde_json::to_string(&nickname).unwrap(),
		r#""discord.519081871766978572""#
	);
}
