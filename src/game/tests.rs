use std::str::FromStr;

use chrono::{TimeZone, Utc};
use uuid::Uuid;

use crate::{
	game::{
		BastionType, MatchCategory, MatchCompletion, MatchEloUpdate, MatchInfo, MatchRank,
		MatchResult, MatchTimelineEvent, MatchType, SeedType,
	},
	user::{role_type::SupporterTier, UserProfile},
	Time,
};

#[test]
fn match_info() {
	const JSON: &str = r#"{"id":1524115,"type":2,"category":"ANY","gameMode":"default","players":[{"uuid":"79635c3dbf634a228bf44544cc7c0d27","nickname":"LaysarOwO","roleType":0,"eloRate":1226,"eloRank":348},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","nickname":"Frigbob","roleType":0,"eloRate":1412,"eloRank":86}],"spectators":[],"result":{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":620128},"forfeited":false,"decayed":false,"rank":{"season":1000,"allTime":null},"changes":[{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","change":19,"eloRate":1282},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","change":-19,"eloRate":1245}],"completions":[{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":620128}],"timelines":[{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":610048,"type":"projectelo.timeline.dragon_death"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":535511,"type":"end.root"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":535393,"type":"story.enter_the_end"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":494792,"type":"story.follow_ender_eye"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":454844,"type":"story.follow_ender_eye"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":419050,"type":"projectelo.timeline.blind_travel"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":358761,"type":"projectelo.timeline.blind_travel"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":354196,"type":"nether.obtain_blaze_rod"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":331692,"type":"nether.obtain_blaze_rod"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":320335,"type":"nether.find_fortress"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":311082,"type":"adventure.root"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":310966,"type":"adventure.kill_a_mob"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":310597,"type":"nether.find_fortress"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":290740,"type":"husbandry.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":282496,"type":"adventure.kill_a_mob"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":282404,"type":"adventure.root"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":266018,"type":"nether.loot_bastion"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":255043,"type":"story.form_obsidian"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":235241,"type":"nether.obtain_crying_obsidian"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":230341,"type":"nether.obtain_crying_obsidian"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":223692,"type":"story.form_obsidian"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":170186,"type":"nether.distract_piglin"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":162694,"type":"nether.loot_bastion"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":161055,"type":"nether.find_bastion"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":157338,"type":"nether.find_bastion"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":153691,"type":"husbandry.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":132943,"type":"story.enter_the_nether"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":132837,"type":"nether.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":120786,"type":"story.lava_bucket"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":118642,"type":"nether.root"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":118522,"type":"story.enter_the_nether"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":106937,"type":"story.lava_bucket"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":37230,"type":"story.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":33038,"type":"story.mine_stone"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":31439,"type":"story.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":29794,"type":"story.obtain_armor"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":29794,"type":"story.iron_tools"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":29684,"type":"story.smelt_iron"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":24938,"type":"story.mine_stone"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":23237,"type":"story.iron_tools"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":22337,"type":"story.obtain_armor"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":21037,"type":"story.smelt_iron"}],"season":7,"date":1735326765,"seedType":"VILLAGE","bastionType":"HOUSING","tag":null,"replayExist":true}"#;

	let info: MatchInfo = serde_json::from_str(JSON).unwrap();
	assert_eq!(info.id, 1524115);
	assert_eq!(info.kind, MatchType::Ranked);
	assert_eq!(info.season, 7);
	assert_eq!(info.category, MatchCategory::Any);
	assert_eq!(info.date, Utc.timestamp_opt(1735326765, 0).unwrap());
	assert_eq!(
		info.players(),
		[
			UserProfile::new(
				Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				"LaysarOwO",
				SupporterTier::None,
				Some(1226),
				Some(348),
			),
			UserProfile::new(
				Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				"Frigbob",
				SupporterTier::None,
				Some(1412),
				Some(86)
			)
		]
	);
	assert_eq!(info.spectators(), []);
	assert_eq!(
		info.result,
		MatchResult {
			uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
			time: Time(620128),
		}
	);
	assert_eq!(info.forfeited, false);
	assert_eq!(info.decayed, false);
	assert_eq!(
		info.rank,
		MatchRank {
			season: Some(1000),
			all_time: None
		}
	);
	assert_eq!(
		info.elo_updates(),
		[
			MatchEloUpdate {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				change: Some(19),
				elo_rate: Some(1282),
			},
			MatchEloUpdate {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				change: Some(-19),
				elo_rate: Some(1245),
			}
		],
	);
	assert_eq!(info.seed_type, SeedType::Village);
	assert_eq!(info.bastion_type, BastionType::Housing);
	assert_eq!(
		info.completions(),
		[MatchCompletion {
			uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
			time: Time(620128),
		}]
	);
	assert_eq!(
		info.timeline_events().unwrap(),
		[
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(610048),
				kind: "projectelo.timeline.dragon_death".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(535511),
				kind: "end.root".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(535393),
				kind: "story.enter_the_end".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(494792),
				kind: "story.follow_ender_eye".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(454844),
				kind: "story.follow_ender_eye".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(419050),
				kind: "projectelo.timeline.blind_travel".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(358761),
				kind: "projectelo.timeline.blind_travel".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(354196),
				kind: "nether.obtain_blaze_rod".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(331692),
				kind: "nether.obtain_blaze_rod".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(320335),
				kind: "nether.find_fortress".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(311082),
				kind: "adventure.root".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(310966),
				kind: "adventure.kill_a_mob".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(310597),
				kind: "nether.find_fortress".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(290740),
				kind: "husbandry.root".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(282496),
				kind: "adventure.kill_a_mob".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(282404),
				kind: "adventure.root".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(266018),
				kind: "nether.loot_bastion".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(255043),
				kind: "story.form_obsidian".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(235241),
				kind: "nether.obtain_crying_obsidian".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(230341),
				kind: "nether.obtain_crying_obsidian".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(223692),
				kind: "story.form_obsidian".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(170186),
				kind: "nether.distract_piglin".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(162694),
				kind: "nether.loot_bastion".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(161055),
				kind: "nether.find_bastion".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(157338),
				kind: "nether.find_bastion".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(153691),
				kind: "husbandry.root".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(132943),
				kind: "story.enter_the_nether".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(132837),
				kind: "nether.root".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(120786),
				kind: "story.lava_bucket".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(118642),
				kind: "nether.root".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(118522),
				kind: "story.enter_the_nether".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(106937),
				kind: "story.lava_bucket".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(37230),
				kind: "story.root".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(33038),
				kind: "story.mine_stone".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(31439),
				kind: "story.root".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(29794),
				kind: "story.obtain_armor".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(29794),
				kind: "story.iron_tools".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap(),
				time: Time(29684),
				kind: "story.smelt_iron".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(24938),
				kind: "story.mine_stone".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(23237),
				kind: "story.iron_tools".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(22337),
				kind: "story.obtain_armor".into(),
			},
			MatchTimelineEvent {
				uuid: Uuid::from_str("7d3a6bb9f62645ae80cf40840ca84c50").unwrap(),
				time: Time(21037),
				kind: "story.smelt_iron".into(),
			}
		]
	);
	assert_eq!(info.replay_exist, true);
}

#[test]
fn match_category() {
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ANY""#).unwrap(),
		MatchCategory::Any
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""CUSTOM""#).unwrap(),
		MatchCategory::Custom
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""HIGH""#).unwrap(),
		MatchCategory::High
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""KILL_ALL_BOSSES""#).unwrap(),
		MatchCategory::KillAllBosses
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""KILL_WITHER""#).unwrap(),
		MatchCategory::KillWither
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""KILL_ELDER_GUARDIAN""#).unwrap(),
		MatchCategory::KillElderGuardian
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ALL_ADVANCEMENTS""#).unwrap(),
		MatchCategory::AllAdvancements
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""HALF""#).unwrap(),
		MatchCategory::Half
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""POGLOOT_QUATER""#).unwrap(),
		MatchCategory::PoglootQuater
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""HOW_DID_WE_GET_HERE""#).unwrap(),
		MatchCategory::HowDidWeGetHere
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""HERO_OF_THE_VILLAGE""#).unwrap(),
		MatchCategory::HeroOfTheVillage
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ARBALISTIC""#).unwrap(),
		MatchCategory::Arbalistic
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""COVER_ME_IN_DEBRIS""#).unwrap(),
		MatchCategory::CoverMeInDebris
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ENTER_NETHER""#).unwrap(),
		MatchCategory::EnterNether
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ENTER_END""#).unwrap(),
		MatchCategory::EnterEnd
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ALL_SWORDS""#).unwrap(),
		MatchCategory::AllSwords
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ALL_MINERALS""#).unwrap(),
		MatchCategory::AllMinerals
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""FULL_IA_15_LVL""#).unwrap(),
		MatchCategory::FullIa15Lvl
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ALL_WORKSTATIONS""#).unwrap(),
		MatchCategory::AllWorkstations
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""FULL_INV""#).unwrap(),
		MatchCategory::FullInv
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""STACK_OF_LIME_WOOL""#).unwrap(),
		MatchCategory::StackOfLimeWool
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ALL_PORTALS""#).unwrap(),
		MatchCategory::AllPortals
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""ALL_BLOCKS""#).unwrap(),
		MatchCategory::AllBlocks
	);
	assert_eq!(
		serde_json::from_str::<MatchCategory>(r#""MINE_A_CHUNK""#).unwrap(),
		MatchCategory::MineAChunk
	);
}

#[test]
fn match_type() {
	assert_eq!(
		serde_json::from_str::<MatchType>("1").unwrap(),
		MatchType::Causal
	);
	assert_eq!(
		serde_json::from_str::<MatchType>("2").unwrap(),
		MatchType::Ranked
	);
	assert_eq!(
		serde_json::from_str::<MatchType>("3").unwrap(),
		MatchType::Private
	);
	assert_eq!(
		serde_json::from_str::<MatchType>("4").unwrap(),
		MatchType::Event
	);
	assert!(serde_json::from_str::<MatchType>("0").is_err());
	assert!(serde_json::from_str::<MatchType>("99").is_err());
}

#[test]
fn seed_type() {
	assert_eq!(
		serde_json::from_str::<SeedType>(r#""VILLAGE""#).unwrap(),
		SeedType::Village
	);
	assert_eq!(
		serde_json::from_str::<SeedType>(r#""BURRIED_TREASURE""#).unwrap(),
		SeedType::BurriedTreasure
	);
	assert_eq!(
		serde_json::from_str::<SeedType>(r#""SHIPWRECK""#).unwrap(),
		SeedType::Shipwreck
	);
	assert_eq!(
		serde_json::from_str::<SeedType>(r#""DESERT_TEMPLE""#).unwrap(),
		SeedType::DesertTemple
	);
	assert_eq!(
		serde_json::from_str::<SeedType>(r#""RUINED_PORTAL""#).unwrap(),
		SeedType::RuinedPortal
	);
	assert!(serde_json::from_str::<SeedType>(r#""DOESN'T EXIST""#).is_err());
}

#[test]
fn bastion_type() {
	assert_eq!(
		serde_json::from_str::<BastionType>(r#""HOUSING""#).unwrap(),
		BastionType::Housing
	);
	assert_eq!(
		serde_json::from_str::<BastionType>(r#""TREASURE""#).unwrap(),
		BastionType::Treasure
	);
	assert_eq!(
		serde_json::from_str::<BastionType>(r#""BRIDGE""#).unwrap(),
		BastionType::Bridge
	);
	assert_eq!(
		serde_json::from_str::<BastionType>(r#""STABLES""#).unwrap(),
		BastionType::Stables
	);
	assert!(serde_json::from_str::<BastionType>(r#""DOESN'T EXIST""#).is_err());
}
