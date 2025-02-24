use std::str::FromStr;

use uuid::Uuid;

use crate::user::{identifier::UserIdentifier, SupporterTier, UserProfile};

#[test]
fn user_profile() {
	const DOOGILE: &str = r#"{"uuid":"3c8757790ab0400b8b9e3936e0dd535b","nickname":"doogile","roleType":3,"eloRate":1804,"eloRank":33,"country":"us"}"#;
	const LAYSAR: &str = r#"{"uuid":"79635c3dbf634a228bf44544cc7c0d27","nickname":"LaysarOwO","roleType":0,"eloRate":1226,"eloRank":333,"country":null}"#;

	let doogile: UserProfile = serde_json::from_str(DOOGILE).unwrap();
	let laysar: UserProfile = serde_json::from_str(LAYSAR).unwrap();

	assert_eq!(
		doogile,
		UserProfile::new(
			Uuid::from_str("3c8757790ab0400b8b9e3936e0dd535b").unwrap(),
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
			Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
			"LaysarOwO",
			SupporterTier::None,
			Some(1226),
			Some(333),
			None,
		)
	);
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
