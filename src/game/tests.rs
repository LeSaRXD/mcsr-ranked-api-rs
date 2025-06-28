use chrono::{TimeZone, Utc};
use uuid::uuid;

use crate::{
	game::{
		AdvancedMatchInfo, BastionType, MatchCategory, MatchCompletion, MatchEloUpdate,
		MatchOutcome, MatchRank, MatchSeedInfo, MatchTimelineEvent, MatchType, OverworldType,
	},
	types::Time,
	user::{SupporterTier, UserProfile},
};

#[test]
fn match_info() {
	const JSON: &str = r#"{"id":1524115,"type":2,"seed":{"id":null,"overworld":"VILLAGE","bastion":"HOUSING","variations":[]},"category":"ANY","gameMode":"default","players":[{"uuid":"79635c3dbf634a228bf44544cc7c0d27","nickname":"LaysarOwO","roleType":0,"eloRate":1146,"eloRank":1048,"country":null},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","nickname":"Frigbob","roleType":0,"eloRate":1470,"eloRank":211,"country":null}],"spectators":[],"result":{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":620128},"forfeited":false,"decayed":false,"rank":{"season":1000,"allTime":null},"changes":[{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","change":19,"eloRate":1282},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","change":-19,"eloRate":1245}],"completions":[{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":620128}],"timelines":[{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":610048,"type":"projectelo.timeline.dragon_death"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":535511,"type":"end.root"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":535393,"type":"story.enter_the_end"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":494792,"type":"story.follow_ender_eye"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":454844,"type":"story.follow_ender_eye"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":419050,"type":"projectelo.timeline.blind_travel"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":358761,"type":"projectelo.timeline.blind_travel"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":354196,"type":"nether.obtain_blaze_rod"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":331692,"type":"nether.obtain_blaze_rod"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":320335,"type":"nether.find_fortress"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":311082,"type":"adventure.root"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":310966,"type":"adventure.kill_a_mob"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":310597,"type":"nether.find_fortress"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":290740,"type":"husbandry.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":282496,"type":"adventure.kill_a_mob"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":282404,"type":"adventure.root"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":266018,"type":"nether.loot_bastion"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":255043,"type":"story.form_obsidian"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":235241,"type":"nether.obtain_crying_obsidian"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":230341,"type":"nether.obtain_crying_obsidian"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":223692,"type":"story.form_obsidian"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":170186,"type":"nether.distract_piglin"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":162694,"type":"nether.loot_bastion"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":161055,"type":"nether.find_bastion"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":157338,"type":"nether.find_bastion"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":153691,"type":"husbandry.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":132943,"type":"story.enter_the_nether"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":132837,"type":"nether.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":120786,"type":"story.lava_bucket"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":118642,"type":"nether.root"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":118522,"type":"story.enter_the_nether"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":106937,"type":"story.lava_bucket"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":37230,"type":"story.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":33038,"type":"story.mine_stone"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":31439,"type":"story.root"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":29794,"type":"story.obtain_armor"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":29794,"type":"story.iron_tools"},{"uuid":"79635c3dbf634a228bf44544cc7c0d27","time":29684,"type":"story.smelt_iron"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":24938,"type":"story.mine_stone"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":23237,"type":"story.iron_tools"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":22337,"type":"story.obtain_armor"},{"uuid":"7d3a6bb9f62645ae80cf40840ca84c50","time":21037,"type":"story.smelt_iron"}],"season":7,"date":1735326765,"seedType":"VILLAGE","bastionType":"HOUSING","tag":null,"replayExist":false}"#;

	let full_info: AdvancedMatchInfo = serde_json::from_str(JSON).unwrap();
	let info = &full_info.info;
	assert_eq!(info.id, 1524115);
	assert_eq!(info.kind, MatchType::Ranked);
	assert_eq!(info.season, 7);
	assert_eq!(info.category, Some(MatchCategory::Any));
	assert_eq!(info.date, Utc.timestamp_opt(1735326765, 0).unwrap());
	let mut players = info.players().to_vec();
	players.sort_by_key(|l| l.uuid);
	assert_eq!(
		players,
		[
			UserProfile::new(
				"79635c3dbf634a228bf44544cc7c0d27",
				"LaysarOwO",
				SupporterTier::None,
				Some(1146),
				Some(1048),
				None,
			),
			UserProfile::new(
				"7d3a6bb9f62645ae80cf40840ca84c50",
				"Frigbob",
				SupporterTier::None,
				Some(1470),
				Some(211),
				None,
			),
		]
	);
	let frigidbob = uuid!("7d3a6bb9f62645ae80cf40840ca84c50");
	let laysar = uuid!("79635c3dbf634a228bf44544cc7c0d27");
	assert_eq!(info.spectators(), []);
	assert_eq!(
		info.seed,
		Some(MatchSeedInfo {
			id: None,
			overworld: Some(OverworldType::Village),
			bastion: Some(BastionType::Housing),
			variations: [].into(),
			end_towers: None,
		})
	);
	assert_eq!(
		info.result,
		MatchOutcome {
			winner_uuid: Some(frigidbob),
			time: Time(620128),
		}
	);
	assert!(!info.forfeited);
	assert!(!info.decayed);
	assert_eq!(
		info.rank,
		MatchRank {
			season: Some(1000),
			all_time: None,
		}
	);
	assert_eq!(
		info.elo_updates(),
		[
			MatchEloUpdate {
				player_uuid: frigidbob,
				change: Some(19),
				elo: Some(1282),
			},
			MatchEloUpdate {
				player_uuid: laysar,
				change: Some(-19),
				elo: Some(1245),
			}
		],
	);
	assert_eq!(
		full_info.completions(),
		[MatchCompletion {
			player_uuid: frigidbob,
			time: Time(620128),
		}]
	);
	assert_eq!(
		full_info.timeline_events(),
		[
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(610048),
				id: "projectelo.timeline.dragon_death".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(535511),
				id: "end.root".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(535393),
				id: "story.enter_the_end".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(494792),
				id: "story.follow_ender_eye".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(454844),
				id: "story.follow_ender_eye".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(419050),
				id: "projectelo.timeline.blind_travel".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(358761),
				id: "projectelo.timeline.blind_travel".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(354196),
				id: "nether.obtain_blaze_rod".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(331692),
				id: "nether.obtain_blaze_rod".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(320335),
				id: "nether.find_fortress".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(311082),
				id: "adventure.root".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(310966),
				id: "adventure.kill_a_mob".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(310597),
				id: "nether.find_fortress".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(290740),
				id: "husbandry.root".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(282496),
				id: "adventure.kill_a_mob".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(282404),
				id: "adventure.root".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(266018),
				id: "nether.loot_bastion".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(255043),
				id: "story.form_obsidian".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(235241),
				id: "nether.obtain_crying_obsidian".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(230341),
				id: "nether.obtain_crying_obsidian".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(223692),
				id: "story.form_obsidian".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(170186),
				id: "nether.distract_piglin".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(162694),
				id: "nether.loot_bastion".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(161055),
				id: "nether.find_bastion".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(157338),
				id: "nether.find_bastion".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(153691),
				id: "husbandry.root".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(132943),
				id: "story.enter_the_nether".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(132837),
				id: "nether.root".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(120786),
				id: "story.lava_bucket".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(118642),
				id: "nether.root".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(118522),
				id: "story.enter_the_nether".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(106937),
				id: "story.lava_bucket".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(37230),
				id: "story.root".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(33038),
				id: "story.mine_stone".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(31439),
				id: "story.root".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(29794),
				id: "story.obtain_armor".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(29794),
				id: "story.iron_tools".into(),
			},
			MatchTimelineEvent {
				player_uuid: laysar,
				time: Time(29684),
				id: "story.smelt_iron".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(24938),
				id: "story.mine_stone".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(23237),
				id: "story.iron_tools".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(22337),
				id: "story.obtain_armor".into(),
			},
			MatchTimelineEvent {
				player_uuid: frigidbob,
				time: Time(21037),
				id: "story.smelt_iron".into(),
			}
		]
	);
	assert!(!full_info.replay_exists);
	#[cfg(feature = "serialize")]
	{
		let re_deserialized: AdvancedMatchInfo =
			serde_json::from_str(&serde_json::to_string(&full_info).unwrap()).unwrap();
		assert_eq!(full_info, re_deserialized)
	}
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
fn overworld_type() {
	assert_eq!(
		serde_json::from_str::<OverworldType>(r#""VILLAGE""#).unwrap(),
		OverworldType::Village
	);
	assert_eq!(
		serde_json::from_str::<OverworldType>(r#""BURIED_TREASURE""#).unwrap(),
		OverworldType::BuriedTreasure
	);
	assert_eq!(
		serde_json::from_str::<OverworldType>(r#""SHIPWRECK""#).unwrap(),
		OverworldType::Shipwreck
	);
	assert_eq!(
		serde_json::from_str::<OverworldType>(r#""DESERT_TEMPLE""#).unwrap(),
		OverworldType::DesertTemple
	);
	assert_eq!(
		serde_json::from_str::<OverworldType>(r#""RUINED_PORTAL""#).unwrap(),
		OverworldType::RuinedPortal
	);
	assert!(serde_json::from_str::<OverworldType>(r#""DOESN'T EXIST""#).is_err());
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

	#[cfg(feature = "serialize")]
	{
		assert_eq!(
			serde_json::to_string(&BastionType::Housing).unwrap(),
			r#""HOUSING""#
		);
		assert_eq!(
			serde_json::to_string(&BastionType::Treasure).unwrap(),
			r#""TREASURE""#
		);
		assert_eq!(
			serde_json::to_string(&BastionType::Bridge).unwrap(),
			r#""BRIDGE""#
		);
		assert_eq!(
			serde_json::to_string(&BastionType::Stables).unwrap(),
			r#""STABLES""#
		);
	}
}
